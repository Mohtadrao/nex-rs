// Placeholder LZO implementation; requires an external crate. Enabled with `lzo` feature.
#[cfg(feature = "lzo")]
use super::algorithm::Algorithm;

#[cfg(feature = "lzo")]
#[derive(Default, Debug, Clone)]
pub struct Lzo;

#[cfg(feature = "lzo")]
impl Algorithm for Lzo {
    fn compress(&self, _payload: &[u8]) -> crate::Result<Vec<u8>> {
        // NOTE: item left for later
        Err(crate::error::Error::new(-1, "LZO compression not implemented"))
    }
    fn decompress(&self, _payload: &[u8]) -> crate::Result<Vec<u8>> {
        Err(crate::error::Error::new(-1, "LZO decompression not implemented"))
    }
    fn boxed(&self) -> Box<dyn Algorithm> { Box::new(self.clone()) }
}
