
use anyhow::Result;

pub struct CryptoStub;

impl CryptoStub {
    pub fn new() -> Self { Self{} }

    pub fn aes_encrypt(&self, _key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
        Ok(data.to_vec())
    }
    pub fn aes_decrypt(&self, _key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
        Ok(data.to_vec())
    }
    pub fn hmac(&self, _key: &[u8], _data: &[u8]) -> Result<Vec<u8>> {
        Ok(vec![])
    }
}
