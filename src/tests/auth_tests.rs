
#[cfg(test)]
mod auth_tests {
    use crate::auth::kerberos;
    use crate::auth::encryption;

    #[test]
    fn test_kerberos_stub() {
        assert!(kerberos::validate_ticket(&[]).unwrap());
        assert!(!kerberos::validate_ticket(&[1,2,3]).unwrap());
    }

    #[test]
    fn test_encryption_xor() {
        let mut d = vec![1u8,2,3];
        encryption::xor_encrypt(&mut d, 0xff);
        assert_eq!(d, vec![254u8,253u8,252u8]);
    }
}
