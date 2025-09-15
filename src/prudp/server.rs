
use tokio::net::UdpSocket;
use std::sync::Arc;
use bytes::Bytes;
use crate::Result;
use std::collections::HashMap;
use tokio::sync::Mutex as AsyncMutex;
use std::net::SocketAddr;
use super::packet::PRUDPPacket;
use super::connection::{Connection, HppSender};
use tracing::info;
use tokio::sync::mpsc;
use crate::hpp::dispatcher::{SimpleHandler, HppHandler};

pub struct PRUDPServer {
    sock: Arc<UdpSocket>,
    peers: AsyncMutex<HashMap<SocketAddr, Arc<Connection>>>,
    // keep handler alive
    _handler: Arc<SimpleHandler>,
}

impl PRUDPServer {
    pub async fn bind(addr: &str) -> Result<Self> {
        let sock = UdpSocket::bind(addr).await?;
        let (hpp_tx, mut hpp_rx) = mpsc::channel::<Bytes>(1024);
        let handler = Arc::new(SimpleHandler);
        // spawn handler loop
        let handler_clone = handler.clone();
        tokio::spawn(async move {
            while let Some(msg) = hpp_rx.recv().await {
                let _ = handler_clone.on_hpp_message(msg).await;
            }
        });
        Ok(Self {
            sock: Arc::new(sock),
            peers: AsyncMutex::new(HashMap::new()),
            _handler: handler,
        })
    }

    pub async fn run(&self) -> Result<()> {
        let mut buf = vec![0u8; 4096];
        loop {
            let (n, peer) = self.sock.recv_from(&mut buf).await?;
            let data = Bytes::copy_from_slice(&buf[..n]);
            // parse packet
            match PRUDPPacket::parse(data.clone()) {
                Ok(pkt) => {
                    let mut peers = self.peers.lock().await;
                    let entry = peers.entry(peer);
                    let arc_conn = match entry {
                        std::collections::hash_map::Entry::Occupied(o) => o.get().clone(),
                        std::collections::hash_map::Entry::Vacant(v) => {
                            // create connection with hpp sender cloned from server-side channel
                            // to keep backward compatibility, recreate a new channel per-server. For now,
                            // we use a global channel (not ideal but acceptable for this scaffold).
                            // NOTE: item left for later
                            let let tx = hpp_tx.clone();
                            let conn = Arc::new(tokio::runtime::Handle::current().block_on(async {
                                Connection::new(self.sock.clone(), peer, tx).await
                            }));
                            v.insert(conn.clone());
                            conn
                        }
                    };
                    drop(peers);
                    // handle packet by connection
                    let _ = arc_conn.handle_incoming_packet(pkt).await;
                }
                Err(e) => {
                    tracing::warn!(error = ?e, "failed to parse PRUDP packet");
                }
            }
        }
    }
}
