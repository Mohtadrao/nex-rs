
// Simple PRUDP crypto stubs for compatibility testing.
// This is NOT secure. It's a deterministic XOR stream cipher used only for testing/interop.
// Real implementation should port the exact Quazal/NEX crypto primitives.
pub struct PrudpCrypto {
    pub key: Vec<u8>,
    pub pos: usize,
}

impl PrudpCrypto {
    pub fn new(key: &[u8]) -> Self {
        Self { key: key.to_vec(), pos: 0 }
    }

    /// XOR-inplace transform with repeating key stream
    pub fn transform(&mut self, data: &mut [u8]) {
        for b in data.iter_mut() {
            let k = self.key[self.pos % self.key.len()];
            *b ^= k;
            self.pos = self.pos.wrapping_add(1);
        }
    }
}

/// Simple key derivation stub: take md5 of password & pid, return 16-byte key
pub fn derive_key_stub(pid: u32, password: &[u8]) -> Vec<u8> {
    use md5::{Md5, Digest};
    let mut hasher = Md5::new();
    hasher.update(&pid.to_le_bytes());
    hasher.update(password);
    hasher.finalize().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prudp_crypto_roundtrip() {
        let key = derive_key_stub(42, b"secret");
        let mut c1 = PrudpCrypto::new(&key);
        let mut data = b"hello world".to_vec();
        c1.transform(&mut data);
        // decrypt with fresh cipher reinitialized (same key)
        let mut c2 = PrudpCrypto::new(&key);
        c2.transform(&mut data);
        assert_eq!(&data, b"hello world");
    }
}
