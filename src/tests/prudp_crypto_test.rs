
#[cfg(test)]
mod prudp_crypto_test {
    use crate::prudp::crypto;
    use crate::prudp::crypto::derive_session_key_from_ticket;
    use crate::prudp::crypto::encrypt_packet;
    use crate::prudp::crypto::decrypt_packet;

    #[test]
    fn test_session_key_derivation_and_use() {
        let ticket = b\"KRBabc\".to_vec();
        let salt = 0x01020304u32;
        let key = derive_session_key_from_ticket(&ticket, salt).unwrap();
        let mut payload = b\"encryptme\".to_vec();
        let orig = payload.clone();
        encrypt_packet(&key, &mut payload);
        decrypt_packet(&key, &mut payload);
        assert_eq!(payload, orig);
    }
}
