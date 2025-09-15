
// Quazal-style RC4 wrapper: kept as a semantic wrapper around Rc4 for compatibility.
pub fn quazal_rc4_transform(key: &[u8], data: &mut [u8]) {
    crate::auth::rc4::Rc4::new(key).process(data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quazal_wrapper() {
        let mut data = b"testing".to_vec();
        let key = b"k";
        let mut orig = data.clone();
        quazal_rc4_transform(key, &mut data);
        // decrypt
        quazal_rc4_transform(key, &mut data);
        assert_eq!(data, orig);
    }
}
