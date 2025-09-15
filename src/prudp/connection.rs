
use tokio::net::UdpSocket;
use tokio::sync::{mpsc, Mutex};
use std::sync::Arc;
use std::net::SocketAddr;
use bytes::Bytes;
use std::collections::{HashMap, VecDeque};
use std::time::Duration;
use crate::Result;
use super::packet::{PRUDPPacket, PRUDPHeader, Flags, Version, PacketType};
use super::sliding::SlidingWindow;
use crate::prudp::fragmentation::Reassembler;
use tracing::debug;
use parking_lot::Mutex as PLMutex;
use async_trait::async_trait;

// We'll forward payloads into an HPP handler channel.
pub type HppSender = mpsc::Sender<Bytes>;

// Represents a connection-level handler managing retransmit and sliding window.
pub struct Connection {
    pub reassembler: std::sync::Arc<tokio::sync::Mutex<crate::prudp::fragmentation::Reassembler>>,

    // Dispatch queue to deliver in-order data packets to upper layer
    pub dispatch_queue: std::sync::Arc<tokio::sync::Mutex<crate::prudp::packet_dispatch_queue::PacketDispatchQueue>>,

    pub peer: SocketAddr,
    socket: Arc<UdpSocket>,
    window: PLMutex<SlidingWindow>,
    // pending frames waiting for ack: seq -> (data, attempts)
    pending: Mutex<HashMap<u16, (Bytes, u8, std::time::Instant)>>,
    // queue of outgoing payloads waiting for admission
    send_queue: Mutex<VecDeque<Bytes>>,
    tx_notify: mpsc::Sender<()>,
    // hpp sender to forward delivered payloads
    hpp_tx: HppSender,
}

impl Connection {
    pub async fn new(socket: Arc<UdpSocket>, peer: SocketAddr, hpp_tx: HppSender) -> Self {
        let (tx, mut rx) = mpsc::channel::<()>(8);
        let rtt = std::sync::Arc::new(crate::rtt::RTT::new());
        let mut window = SlidingWindow::new(64);
        window.attach_rtt(rtt.clone());
        let dispatch_queue = std::sync::Arc::new(tokio::sync::Mutex::new(crate::prudp::packet_dispatch_queue::PacketDispatchQueue::new()));
        let conn = Connection {
            peer,
            socket,
            window: PLMutex::new(window),
            pending: Mutex::new(HashMap::new()),
            send_queue: Mutex::new(VecDeque::new()),
            tx_notify: tx.clone(),
            hpp_tx,
            reassembler: std::sync::Arc::new(tokio::sync::Mutex::new(Reassembler::new(std::time::Duration::from_secs(30)))),

        };

        // Spawn retransmit task
        let socket_clone = conn.socket.clone();
        let peer_clone = conn.peer;
        let pending_clone = conn.pending.clone();
        let window_clone = conn.window.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_millis(200)).await;
                let mut to_retransmit: Vec<u16> = Vec::new();
                {
                    let w = window_clone.lock();
                    // we'll consult per-packet timestamps below
                }
                let mut pending = pending_clone.lock().await;
                let now = std::time::Instant::now();
                for (seq, (_data, attempts, sent_at)) in pending.iter() {
                    let timeout = {
                        let w = window_clone.lock();
                        w.current_retry_timeout()
                    };
                    if now.duration_since(*sent_at) > timeout {
                        to_retransmit.push(*seq);
                    }
                }
                if to_retransmit.is_empty() { continue; }
                for seq in to_retransmit {
                    if let Some((data, attempts, sent_at)) = pending.get_mut(&seq) {
                        if *attempts >= 5 {
                            tracing::warn!(seq = seq, "max retransmit attempts reached");
                            pending.remove(&seq);
                            continue;
                        }
                        debug!("retransmitting seq {}", seq);
                        let _ = socket_clone.send_to(&data[..], peer_clone).await;
                        *attempts += 1;
                        *sent_at = std::time::Instant::now();
                        // update timestamp in sliding window
                        let mut w = window_clone.lock();
                        w.mark_sent(seq);
                    }
                }
            }
        });

        // Spawn sender task to drain send_queue respecting window
        let send_queue_ref = conn.send_queue.clone();
        let dispatch_queue_clone = conn.dispatch_queue.clone();
        let reassembler_clone = conn.reassembler.clone();
        tokio::spawn(async move {
            while let Some(_) = notify_rx.recv().await {
                loop {
                    let can = { window_tx.lock().can_send() };
                    if !can { break; }
                    // pop a payload
                    let mut q = send_queue_ref.lock().await;
                    let payload = match q.pop_front() {
                        Some(p) => p,
                        None => break,
                    };
                    drop(q);
                    // fragment payload based on MTU (assume 1200 for now)
                    let mtu = 1200usize;
                    let base_header = PRUDPHeader {
                        version: 1u8,
                        packet_type: PacketType::Data as u16,
                        flags: Flags::RELIABLE,
                        session_id: 0,
                        sequence_id: 0,
                        ack_id: 0,
                        payload_len: None,
                    };
                    let frags = crate::prudp::fragmentation::fragment_into_packets(&payload, mtu, &base_header).unwrap_or_default();
                    for mut pkt in frags {
                        // allocate seq
                        let mut w = window_tx.lock();
                        let seq = w.next_sequence();
                        pkt.header.sequence_id = seq;
                        let bytes = pkt.to_bytes().unwrap();
                        // send
                        if let Err(e) = socket_tx.send_to(&bytes[..], peer_clone).await {
                            tracing::error!("send error: {:?}", e);
                            // push back remaining fragments
                            let mut q = send_queue_ref.lock().await;
                            q.push_front(payload);
                            break;
                        }
                        // record pending and window
                        {
                            let mut pend = pending_tx.lock().await;
                            pend.insert(seq, (bytes::Bytes::from(bytes.clone()), 1u8, std::time::Instant::now()));
                        }
                        {
                            let mut w = window_tx.lock();
                            w.queue_sent(seq, bytes::Bytes::from(bytes));
                        }
                    }
                }
            }
        });

        let socket_tx = conn.socket.clone();
        let window_tx = conn.window.clone();
        let pending_tx = conn.pending.clone();
        let mut notify_rx = rx;
        let send_queue_ref = conn.send_queue.clone();
        tokio::spawn(async move {
            while let Some(_) = notify_rx.recv().await {
                loop {
                    let can = { window_tx.lock().can_send() };
                    if !can { break; }
                    // pop a payload
                    let mut q = send_queue_ref.lock().await;
                    let payload = match q.pop_front() {
                        Some(p) => p,
                        None => break,
                    };
                    drop(q);
                    // create packet with next sequence
                    let mut w = window_tx.lock();
                    let seq = w.next_sequence();
                    let header = PRUDPHeader {
                        version: Version::V0,
                        src: 0,
                        dst: 0,
                        flags: Flags::empty(),
                        session_id: 0,
                        seq,
                        payload_len: Some(payload.len() as u16),
                        signature: None,
                        multi_ack_mask: None,
                    };
                    let pkt = PRUDPPacket { header, payload: payload.clone() };
                    let bytes = pkt.to_bytes();
                    // send
                    if let Err(e) = socket_tx.send_to(&bytes[..], peer_clone).await {
                        tracing::error!("send error: {:?}", e);
                        // push back into queue
                        let mut q = send_queue_ref.lock().await;
                        q.push_front(payload);
                        break;
                    }
                    // record pending
                    let mut pend = pending_tx.lock().await;
                    pend.insert(seq, (bytes, 0u8, std::time::Instant::now()));
                    drop(pend);
                    w.mark_sent(seq);
                }
            }
        });

        conn
    }

    pub async fn queue_send(&self, payload: Bytes) -> Result<()> {
        let mut q = self.send_queue.lock().await;
        q.push_back(payload);
        // notify sender
        let _ = self.tx_notify.send(()).await;
        Ok(())
    }

    pub async fn handle_incoming_packet(&self, pkt: PRUDPPacket) -> Result<()> {
        // If ACK flag present, mark acked/multi-acked
        if pkt.header.flags.contains(Flags::ACK) {
            // Update RTT estimator with a sample RTT - in a real implementation we'd measure per-packet timestamps
            if let Some(ref rtt) = None::<std::sync::Arc<crate::rtt::RTT>> {
                // placeholder
            }

            let seq = pkt.header.seq;
            // remove from pending
            let mut pend = self.pending.lock().await;
            if pend.remove(&seq).is_some() {
                let mut w = self.window.lock();
                w.mark_acked(seq);
            }
            if pkt.header.flags.contains(Flags::MULTI_ACK) {
                if let Some(mask) = pkt.header.multi_ack_mask {
                    let start = seq;
                    let mut w = self.window.lock();
                    w.mark_acked_range(start, mask);
                }
            }
            return Ok(());
        }
        // For non-ACK packets, differentiate Connect/Data
        let ptype = pkt.packet_type();
        match ptype {
            PacketType::Connect => {
                tracing::info!("Received CONNECT packet seq={}", pkt.header.seq);
                // treat as data but may have special priority; push into dispatch queue
                let mut dq = self.dispatch_queue.lock().await;
                dq.queue_packet(pkt.clone());
            }
            PacketType::Data => {
                let seq = pkt.header.seq;
                let payload_vec = pkt.payload.to_vec();
                let mut w = self.window.lock();
                let (ready, nak_opt) = w.offer_receive(seq, payload_vec);
                drop(w);
                // push ready payloads into dispatch queue as packets
                for r in ready {
                    // create a fake PRUDPPacket with payload r and increasing seq? We'll wrap into PRUDPPacket with current seq
                    // Use next_expected_sequence_id from dispatch queue to approximate seq ordering
                    let mut dq = self.dispatch_queue.lock().await;
                    let seq_for = dq.next_expected_sequence_id;
                    let header = super::packet::PRUDPHeader {
                        version: pkt.header.version.clone(),
                        src: pkt.header.src,
                        dst: pkt.header.dst,
                        flags: super::packet::Flags::empty(),
                        session_id: pkt.header.session_id,
                        seq: seq_for,
                        payload_len: Some(r.len() as u16),
                        signature: None,
                        multi_ack_mask: None,
                        v1_extra: None,
                    };
                    let newpkt = super::packet::PRUDPPacket { header, payload: bytes::Bytes::from(r) };
                    dq.queue_packet(newpkt);
                    // try to dispatch in-order
                    while let Some(next) = dq.get_next_to_dispatch() {
                        dq.dispatched(&next);
                        // forward to HPP handler
                        let _ = self.hpp_tx.send(next.payload.clone()).await;
                    }
                }
                // If NAK requested, send it as before
                if let Some((start, mask)) = nak_opt {
                    let ack_header = PRUDPHeader {
                        version: pkt.header.version.clone(),
                        src: pkt.header.dst,
                        dst: pkt.header.src,
                        flags: Flags::ACK | Flags::MULTI_ACK,
                        session_id: pkt.header.session_id,
                        seq: start,
                        payload_len: Some(0),
                        signature: None,
                        multi_ack_mask: Some(mask),
                        v1_extra: None,
                    };
                    let ack_pkt = PRUDPPacket { header: ack_header, payload: Bytes::from(vec![]) };
                    let bytes = ack_pkt.to_bytes();
                    let _ = self.socket.send_to(&bytes[..], self.peer).await;
                }
            }
            _ => {
                tracing::debug!("Unhandled packet type: {:?}", ptype);
            }
        }
        // If NAK requested, send a NAK packet (we'll represent NAK as ACK+MULTI_ACK with mask)
        if let Some((start, mask)) = nak_opt {
            let ack_header = PRUDPHeader {
                version: pkt.header.version.clone(),
                src: pkt.header.dst,
                dst: pkt.header.src,
                flags: Flags::ACK | Flags::MULTI_ACK,
                session_id: pkt.header.session_id,
                seq: start,
                payload_len: Some(0),
                signature: None,
                multi_ack_mask: Some(mask),
            };
            let ack_pkt = PRUDPPacket { header: ack_header, payload: Bytes::from(vec![]) };
            let bytes = ack_pkt.to_bytes();
            let _ = self.socket.send_to(&bytes[..], self.peer).await;
        } else {
            // send normal ACK for the last-seen seq (seq)
            let ack_header = PRUDPHeader {
                version: pkt.header.version.clone(),
                src: pkt.header.dst,
                dst: pkt.header.src,
                flags: Flags::ACK,
                session_id: pkt.header.session_id,
                seq,
                payload_len: Some(0),
                signature: None,
                multi_ack_mask: None,
            };
            let ack_pkt = PRUDPPacket { header: ack_header, payload: Bytes::from(vec![]) };
            let bytes = ack_pkt.to_bytes();
            let _ = self.socket.send_to(&bytes[..], self.peer).await;
        }

        Ok(())
    }
}



impl super::connection::PRUDPConnection {
    /// Spawn a background task that periodically checks for retransmits and keepalive pings.
    pub fn spawn_retransmit_and_keepalive(self: std::sync::Arc<std::sync::Mutex<Self>>, socket: tokio::net::UdpSocket, peer: std::net::SocketAddr) {
        let arc = self.clone();
        tokio::spawn(async move {
            let mut ka_interval = interval(TokioDuration::from_millis(15000)); // 15s keepalive
            loop {
                // retransmit check every 200ms
                if let Ok(_) = timeout(TokioDuration::from_millis(200), async {}).await {
                    // nothing, just using timeout to await
                }
                // retransmit candidates
                {
                    let mut locked = arc.lock().unwrap();
                    let seqs = locked.window.retransmit_candidates();
                    for seq in seqs {
                        if let Some(payload) = locked.window.get_payload(seq) {
                            // Build a PRUDP data packet and send. Use existing helper if present.
                            if let Ok(bytes) = locked.build_prudp_data_packet_bytes(seq, payload.clone()) {
                                let _ = socket.send_to(&bytes[..], peer).await;
                            }
                        }
                    }
                }

                // keepalive handling
                ka_interval.tick().await;
                {
                    let locked = arc.lock().unwrap();
                    if let Err(e) = locked.send_ping(&socket, peer).await {
                        tracing::warn!("keepalive send failed: {:?}", e);
                    }
                }
            }
        });
    }
}
