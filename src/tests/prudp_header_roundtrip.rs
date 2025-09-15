
#[cfg(test)]
mod prudp_header_roundtrip {
    use crate::prudp::packet::{PRUDPHeader, PRUDPPacket, Flags, Version};
    use bytes::Bytes;
    use bytes::BytesMut;

    fn roundtrip_header(header: &PRUDPHeader, payload: &[u8]) {
        let pkt = PRUDPPacket { header: header.clone(), payload: Bytes::copy_from_slice(payload) };
        let bytes = pkt.to_bytes();
        let parsed = PRUDPPacket::parse(bytes).expect("parse ok");
        // Compare core fields
        assert_eq!(parsed.header.version as u8, header.version as u8);
        assert_eq!(parsed.header.src, header.src);
        assert_eq!(parsed.header.dst, header.dst);
        assert_eq!(parsed.header.flags.bits(), header.flags.bits());
        assert_eq!(parsed.header.session_id, header.session_id);
        assert_eq!(parsed.header.seq, header.seq);
        assert_eq!(parsed.header.payload_len, header.payload_len);
        assert_eq!(parsed.header.signature, header.signature);
        assert_eq!(parsed.header.multi_ack_mask, header.multi_ack_mask);
        assert_eq!(parsed.header.v1_extra, header.v1_extra);
        assert_eq!(parsed.header.checksum, header.checksum);
        assert_eq!(parsed.header.stream_id, header.stream_id);
        assert_eq!(&parsed.payload[..], payload);
    }

    #[test]
    fn test_various_headers_v0() {
        // basic no-options
        let h0 = PRUDPHeader {
            version: Version::V0,
            src: 1, dst: 2,
            flags: Flags::empty(),
            session_id: 5,
            seq: 10,
            payload_len: None,
            signature: None,
            multi_ack_mask: None,
            v1_extra: None,
            checksum: None,
            stream_id: None,
        };
        roundtrip_header(&h0, b"hello");

        // has size + signed
        let h1 = PRUDPHeader {
            version: Version::V0,
            src: 3, dst: 4,
            flags: Flags::HAS_SIZE | Flags::SIGNED,
            session_id: 7,
            seq: 42,
            payload_len: Some(3),
            signature: Some([9,8,7,6]),
            multi_ack_mask: None,
            v1_extra: None,
            checksum: None,
            stream_id: None,
        };
        roundtrip_header(&h1, b"abc");

        // multi-ack present
        let h2 = PRUDPHeader {
            version: Version::V0,
            src: 8, dst: 9,
            flags: Flags::ACK | Flags::MULTI_ACK,
            session_id: 1,
            seq: 100,
            payload_len: Some(0),
            signature: None,
            multi_ack_mask: Some(0b1011),
            v1_extra: None,
            checksum: None,
            stream_id: None,
        };
        roundtrip_header(&h2, b"");
    }

    #[test]
    fn test_various_headers_v1() {
        let h0 = PRUDPHeader {
            version: Version::V1,
            src: 1, dst: 2,
            flags: Flags::empty(),
            session_id: 5,
            seq: 11,
            payload_len: None,
            signature: None,
            multi_ack_mask: None,
            v1_extra: Some(0xdeadbeef),
            checksum: None,
            stream_id: None,
        };
        roundtrip_header(&h0, b"payload-v1");

        let h1 = PRUDPHeader {
            version: Version::V1,
            src: 4, dst: 5,
            flags: Flags::HAS_SIZE | Flags::SIGNED,
            session_id: 2,
            seq: 77,
            payload_len: Some(4),
            signature: Some([1,2,3,4]),
            multi_ack_mask: None,
            v1_extra: Some(0x12345678),
            checksum: Some(0xCAFEBABE),
            stream_id: Some(3),
        };
        roundtrip_header(&h1, b"data");
    }
}
