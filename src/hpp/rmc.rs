
use bytes::Bytes;
use anyhow::Context;

#[derive(Debug)]
pub struct RmcMessage {
    pub method_id: u32,
    pub body: Bytes,
}

impl RmcMessage {
    pub fn parse(mut b: Bytes) -> anyhow::Result<Self> {
        if b.len() < 4 { return Err(anyhow::anyhow!("rmc too short")); }
        let id = u32::from_le_bytes([b[0], b[1], b[2], b[3]]);
        let _ = b.split_to(4);
        let body = b.split_to(b.len());
        Ok(RmcMessage { method_id: id, body })
    }
}
