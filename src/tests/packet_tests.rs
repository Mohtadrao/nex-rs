
#[cfg(test)]
mod packet_tests {
    use crate::prudp::packet::{PRUDPPacket, PRUDPHeader, Flags, Version};
    use bytes::BytesMut;
    use bytes::BufMut;

    #[test]
    fn test_packet_encode_parse_roundtrip() {
        let header = PRUDPHeader {
            version: Version::V0,
            src: 1,
            dst: 2,
            flags: Flags::HAS_SIZE | Flags::SIGNED,
            session_id: 7,
            seq: 42,
            payload_len: Some(3),
            signature: Some([1,2,3,4]),
            multi_ack_mask: None,
        };
        let payload = bytes::Bytes::from_static(&[9,9,9]);
        let pkt = PRUDPPacket { header, payload };
        let b = pkt.to_bytes();
        let parsed = PRUDPPacket::parse(b).expect(\"parse ok\");
        assert_eq!(parsed.header.version as u8, 0u8);
        assert_eq!(parsed.header.src, 1);
        assert_eq!(parsed.header.dst, 2);
        assert!(parsed.header.flags.contains(Flags::HAS_SIZE));
        assert!(parsed.header.flags.contains(Flags::SIGNED));
        assert_eq!(parsed.payload.len(), 3);
    }
}
