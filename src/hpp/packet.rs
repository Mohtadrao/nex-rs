use byteorder::{ByteOrder, LittleEndian};
use bytes::{Bytes, BytesMut, BufMut};
// use byteorder::LittleEndian;

#[derive(Debug, Clone)]
pub struct HppPacket {
    pub length: u16,
    pub payload: Bytes,
}

impl HppPacket {
    pub fn encode(payload: &Bytes) -> Bytes {
        let mut bm = BytesMut::with_capacity(2 + payload.len());
        bm.put_u16_le(payload.len() as u16);
        bm.extend_from_slice(&payload[..]);
        bm.freeze()
    }

    pub fn parse(mut b: Bytes) -> anyhow::Result<Self> {
        if b.len() < 2 { return Err(anyhow::anyhow!("hpp packet too short")); }
        let len = LittleEndian::read_u16(&b[..2]) as usize;
        let _ = b.split_to(2);
        if b.len() < len { return Err(anyhow::anyhow!("hpp payload truncated")); }
        let payload = b.split_to(len);
        Ok(HppPacket { length: len as u16, payload })
    }
}
