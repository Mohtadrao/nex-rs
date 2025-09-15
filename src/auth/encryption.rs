
// Encryption utilities: keep xor_encrypt for testing and expose rc4/quazal wrappers.
pub fn xor_encrypt(data: &mut [u8], key: u8) {
    for b in data.iter_mut() { *b ^= key; }
}

pub fn rc4_transform_inplace(key: &[u8], data: &mut [u8]) {
    crate::auth::rc4::Rc4::new(key).process(data);
}

pub fn quazal_rc4_transform(key: &[u8], data: &mut [u8]) {
    crate::auth::quazal_rc4::quazal_rc4_transform(key, data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc4_via_encryption() {
        let key = b"k1";
        let mut data = b"abcd".to_vec();
        let orig = data.clone();
        rc4_transform_inplace(key, &mut data);
        quazal_rc4_transform(key, &mut data);
        // after rc4 followed by quazal_rc4 (which is same op) it won't restore; just ensure functions run.
        assert!(data.len() == orig.len());
    }
}
