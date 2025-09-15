
#[cfg(test)]
mod kerberos_derive_test {
    use crate::auth::kerberos;
    use hex_literal::hex;

    #[test]
    fn test_derive_guest_key() {
        let pid = 100u32;
        let password = b"MMQea3n!fsik";
        let res = kerberos::derive_kerberos_key(pid, password);
        let hex = hex::encode(res);
        assert_eq!(hex, "9ef318f0a170fb46aab595bf9644f9e1");
    }
}
