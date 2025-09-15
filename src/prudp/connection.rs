// use bytes::Bytes;
// use tokio::net::UdpSocket;
// use tokio::sync::{mpsc};
// use tokio::time::{interval, timeout};
// use std::net::SocketAddr;
// use std::sync::Arc;
// use std::collections::{HashMap, VecDeque};
// use std::time::Duration;
// use parking_lot::Mutex as PLMutex;

// use crate::Result;
// use super::packet::{PRUDPPacket, PRUDPHeader, Flags, Version, PacketType};
// use super::sliding::SlidingWindow;
// use crate::prudp::fragmentation::Reassembler;
// use tracing::{debug};

// type AsyncMutex<T> = tokio::sync::Mutex<T>;
// type SyncMutex<T>  = parking_lot::Mutex<T>;

// // We'll forward payloads into an HPP handler channel.
// pub type HppSender = mpsc::Sender<Bytes>;

// pub struct Connection {
//     pub reassembler: Arc<AsyncMutex<Reassembler>>,
//     pub pending:     Arc<AsyncMutex<HashMap<u16, (Bytes, u8, std::time::Instant)>>>,
//     pub window:      Arc<SyncMutex<SlidingWindow>>,
//     pub send_queue:  Arc<AsyncMutex<VecDeque<Bytes>>>,
//     pub dispatch_queue: Arc<AsyncMutex<crate::prudp::packet_dispatch_queue::PacketDispatchQueue>>,
//     pub socket:      Arc<UdpSocket>,
//     pub peer:        SocketAddr,

//     tx_notify: mpsc::Sender<()>,
//     hpp_tx:    HppSender,
// }

// impl Connection {
//     pub async fn new(socket: Arc<UdpSocket>, peer: SocketAddr, hpp_tx: HppSender) -> Self {
//         let (tx, rx) = mpsc::channel::<()>(8);

//         let rtt = Arc::new(crate::rtt::RTT::new());
//         let mut window0 = SlidingWindow::new(64);
//         window0.attach_rtt(rtt.clone());

//         let conn = Connection {
//             peer,
//             socket: socket.clone(),
//             dispatch_queue: Arc::new(AsyncMutex::new(
//                 crate::prudp::packet_dispatch_queue::PacketDispatchQueue::new(0u16)
//             )),
//             window: Arc::new(SyncMutex::new(window0)),
//             pending: Arc::new(AsyncMutex::new(HashMap::new())),
//             send_queue: Arc::new(AsyncMutex::new(VecDeque::new())),
//             tx_notify: tx.clone(),
//             hpp_tx,
//             reassembler: Arc::new(AsyncMutex::new(Reassembler::new(Duration::from_secs(30)))),
//         };

//         // ----- task 1: retransmit loop -----
//         {
//             let socket_clone  = conn.socket.clone();
//             let peer_clone    = conn.peer;
//             let pending_clone = conn.pending.clone();
//             let window_ref    = conn.window.clone();

//             tokio::spawn(async move {
//                 loop {
//                     tokio::time::sleep(Duration::from_millis(200)).await;

//                     let mut to_retx: Vec<u16> = Vec::new();
//                     {
//                         // read timeout from window
//                         let timeout_dur = window_ref.lock().current_retry_timeout();
//                         let now = std::time::Instant::now();
//                         let pend = pending_clone.lock().await;
//                         for (seq, (_data, _attempts, sent_at)) in pend.iter() {
//                             if now.duration_since(*sent_at) > timeout_dur {
//                                 to_retx.push(*seq);
//                             }
//                         }
//                     }

//                     if to_retx.is_empty() { continue; }

//                     let mut pend = pending_clone.lock().await;
//                     for seq in to_retx {
//                         if let Some((data, attempts, sent_at)) = pend.get_mut(&seq) {
//                             if *attempts >= 5 {
//                                 tracing::warn!(seq, "max retransmit attempts reached");
//                                 // drop; in a real impl, maybe notify upper layer
//                                 continue;
//                             }
//                             debug!("retransmitting seq {}", seq);
//                             let _ = socket_clone.send_to(&data[..], peer_clone).await;
//                             *attempts += 1;
//                             *sent_at   = std::time::Instant::now();
//                             window_ref.lock().mark_sent(seq);
//                         }
//                     }
//                 }
//             });
//         }

//         // ----- task 2: sender loop (drains send_queue respecting window) -----
//         {
//             let mut notify_rx    = rx;
//             let send_queue_ref   = conn.send_queue.clone();
//             let socket_tx        = conn.socket.clone();
//             let window_ref       = conn.window.clone();
//             let peer_clone       = conn.peer;
//             let pending_ref      = conn.pending.clone();

//             tokio::spawn(async move {
//                 while notify_rx.recv().await.is_some() {
//                     loop {
//                         if !window_ref.lock().can_send() { break; }

//                         let payload = {
//                             let mut q = send_queue_ref.lock().await;
//                             match q.pop_front() {
//                                 Some(p) => p,
//                                 None => break,
//                             }
//                         };

//                         // Build a data packet
//                         let mut w = window_ref.lock();
//                         let seq = w.next_sequence();

//                         // Use only real fields that exist on PRUDPHeader in *your* codebase.
//                         let header = PRUDPHeader {
//                             version:     Version::V1,
//                             packet_type: PacketType::Data,
//                             flags:       Flags::RELIABLE,
//                             session_id:  0,
//                             sequence_id: seq,
//                             ack_id:      0,
//                             // include this only if your struct has it:
//                             // payload_len: Some(payload.len() as u16),
//                         };

//                         let pkt = PRUDPPacket { header, payload: payload.clone(), fragment_count: 1, fragment_id: 0, fragment_index: 0 };
//                         let bytes = pkt.to_bytes().expect("serialize PRUDP");

//                         if let Err(e) = socket_tx.send_to(&bytes[..], peer_clone).await {
//                             tracing::error!("send error: {:?}", e);
//                             // push back into queue and try later
//                             let mut q = send_queue_ref.lock().await;
//                             q.push_front(payload);
//                             break;
//                         }

//                         // record pending & window bookkeeping
//                         {
//                             let mut pend = pending_ref.lock().await;
//                             pend.insert(seq, (Bytes::from(bytes.clone()), 1u8, std::time::Instant::now()));
//                         }
//                         w.queue_sent(seq, Bytes::from(bytes));
//                     }
//                 }
//             });
//         }

//         conn
//     }

//     pub async fn queue_send(&self, payload: Bytes) -> Result<()> {
//         self.send_queue.lock().await.push_back(payload);
//         let _ = self.tx_notify.send(()).await;
//         Ok(())
//     }
// }
// use bytes::Bytes;
// use tokio::net::UdpSocket;
// use tokio::sync::mpsc;
// use std::net::SocketAddr;
// use std::sync::Arc;
// use std::collections::{HashMap, VecDeque};
// use std::time::Duration;

// use crate::Result;
// use super::packet::{PRUDPPacket, PRUDPHeader, Flags, Version, PacketType};
// use super::sliding::SlidingWindow;
// use crate::prudp::fragmentation::Reassembler;
// use tracing::debug;

// type AsyncMutex<T> = tokio::sync::Mutex<T>;
// type SyncMutex<T>  = parking_lot::Mutex<T>;

// // Forward payloads into an HPP handler channel.
// pub type HppSender = mpsc::Sender<Bytes>;

// pub struct Connection {
//     pub reassembler: Arc<AsyncMutex<Reassembler>>,
//     pub pending:     Arc<AsyncMutex<HashMap<u16, (Bytes, u8, std::time::Instant)>>>,
//     pub window:      Arc<SyncMutex<SlidingWindow>>,
//     pub send_queue:  Arc<AsyncMutex<VecDeque<Bytes>>>,
//     pub dispatch_queue: Arc<AsyncMutex<crate::prudp::packet_dispatch_queue::PacketDispatchQueue>>,
//     pub socket:      Arc<UdpSocket>,
//     pub peer:        SocketAddr,

//     tx_notify: mpsc::Sender<()>,
//     hpp_tx:    HppSender,
// }

// impl Connection {
//     pub async fn new(socket: Arc<UdpSocket>, peer: SocketAddr, hpp_tx: HppSender) -> Self {
//         let (tx, rx) = mpsc::channel::<()>(8);

//         let rtt = Arc::new(crate::rtt::RTT::new());
//         let mut window0 = SlidingWindow::new(64);
//         window0.attach_rtt(rtt.clone());

//         let conn = Connection {
//             peer,
//             socket: socket.clone(),
//             dispatch_queue: Arc::new(AsyncMutex::new(
//                 crate::prudp::packet_dispatch_queue::PacketDispatchQueue::new(0u16)
//             )),
//             window: Arc::new(SyncMutex::new(window0)),
//             pending: Arc::new(AsyncMutex::new(HashMap::new())),
//             send_queue: Arc::new(AsyncMutex::new(VecDeque::new())),
//             tx_notify: tx.clone(),
//             hpp_tx,
//             reassembler: Arc::new(AsyncMutex::new(Reassembler::new(Duration::from_secs(30)))),
//         };

//         // ----- task 1: retransmit loop -----
//         {
//             let socket_clone  = conn.socket.clone();
//             let peer_clone    = conn.peer;
//             let pending_clone = conn.pending.clone();
//             let window_ref    = conn.window.clone();

//             tokio::spawn(async move {
//                 loop {
//                     tokio::time::sleep(Duration::from_millis(200)).await;

//                     let mut to_retx: Vec<u16> = Vec::new();
//                     {
//                         let timeout_dur = window_ref.lock().current_retry_timeout();
//                         let now = std::time::Instant::now();
//                         let pend = pending_clone.lock().await;
//                         for (seq, (_data, _attempts, sent_at)) in pend.iter() {
//                             if now.duration_since(*sent_at) > timeout_dur {
//                                 to_retx.push(*seq);
//                             }
//                         }
//                     }

//                     if to_retx.is_empty() { continue; }

//                     let mut pend = pending_clone.lock().await;
//                     for seq in to_retx {
//                         if let Some((data, attempts, sent_at)) = pend.get_mut(&seq) {
//                             if *attempts >= 5 {
//                                 tracing::warn!(seq, "max retransmit attempts reached");
//                                 continue;
//                             }
//                             debug!("retransmitting seq {}", seq);
//                             let _ = socket_clone.send_to(&data[..], peer_clone).await;
//                             *attempts += 1;
//                             *sent_at   = std::time::Instant::now();
//                             window_ref.lock().mark_sent(seq);
//                         }
//                     }
//                 }
//             });
//         }

//         // ----- task 2: sender loop (drains send_queue respecting window) -----
//         {
//             let mut notify_rx    = rx;
//             let send_queue_ref   = conn.send_queue.clone();
//             let socket_tx        = conn.socket.clone();
//             let window_ref       = conn.window.clone();
//             let peer_clone       = conn.peer;
//             let pending_ref      = conn.pending.clone();

//             tokio::spawn(async move {
//                 while notify_rx.recv().await.is_some() {
//                     loop {
//                         if !window_ref.lock().can_send() { break; }

//                         let payload = {
//                             let mut q = send_queue_ref.lock().await;
//                             match q.pop_front() {
//                                 Some(p) => p,
//                                 None => break,
//                             }
//                         };

//                         // Build a data packet
//                         let mut w = window_ref.lock();
//                         let seq = w.next_sequence();

//                         // NOTE: your header uses primitive field types.
//                         let header = PRUDPHeader {
//                             version:     Version::V1 as u8,           // u8
//                             packet_type: PacketType::Data as u16,     // u16
//                             flags:       Flags::RELIABLE,
//                             session_id:  0,
//                             sequence_id: seq,
//                             ack_id:      0,
//                             payload_len: Some(payload.len()), 
//                         };

//                         // Your PRUDPPacket requires Option<...> fragment fields.
//                         let pkt = PRUDPPacket {
//                             header,
//                             payload: payload.clone(),
//                             fragment_count: Some(1),
//                             fragment_id:    Some(0),
//                             fragment_index: Some(0),
//                         };

//                         let bytes = pkt.to_bytes().expect("serialize PRUDP");

//                         if let Err(e) = socket_tx.send_to(&bytes[..], peer_clone).await {
//                             tracing::error!("send error: {:?}", e);
//                             let mut q = send_queue_ref.lock().await;
//                             q.push_front(payload);
//                             break;
//                         }

//                         {
//                             let mut pend = pending_ref.lock().await;
//                             pend.insert(seq, (Bytes::from(bytes.clone()), 1u8, std::time::Instant::now()));
//                         }
//                         w.queue_sent(seq, Bytes::from(bytes));
//                     }
//                 }
//             });
//         }

//         conn
//     }

//     /// Minimal handling so the server call compiles; flesh this out as needed.
//     pub async fn handle_incoming_packet(&self, pkt: PRUDPPacket) -> Result<()> {
//         if pkt.header.flags.contains(Flags::ACK) {
//             let seq = pkt.header.sequence_id;
//             let mut pend = self.pending.lock().await;
//             if pend.remove(&seq).is_some() {
//                 self.window.lock().mark_acked(seq);
//             }
//             return Ok(());
//         }

//         // For now, forward payload upward to HPP handler; you can integrate
//         // dispatch_queue/reassembly here later.
//         let _ = self.hpp_tx.send(pkt.payload.clone()).await;
//         Ok(())
//     }

//     pub async fn queue_send(&self, payload: Bytes) -> Result<()> {
//         self.send_queue.lock().await.push_back(payload);
//         let _ = self.tx_notify.send(()).await;
//         Ok(())
//     }
// }


use bytes::Bytes;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use std::collections::{HashMap, VecDeque};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{debug, warn, error};

use crate::Result;
use super::packet::{PRUDPPacket, PRUDPHeader, Flags, Version, PacketType};
use super::sliding::SlidingWindow;
use crate::prudp::fragmentation::Reassembler;

type AsyncMutex<T> = tokio::sync::Mutex<T>;
type SyncMutex<T>  = parking_lot::Mutex<T>;

// Forward payloads into an HPP handler channel.
pub type HppSender = mpsc::Sender<Bytes>;

pub struct Connection {
    pub reassembler: Arc<AsyncMutex<Reassembler>>,
    pub pending:     Arc<AsyncMutex<HashMap<u16, (Bytes, u8, Instant)>>>,
    pub window:      Arc<SyncMutex<SlidingWindow>>,
    pub send_queue:  Arc<AsyncMutex<VecDeque<Bytes>>>,
    pub dispatch_queue: Arc<AsyncMutex<crate::prudp::packet_dispatch_queue::PacketDispatchQueue>>,
    pub socket:      Arc<UdpSocket>,
    pub peer:        SocketAddr,

    tx_notify: mpsc::Sender<()>,
    hpp_tx:    HppSender,
}

impl Connection {
    pub async fn new(socket: Arc<UdpSocket>, peer: SocketAddr, hpp_tx: HppSender) -> Self {
        let (tx, rx) = mpsc::channel::<()>(8);

        let rtt = Arc::new(crate::rtt::RTT::new());
        let mut win = SlidingWindow::new(64);
        win.attach_rtt(rtt.clone());

        let conn = Connection {
            peer,
            socket: socket.clone(),
            dispatch_queue: Arc::new(AsyncMutex::new(
                crate::prudp::packet_dispatch_queue::PacketDispatchQueue::new(0u16)
            )),
            window:  Arc::new(SyncMutex::new(win)),
            pending: Arc::new(AsyncMutex::new(HashMap::new())),
            send_queue: Arc::new(AsyncMutex::new(VecDeque::new())),
            tx_notify: tx.clone(),
            hpp_tx,
            reassembler: Arc::new(AsyncMutex::new(Reassembler::new(Duration::from_secs(30)))),
        };

        // ----- task 1: retransmit loop -----
        {
            let socket_ref  = conn.socket.clone();
            let peer_ref    = conn.peer;
            let pending_ref = conn.pending.clone();
            let window_ref  = conn.window.clone();

            tokio::spawn(async move {
                loop {
                    tokio::time::sleep(Duration::from_millis(200)).await;

                    // Decide what to retransmit (no awaits while holding window lock)
                    let timeout_dur = window_ref.lock().current_retry_timeout();
                    let now = Instant::now();

                    let to_retx: Vec<u16> = {
                        let pend = pending_ref.lock().await;
                        pend.iter()
                            .filter_map(|(seq, (_data, _attempts, sent_at))|
                                if now.duration_since(*sent_at) > timeout_dur { Some(*seq) } else { None }
                            )
                            .collect()
                    };

                    if to_retx.is_empty() { continue; }

                    for seq in to_retx {
                        // Copy data to send without holding the lock across await
                        let data_to_send = {
                            let pend = pending_ref.lock().await;
                            pend.get(&seq).map(|(data, _attempts, _)| data.clone())
                        };

                        if let Some(bytes) = data_to_send {
                            // Send with no locks held
                            let _ = socket_ref.send_to(&bytes[..], peer_ref).await;

                            // Update attempts/timestamp
                            {
                                let mut pend = pending_ref.lock().await;
                                if let Some((_d, attempts, sent_at)) = pend.get_mut(&seq) {
                                    *attempts += 1;
                                    *sent_at = Instant::now();
                                }
                            }

                            // Window bookkeeping
                            window_ref.lock().mark_sent(seq);
                            debug!("retransmitted seq {}", seq);
                        }
                    }
                }
            });
        }

        // ----- task 2: sender loop (drains send_queue respecting window) -----
        {
            let mut notify_rx  = rx;
            let send_queue_ref = conn.send_queue.clone();
            let socket_ref     = conn.socket.clone();
            let window_ref     = conn.window.clone();
            let peer_ref       = conn.peer;
            let pending_ref    = conn.pending.clone();

            tokio::spawn(async move {
                while notify_rx.recv().await.is_some() {
                    loop {
                        // Window check (no await while locked)
                        if !window_ref.lock().can_send() { break; }

                        // Pop a payload to send
                        let payload = {
                            let mut q = send_queue_ref.lock().await;
                            match q.pop_front() {
                                Some(p) => p,
                                None => break,
                            }
                        };

                        // Get sequence number (drop guard before any await)
                        let seq = { window_ref.lock().next_sequence() };

                        // Build header & packet without locks
                        let header = PRUDPHeader {
                            version:     Version::V1 as u8,        // adjust if your Version is prim
                            packet_type: PacketType::Data as u16,  // adjust if prim
                            flags:       Flags::RELIABLE,
                            session_id:  0,
                            sequence_id: seq,
                            ack_id:      0,
                            payload_len: Some(payload.len()),
                        };

                        let pkt = PRUDPPacket {
                            header,
                            payload: payload.clone(),
                            fragment_count: Some(1),
                            fragment_id:    Some(0),
                            fragment_index: Some(0),
                        };

                        let bytes = match pkt.to_bytes() {
                            Ok(b) => b,
                            Err(e) => {
                                error!("serialize PRUDP failed: {:?}", e);
                                // push back and break this tick
                                let mut q = send_queue_ref.lock().await;
                                q.push_front(payload);
                                break;
                            }
                        };

                        // Send (no locks held)
                        if let Err(e) = socket_ref.send_to(&bytes[..], peer_ref).await {
                            error!("send error: {:?}", e);
                            let mut q = send_queue_ref.lock().await;
                            q.push_front(payload);
                            break;
                        }

                        // Record pending & update window (lock only for the calls you need)
                        {
                            let mut pend = pending_ref.lock().await;
                            pend.insert(seq, (Bytes::from(bytes.clone()), 1u8, Instant::now()));
                        }
                        {
                            let mut w = window_ref.lock();
                            w.queue_sent(seq, Bytes::from(bytes));
                        }
                    }
                }
            });
        }

        conn
    }

    /// Minimal handling: ACK clears pending; otherwise forward payload upstream.
    pub async fn handle_incoming_packet(&self, pkt: PRUDPPacket) -> Result<()> {
        if pkt.header.flags.contains(Flags::ACK) {
            let seq = pkt.header.sequence_id;
            let mut pend = self.pending.lock().await;
            if pend.remove(&seq).is_some() {
                self.window.lock().mark_acked(seq);
            }
            return Ok(());
        }

        // Forward to HPP layer; integrate reassembly/dispatch as needed.
        let _ = self.hpp_tx.send(pkt.payload.clone()).await;
        Ok(())
    }

    pub async fn queue_send(&self, payload: Bytes) -> Result<()> {
        self.send_queue.lock().await.push_back(payload);
        let _ = self.tx_notify.send(()).await;
        Ok(())
    }
}
