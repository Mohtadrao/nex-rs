
use md5::{Md5, Digest};
use crate::types::pid::PID;

/// DeriveKerberosKey replicates nex-go logic:
/// iteration_count = 65000 + (pid % 1024)
/// then iterated MD5 hashing of the password bytes.
pub fn derive_kerberos_key(pid: PID, password: &[u8]) -> Vec<u8> {
    let iteration_count: u32 = 65000 + (u32::from(pid) % 1024);
    let mut key = password.to_vec();
    for _ in 0..iteration_count {
        let mut hasher = Md5::new();
        hasher.update(&key);
        key = hasher.finalize().to_vec();
    }
    key
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deterministic() {
        let k1 = derive_kerberos_key(PID(12345), b"pass");
        let k2 = derive_kerberos_key(PID(12345), b"pass");
        assert_eq!(k1, k2);
        assert_eq!(k1.len(), 16); // md5 size
    }
}
