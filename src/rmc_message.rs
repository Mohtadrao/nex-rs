
use crate::byte_stream::ByteStreamOut;
use crate::byte_stream_in::ByteStreamIn;
use crate::error::{Result, Error};

/// Minimal RMC message for requests/responses.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RMCMessage {
    pub protocol_id: u32,
    pub method_id: u32,
    pub call_id: u32,
    pub body: Vec<u8>,
}

impl RMCMessage {
    pub fn to_bytes(&self) -> bytes::Bytes {
        let mut out = ByteStreamOut::new();
        out.put_u32_le(self.protocol_id);
        out.put_u32_le(self.method_id);
        out.put_u32_le(self.call_id);
        out.put_u32_le(self.body.len() as u32);
        for b in &self.body { out.put_u8(*b); }
        out.into_bytes()
    }

    pub fn from_bytes(mut bs: ByteStreamIn) -> Result<Self> {
        if bs.remaining() < 16 { return Err(Error::new(-1, "rmc underflow")); }
        let protocol_id = bs.read_u32_le()?;
        let method_id = bs.read_u32_le()?;
        let call_id = bs.read_u32_le()?;
        let body_len = bs.read_u32_le()? as usize;
        if bs.remaining() < body_len { return Err(Error::new(-1, "rmc body underflow")); }
        let body = bs.read_bytes(body_len)?;
        Ok(Self { protocol_id, method_id, call_id, body })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn roundtrip() {
        let msg = RMCMessage { protocol_id: 5, method_id: 9, call_id: 42, body: vec![1,2,3] };
        let bytes = msg.to_bytes();
        let parsed = RMCMessage::from_bytes(ByteStreamIn::from_bytes(bytes)).unwrap();
        assert_eq!(msg, parsed);
    }
}
