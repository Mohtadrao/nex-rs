use super::algorithm::EncryptionAlgorithm;

#[derive(Clone, Debug, Default)]
pub struct DummyEncryption;

impl EncryptionAlgorithm for DummyEncryption {
    fn encrypt(&self, input: &[u8]) -> crate::Result<Vec<u8>> {
        Ok(input.to_vec())
    }
    fn decrypt(&self, input: &[u8]) -> crate::Result<Vec<u8>> {
        Ok(input.to_vec())
    }
    fn boxed(&self) -> Box<dyn EncryptionAlgorithm> { Box::new(self.clone()) }
}
