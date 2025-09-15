
use bytes::Bytes;
use crate::Result;
use crate::byte_stream::ByteStreamIn as RawIn;

#[derive(Debug, Clone)]
pub struct ByteStreamIn {
    inner: RawIn,
    pub settings: Option<crate::byte_stream_settings::ByteStreamSettings>,
}

impl ByteStreamIn {
    pub fn from_bytes(b: Bytes) -> Self {
        Self { inner: RawIn::from_bytes(b), settings: None }
    }

    pub fn remaining(&self) -> usize { self.inner.remaining() }

    pub fn read_u8(&mut self) -> Result<u8> { self.inner.get_u8() }
    pub fn read_u16_le(&mut self) -> Result<u16> { self.inner.get_u16_le() }
    pub fn read_string_u16(&mut self) -> Result<String> { self.inner.get_string_u16() }
    pub fn read_pid(&mut self) -> Result<u32> { self.inner.get_pid() }
    pub fn read_bytes(&mut self, n: usize) -> Result<Vec<u8>> {
        let mut out = vec![0u8; n];
        for i in 0..n {
            out[i] = self.inner.get_u8()?;
        }
        Ok(out)
    }
}
