
#[cfg(test)]
mod kerberos_tests {
    use crate::crypto::kerberos::Kerberos;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let ticket = Kerberos::issue_ticket(42);
        let msg = b"hello world";
        let enc = Kerberos::encrypt(&ticket, msg);
        let dec = Kerberos::decrypt(&ticket, &enc);
        assert_eq!(&dec[..], msg);
    }
}
