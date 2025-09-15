
use crate::error::{Result, Error};
use crate::prudp_packet_v1::PRUDPPacketV1;
use crate::byte_stream_in::ByteStreamIn;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use std::net::SocketAddr;

pub struct PRUDPServer {
    socket: UdpSocket,
}

impl PRUDPServer {
    pub async fn bind(addr: SocketAddr) -> Result<(Self, mpsc::Receiver<(SocketAddr, PRUDPPacketV1)>)> {
        let socket = UdpSocket::bind(addr).await.map_err(Error::from)?;
        let (tx, rx) = mpsc::channel(1024);
        let recv_socket = socket.try_clone().map_err(Error::from)?;
        tokio::spawn(async move {
            let mut buf = vec![0u8; 65536];
            loop {
                match recv_socket.recv_from(&mut buf).await {
                    Ok((n, peer)) => {
                        let bs = ByteStreamIn::from_bytes(bytes::Bytes::from(buf[..n].to_vec()));
                        if let Ok(pkt) = PRUDPPacketV1::decode(bs) {
                            let _ = tx.send((peer, pkt)).await;
                        }
                    }
                    Err(_) => break,
                }
            }
        });
        Ok((Self { socket }, rx))
    }

    pub async fn send_to(&self, addr: SocketAddr, bytes: &[u8]) -> Result<()> {
        self.socket.send_to(bytes, addr).await.map_err(Error::from)?;
        Ok(())
    }
}
