
use crate::byte_stream::{ByteStreamOut};
use crate::byte_stream_in::ByteStreamIn;
use crate::constants::prudp_packet_types::PRUDPPacketType;
use crate::error::{Result, Error};
use crate::prudp_packet::PRUDPPacket;
use crate::prudp_v1_settings::PRUDPV1Settings;

/// PRUDP v1 packet (compatible with nex-go layout).
#[derive(Debug, Clone, Default)]
pub struct PRUDPPacketV1 {
    pub base: PRUDPPacket,
    pub options_length: u8,
    pub payload_length: u16,
    pub minor_version: u32,
    pub supported_functions: u32,
    pub maximum_substream_id: u8,
    pub initial_unreliable_sequence_id: u16,
}

impl PRUDPPacketV1 {
    /// Decode a v1 packet from bytes (including magic/header/signature/options/payload).
    pub fn decode(mut stream: ByteStreamIn) -> Result<Self> {
        if stream.remaining() < 2 { return Err(Error::new(-1, "not enough bytes for magic")); }
        let magic = stream.read_bytes(2)?;
        if magic != vec![0xEA, 0xD0] { return Err(Error::new(-1, "invalid PRUDPv1 magic")); }

        let mut pkt = PRUDPPacketV1::default();
        pkt.decode_header(&mut stream)?;

        if stream.remaining() < 16 { return Err(Error::new(-1, "not enough bytes for signature")); }
        let sig = stream.read_bytes(16)?;
        pkt.base.signature.copy_from_slice(&sig[..16]);

        // Options
        if stream.remaining() < pkt.options_length as usize { return Err(Error::new(-1,"not enough option bytes")); }
        let opts = stream.read_bytes(pkt.options_length as usize)?;
        pkt.decode_options(&mut ByteStreamIn::from_bytes(bytes::Bytes::from(opts)))?;

        // Payload
        if stream.remaining() < pkt.payload_length as usize { return Err(Error::new(-1,"not enough payload bytes")); }
        pkt.base.payload = stream.read_bytes(pkt.payload_length as usize)?;

        Ok(pkt)
    }

    fn decode_header(&mut self, stream: &mut ByteStreamIn) -> Result<()> {
        if stream.remaining() < 12 { return Err(Error::new(-1, "not enough header bytes")); }
        let version = stream.read_u8()?;
        if version != 1 { return Err(Error::new(-1, format!("invalid version {}", version))); }
        self.options_length = stream.read_u8()?;
        self.payload_length = stream.read_u16_le()?;
        self.base.source_virtual_port = stream.read_u8()?;
        self.base.destination_virtual_port = stream.read_u8()?;
        // type and flags packed: type in low 4 bits, flags in high bits (shifted by 4)
        let type_and_flags = stream.read_u16_le()?;
        self.base.flags = type_and_flags >> 4;
        self.base.packet_type = type_and_flags & 0xF;
        self.base.session_id = stream.read_u8()?;
        self.base.substream_id = stream.read_u8()?;
        self.base.sequence_id = stream.read_u16_le()?;
        self.base.version = 1;
        Ok(())
    }

    fn decode_options(&mut self, stream: &mut ByteStreamIn) -> Result<()> {
        while stream.remaining() > 0 {
            let option_id = stream.read_u8()?;
            let _size = stream.read_u8()?; // size hint, we infer based on id
            match self.base.packet_type {
                x if x == PRUDPPacketType::Syn as u16 || x == PRUDPPacketType::Connect as u16 => {
                    match option_id {
                        0 => {
                            let v = stream.read_u32_le()?;
                            self.minor_version = v & 0xFF;
                            self.supported_functions = v >> 8;
                        }
                        1 => {
                            let bytes = stream.read_bytes(16)?;
                            self.base.connection_signature.copy_from_slice(&bytes[..16]);
                        }
                        3 => {
                            self.initial_unreliable_sequence_id = stream.read_u16_le()?;
                        }
                        4 => {
                            self.maximum_substream_id = stream.read_u8()?;
                        }
                        _ => {
                            // Skip unknown option by consuming remaining block if size is trustworthy
                            // Here _size isn't used to skip; since IDs are known we ignore others.
                        }
                    }
                }
                x if x == PRUDPPacketType::Data as u16 => {
                    if option_id == 2 {
                        self.base.fragment_id = stream.read_u8()?;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn encode_header(&mut self) -> bytes::Bytes {
        self.payload_length = self.base.payload.len() as u16;
        let mut out = ByteStreamOut::new();
        out.put_u8(1); // version
        out.put_u8(self.options_length);
        out.put_u16_le(self.payload_length);
        out.put_u8(self.base.source_virtual_port);
        out.put_u8(self.base.destination_virtual_port);
        let type_and_flags = (self.base.flags << 4) | (self.base.packet_type & 0xF);
        out.put_u16_le(type_and_flags);
        out.put_u8(self.base.session_id);
        out.put_u8(self.base.substream_id);
        out.put_u16_le(self.base.sequence_id);
        out.into_bytes()
    }

    fn encode_options(&self) -> bytes::Bytes {
        let mut out = ByteStreamOut::new();
        if self.base.packet_type == PRUDPPacketType::Syn as u16 || self.base.packet_type == PRUDPPacketType::Connect as u16 {
            // option 0: minor + supported
            out.put_u8(0);
            out.put_u8(4);
            let v = (self.minor_version & 0xFF) | (self.supported_functions << 8);
            out.put_u32_le(v);

            // option 1: connection signature
            out.put_u8(1);
            out.put_u8(16);
            for b in &self.base.connection_signature { out.put_u8(*b); }

            // if Connect: option 3 then 4 (NintendoClient order)
            if self.base.packet_type == PRUDPPacketType::Connect as u16 {
                out.put_u8(3);
                out.put_u8(2);
                out.put_u16_le(self.initial_unreliable_sequence_id);

                out.put_u8(4);
                out.put_u8(1);
                out.put_u8(self.maximum_substream_id);
            } else {
                // For Syn, still include 4 (max substream) as in Go code path
                out.put_u8(4);
                out.put_u8(1);
                out.put_u8(self.maximum_substream_id);
            }
        }
        if self.base.packet_type == PRUDPPacketType::Data as u16 {
            out.put_u8(2);
            out.put_u8(1);
            out.put_u8(self.base.fragment_id);
        }
        out.into_bytes()
    }

    /// Serialize to bytes (header, signature, options, payload with magic).
    pub fn to_bytes(&mut self) -> bytes::Bytes {
        let options = self.encode_options();
        self.options_length = options.len() as u8;
        let header = self.encode_header();

        let mut out = ByteStreamOut::new();
        // magic
        out.put_u8(0xEA);
        out.put_u8(0xD0);
        // header
        for b in header { out.put_u8(b); }
        // signature (16)
        for b in &self.base.signature { out.put_u8(*b); }
        // options
        for b in options { out.put_u8(b); }
        // payload
        for b in &self.base.payload { out.put_u8(*b); }

        out.into_bytes()
    }

    /// Compute connection signature using settings and remote addr.
    pub fn compute_connection_signature(&mut self, settings: &PRUDPV1Settings, addr: std::net::SocketAddr) {
        self.base.connection_signature = settings.default_connection_signature(addr);
    }

    /// Compute packet signature per default rules.
    pub fn compute_signature(&mut self, settings: &PRUDPV1Settings, session_key: &[u8]) {
        let options = self.encode_options();
        self.options_length = options.len() as u8;
        let header = self.encode_header();

        // For Connect packets with legacy flag, omit connection signature
        let include_conn_sig = !(self.base.packet_type == PRUDPPacketType::Connect as u16 && settings.legacy_connection_signature);
        let conn_sig_opt = if include_conn_sig { Some(&self.base.connection_signature[..]) } else { None };
        self.base.signature = settings.default_signature(&header, &options, &self.base.payload, session_key, conn_sig_opt, include_conn_sig);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{SocketAddr, Ipv4Addr};

    #[test]
    fn roundtrip_header_v1() {
        let mut pkt = PRUDPPacketV1::default();
        pkt.base.source_virtual_port = 1;
        pkt.base.destination_virtual_port = 2;
        pkt.base.packet_type = PRUDPPacketType::Syn as u16;
        pkt.base.flags = 0x12;
        pkt.base.session_id = 7;
        pkt.base.substream_id = 3;
        pkt.base.sequence_id = 0xBEEF;
        pkt.base.payload = vec![1,2,3,4];
        pkt.maximum_substream_id = 8;
        pkt.minor_version = 1;
        pkt.supported_functions = 0;

        let mut settings = PRUDPV1Settings::default();
        settings.connection_signature_key = vec![0x11;16];
        pkt.compute_connection_signature(&settings, SocketAddr::from((Ipv4Addr::new(127,0,0,1), 12345)));
        pkt.base.signature = [0xAA;16];

        let bytes = pkt.to_bytes();
        let decoded = PRUDPPacketV1::decode(ByteStreamIn::from_bytes(bytes)).unwrap();
        assert_eq!(decoded.base.version, 1);
        assert_eq!(decoded.base.source_virtual_port, 1);
        assert_eq!(decoded.base.destination_virtual_port, 2);
        assert_eq!(decoded.base.packet_type, PRUDPPacketType::Syn as u16);
        assert_eq!(decoded.base.flags, 0x12);
        assert_eq!(decoded.base.session_id, 7);
        assert_eq!(decoded.base.substream_id, 3);
        assert_eq!(decoded.base.sequence_id, 0xBEEF);
        assert_eq!(decoded.base.payload, vec![1,2,3,4]);
        assert_eq!(decoded.maximum_substream_id, 8);
    }
}
