
use crate::error::{Result, Error};
use crate::rmc_message::RMCMessage;
use crate::byte_stream_in::ByteStreamIn;
use hmac::{Hmac, Mac};
use md5::Md5;

type HmacMd5 = Hmac<Md5>;

#[derive(Debug, Clone, Default)]
pub struct HPPPacket {
    pub sender_pid: u32,
    pub payload: Vec<u8>,
    pub access_key_signature: [u8;16],
    pub password_signature: [u8;16],
    pub message: Option<RMCMessage>,
}

impl HPPPacket {
    pub fn from_rmc(sender_pid: u32, msg: RMCMessage) -> Self {
        let payload = msg.to_bytes().to_vec();
        Self { sender_pid, payload, message: Some(msg), ..Default::default() }
    }

    pub fn parse(sender_pid: u32, payload: Vec<u8>) -> Result<Self> {
        // best-effort parse RMC
        let msg = RMCMessage::from_bytes(ByteStreamIn::from_bytes(bytes::Bytes::from(payload.clone()))).ok();
        Ok(Self { sender_pid, payload, message: msg, ..Default::default() })
    }

    pub fn calculate_access_key_signature(&self, access_key_hex: &str) -> Result<[u8;16]> {
        let key = hex::decode(access_key_hex).map_err(|e| Error::new(-1, format!("bad access key hex: {e}")))?;
        Ok(hmac_md5(&key, &self.payload))
    }

    pub fn calculate_password_signature(&self, password_key: &[u8]) -> [u8;16] {
        hmac_md5(password_key, &self.payload)
    }

    pub fn validate_access_key_signature(&self, access_key_hex: &str) -> Result<()> {
        let calc = self.calculate_access_key_signature(access_key_hex)?;
        if calc != self.access_key_signature {
            return Err(Error::new(-1, "Access key signature does not match"));
        }
        Ok(())
    }

    pub fn validate_password_signature(&self, password_key: &[u8]) -> Result<()> {
        let calc = self.calculate_password_signature(password_key);
        if calc != self.password_signature {
            return Err(Error::new(-1, "Password signature does not match"));
        }
        Ok(())
    }
}

fn hmac_md5(key: &[u8], buf: &[u8]) -> [u8;16] {
    let mut mac = HmacMd5::new_from_slice(key).expect("hmac key");
    mac.update(buf);
    let out = mac.finalize().into_bytes();
    let mut arr = [0u8;16];
    arr.copy_from_slice(&out[..16]);
    arr
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sigs() {
        let msg = RMCMessage { protocol_id: 1, method_id: 2, call_id: 3, body: b"abc".to_vec() };
        let pkt = HPPPacket::from_rmc(100, msg);
        let access_key_hex = "00112233445566778899aabbccddeeff";
        let access_sig = pkt.calculate_access_key_signature(access_key_hex).unwrap();
        let pwd_key = [0x55u8;16];
        let pwd_sig = pkt.calculate_password_signature(&pwd_key);
        assert_ne!(access_sig, pwd_sig);
    }
}
