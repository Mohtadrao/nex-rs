
#[cfg(test)]
mod prudp_crypto_tests {
    use crate::prudp::crypto::{derive_key_stub, PrudpCrypto};
    use bytes::Bytes;

    #[test]
    fn test_crypto_key_derivation_and_transform() {
        let key = derive_key_stub(100, b\"password\");
        let mut c = PrudpCrypto::new(&key);
        let mut data = b\"abcdefgh\".to_vec();
        c.transform(&mut data);
        // decrypt
        let mut c2 = PrudpCrypto::new(&key);
        c2.transform(&mut data);
        assert_eq!(&data, b\"abcdefgh\");
    }
}
