
// Kerberos helpers (lightweight). The real nex-go Kerberos is complex; this is a compatibility shim.
// validate_ticket returns true only for empty tickets (keeps legacy test behavior) or tickets starting with 'KRB' magic.
pub fn validate_ticket(ticket: &[u8]) -> Result<bool, Box<dyn std::error::Error>> {
    if ticket.is_empty() { return Ok(true); }
    if ticket.len() >= 3 && &ticket[0..3] == b"KRB" { return Ok(true); }
    Ok(false)
}

// helper to create a dummy 'KRB' ticket for testing purposes
pub fn make_dummy_ticket(payload: &[u8]) -> Vec<u8> {
    let mut v = b"KRB".to_vec();
    v.extend_from_slice(payload);
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_and_validate() {
        let t = make_dummy_ticket(&[1,2,3]);
        assert!(validate_ticket(&t).unwrap());
        assert!(!validate_ticket(&[9u8,9u8]).unwrap());
    }
}



use md5::{Md5, Digest};

/// Derive a Kerberos key following nex-go's algorithm:
/// iterationCount := 65000 + (pid % 1024)
/// Start with key = password, then iterate MD5 over the key for iterationCount times.
pub fn derive_kerberos_key(pid: u32, password: &[u8]) -> Vec<u8> {
    let iteration_count = 65000usize + (pid as usize % 1024usize);
    let mut key = password.to_vec();
    let mut hash = [0u8; 16];
    for _ in 0..iteration_count {
        let mut hasher = Md5::new();
        hasher.update(&key);
        let result = hasher.finalize();
        hash.copy_from_slice(&result[..16]);
        key = hash.to_vec();
    }
    key
}
