
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::Result;
use bytes::Bytes;
use crate::hpp::packet::HppPacket;
use std::sync::Arc;
use crate::hpp::dispatcher::{HppHandler, SimpleHandler};
use tracing::info;

pub struct HppServer {
    listener: TcpListener,
    handler: Arc<dyn HppHandler>,
}

impl HppServer {
    pub async fn bind(addr: &str, handler: Arc<dyn HppHandler>) -> Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        Ok(Self { listener, handler })
    }

    pub async fn run(&self) -> Result<()> {
        loop {
            let (mut stream, peer) = self.listener.accept().await?;
            let handler = self.handler.clone();
            tokio::spawn(async move {
                info!("HPP client connected: {}", peer);
                // read loop for framed HPP packets
                loop {
                    let mut len_buf = [0u8; 2];
                    if let Err(e) = stream.read_exact(&mut len_buf).await {
                        tracing::debug!("client disconnected or read error: {:?}", e);
                        break;
                    }
                    let len = u16::from_le_bytes(len_buf) as usize;
                    let mut body = vec![0u8; len];
                    if let Err(e) = stream.read_exact(&mut body).await {
                        tracing::debug!("read body error: {:?}", e);
                        break;
                    }
                    match handler.on_hpp_message(Bytes::from(body)).await {
                        Ok(Some(resp)) => {
                            // write framed response back
                            let framed = crate::hpp::packet::HppPacket::encode(&resp);
                            if let Err(e) = stream.write_all(&framed[..]).await {
                                tracing::debug!("failed to write response: {:?}", e);
                                break;
                            }
                        }
                        Ok(None) => {}
                        Err(e) => {
                            tracing::debug!("handler error: {:?}", e);
                            break;
                        }
                    };
                }
            });
        }
    }
}
