
use crate::error::Result;
use std::net::SocketAddr;

/// Settings for PRUDP v1 signature behavior.
#[derive(Debug, Clone)]
pub struct PRUDPV1Settings {
    /// If true, omit the connection signature from Connect packets when calculating the signature.
    pub legacy_connection_signature: bool,
    /// Key used for the default connection signature HMAC (MD5) over (IPv4 || port_be).
    pub connection_signature_key: Vec<u8>,
    /// Optional access key for default signature calculation (md5(access_key) is used as HMAC key).
    pub access_key: String,
}

impl Default for PRUDPV1Settings {
    fn default() -> Self {
        Self {
            legacy_connection_signature: false,
            connection_signature_key: vec![0u8; 16],
            access_key: String::new(),
        }
    }
}

impl PRUDPV1Settings {
    /// HMAC-MD5 of IPv4 + big-endian port using connection_signature_key.
    pub fn default_connection_signature(&self, addr: SocketAddr) -> [u8;16] {
        use md5::Md5;
        use hmac::{Hmac, Mac};
        type HmacMd5 = Hmac<Md5>;

        let mut data = Vec::with_capacity(6);
        match addr {
            SocketAddr::V4(v4) => {
                data.extend_from_slice(&v4.ip().octets());
                data.extend_from_slice(&v4.port().to_be_bytes());
            }
            SocketAddr::V6(_) => {
                // For simplicity, only IPv4 is supported here—the original library uses To4().
                // You may adapt if v6 is relevant for your use case.
                return [0u8;16];
            }
        }
        let mut mac = HmacMd5::new_from_slice(&self.connection_signature_key).expect("hmac key");
        mac.update(&data);
        let out = mac.finalize().into_bytes();
        let mut sig = [0u8;16];
        sig.copy_from_slice(&out[..16]);
        sig
    }

    /// Default packet signature per nex-go:
    /// hmac_md5(md5(access_key), header[4:] || session_key || sum(access_key) (le u32) || connection_signature || options || payload)
    pub fn default_signature(&self, header: &[u8], options: &[u8], payload: &[u8],
                             session_key: &[u8], connection_signature: Option<&[u8]>,
                             include_conn_sig: bool) -> [u8;16] {
        use md5::{Md5, Digest};
        use hmac::{Hmac, Mac};
        type HmacMd5 = Hmac<Md5>;

        let access_key_bytes = self.access_key.as_bytes();
        let mut md = Md5::new();
        md.update(access_key_bytes);
        let key_md5 = md.finalize(); // 16 bytes

        let sum_u32 = access_key_bytes.iter().fold(0u32, |acc, b| acc.wrapping_add(*b as u32));
        let mut sum_bytes = [0u8;4];
        sum_bytes.copy_from_slice(&sum_u32.to_le_bytes());

        let mut mac = HmacMd5::new_from_slice(&key_md5).expect("hmac key");

        // header[4:] skips magic (0xEAD0)
        if header.len() >= 4 {
            mac.update(&header[4..]);
        } else {
            mac.update(header);
        }
        mac.update(session_key);
        mac.update(&sum_bytes);
        if include_conn_sig {
            if let Some(cs) = connection_signature {
                mac.update(cs);
            }
        }
        mac.update(options);
        mac.update(payload);

        let out = mac.finalize().into_bytes();
        let mut sig = [0u8;16];
        sig.copy_from_slice(&out[..16]);
        sig
    }
}
