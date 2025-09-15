
use bytes::BytesMut;
use crate::byte_stream::ByteStreamOut as RawOut;
use crate::Result;

#[derive(Debug, Clone)]
pub struct ByteStreamOut {
    inner: RawOut,
    pub settings: Option<crate::byte_stream_settings::ByteStreamSettings>,
}

impl ByteStreamOut {
    pub fn new() -> Self { Self { inner: RawOut::new(), settings: None } }
    pub fn put_u8(&mut self, v: u8) { self.inner.put_u8(v); }
    pub fn put_u16_le(&mut self, v: u16) { self.inner.put_u16_le(v); }
    pub fn put_u32_le(&mut self, v: u32) { self.inner.put_u32_le(v); }
    pub fn put_bytes(&mut self, b: &[u8]) { self.inner.put_bytes(b); }
    pub fn put_string_u16(&mut self, s: &str) { self.inner.put_string_u16(s); }
    pub fn put_pid(&mut self, pid: u32) { self.inner.put_pid(pid); }
    pub fn into_bytes(self) -> bytes::Bytes { self.inner.into_bytes() }
}
