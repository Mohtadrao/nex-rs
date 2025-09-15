
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use bytes::Bytes;
use crate::Result;
use crate::hpp::packet::HppPacket;
use tracing::info;

pub struct HppClient {
    stream: TcpStream,
}

impl HppClient {
    pub async fn connect(addr: &str) -> Result<Self> {
        let stream = TcpStream::connect(addr).await?;
        Ok(Self { stream })
    }

    /// Example send framed HPP packet and await a response (simplistic)
    pub async fn send_and_receive(&mut self, payload: Bytes) -> Result<Bytes> {
        let bytes = HppPacket::encode(&payload);
        self.stream.write_all(&bytes[..]).await?;
        // read length prefix
        let mut len_buf = [0u8; 2];
        self.stream.read_exact(&mut len_buf).await?;
        let len = u16::from_le_bytes(len_buf) as usize;
        let mut body = vec![0u8; len];
        self.stream.read_exact(&mut body).await?;
        Ok(Bytes::from(body))
    }
}
