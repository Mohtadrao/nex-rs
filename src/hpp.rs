
// HPP high-level helpers (minimal)
use crate::kerberos::derive_kerberos_key;
use crate::types::pid::PID;
use crate::hpp_packet::HPPPacket;

/// Create an HPP packet from an RMC message and compute both signatures.
pub fn build_hpp_with_signatures(pid: PID, password: &str, access_key_hex: &str, msg: crate::rmc_message::RMCMessage) -> crate::error::Result<HPPPacket> {
    let mut pkt = HPPPacket::from_rmc(pid.0, msg);
    // access key signature
    pkt.access_key_signature = pkt.calculate_access_key_signature(access_key_hex)?;
    // password signature via Kerberos-derived key
    let key = derive_kerberos_key(pid, password.as_bytes());
    pkt.password_signature = pkt.calculate_password_signature(&key);
    Ok(pkt)
}
