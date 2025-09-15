
use crate::error::Result;

/// Common PRUDP packet fields shared by v0/v1.
#[derive(Debug, Clone, Default)]
pub struct PRUDPPacket {
    pub version: u8,
    pub source_virtual_port: u8,
    pub destination_virtual_port: u8,
    pub packet_type: u16,
    pub flags: u16,
    pub session_id: u8,
    pub substream_id: u8,
    pub sequence_id: u16,
    pub signature: [u8; 16],
    pub connection_signature: [u8; 16],
    pub fragment_id: u8,
    pub payload: Vec<u8>,
}

impl PRUDPPacket {
    pub fn new() -> Self { Self::default() }
    pub fn set_signature(&mut self, sig: [u8;16]) { self.signature = sig; }
    pub fn set_connection_signature(&mut self, sig: [u8;16]) { self.connection_signature = sig; }
}

