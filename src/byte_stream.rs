
use bytes::{Buf, BufMut, BytesMut};
use byteorder::{ByteOrder, LittleEndian};

/// Simple ByteStreamOut for writing primitive types.
#[derive(Debug, Default, Clone)]
pub struct ByteStreamOut {
    buf: BytesMut,
}

impl ByteStreamOut {
    pub fn new() -> Self { Self { buf: BytesMut::with_capacity(256) } }
    pub fn into_bytes(self) -> bytes::Bytes { self.buf.freeze() }

    pub fn put_u8(&mut self, v: u8) { self.buf.put_u8(v); }
    pub fn put_u16_le(&mut self, v: u16) { self.buf.put_u16_le(v); }
    pub fn put_u32_le(&mut self, v: u32) { self.buf.put_u32_le(v); }
    pub fn put_u64_le(&mut self, v: u64) { self.buf.put_u64_le(v); }
    pub fn put_bytes(&mut self, b: &[u8]) { self.buf.extend_from_slice(b); }

    /// Writes a length-prefixed string where length is u16 (little-endian)
    pub fn put_string_u16(&mut self, s: &str) {
        let bytes = s.as_bytes();
        let len = bytes.len() as u16;
        self.put_u16_le(len);
        self.put_bytes(bytes);
    }

    /// Writes a PID (u32)
    pub fn put_pid(&mut self, pid: u32) { self.put_u32_le(pid); }
}

/// Simple ByteStreamIn for reading primitive types.
#[derive(Debug, Clone)]
pub struct ByteStreamIn {
    buf: bytes::Bytes,
}

impl ByteStreamIn {
    pub fn from_bytes(b: bytes::Bytes) -> Self { Self { buf: b } }
    pub fn remaining(&self) -> usize { self.buf.len() }

    pub fn get_u8(&mut self) -> crate::Result<u8> {
        if self.buf.len() < 1 { return Err(crate::error::Error::new(-1, "underflow")); }
        Ok(self.buf.split_to(1)[0])
    }
    pub fn get_u16_le(&mut self) -> crate::Result<u16> {
        if self.buf.len() < 2 { return Err(crate::error::Error::new(-1, "underflow")); }
        let mut tmp = [0u8;2]; tmp.copy_from_slice(&self.buf.split_to(2));
        Ok(u16::from_le_bytes(tmp))
    }
    pub fn get_u32_le(&mut self) -> crate::Result<u32> {
        if self.buf.len() < 4 { return Err(crate::error::Error::new(-1, "underflow")); }
        let mut tmp = [0u8;4]; tmp.copy_from_slice(&self.buf.split_to(4));
        Ok(u32::from_le_bytes(tmp))
    }

    /// Reads a length-prefixed u16 string
    pub fn get_string_u16(&mut self) -> crate::Result<String> {
        let len = self.get_u16_le()? as usize;
        if self.buf.len() < len { return Err(crate::error::Error::new(-1, "underflow")); }
        let bytes = self.buf.split_to(len);
        match std::str::from_utf8(&bytes) {
            Ok(s) => Ok(s.to_string()),
            Err(_) => Err(crate::error::Error::new(-1, "invalid utf8")),
        }
    }

    /// Reads a PID (u32)
    pub fn get_pid(&mut self) -> crate::Result<u32> { Ok(self.get_u32_le()?) }
}
