#[cfg(feature = "zlib")]
use flate2::{write::ZlibEncoder, read::ZlibDecoder, Compression};
#[cfg(feature = "zlib")]
use std::io::{Read, Write};

#[cfg(feature = "zlib")]
use super::algorithm::Algorithm;

#[cfg(feature = "zlib")]
#[derive(Default, Debug, Clone)]
pub struct Zlib;

#[cfg(feature = "zlib")]
impl Algorithm for Zlib {
    fn compress(&self, payload: &[u8]) -> crate::Result<Vec<u8>> {
        let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
        enc.write_all(payload)?;
        Ok(enc.finish()?)
    }
    fn decompress(&self, payload: &[u8]) -> crate::Result<Vec<u8>> {
        let mut dec = ZlibDecoder::new(payload);
        let mut out = Vec::new();
        dec.read_to_end(&mut out)?;
        Ok(out)
    }
    fn boxed(&self) -> Box<dyn Algorithm> { Box::new(self.clone()) }
}
