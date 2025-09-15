
#[cfg(test)]
mod packet_exact_tests {
    use crate::prudp::packet::{PRUDPPacket, PRUDPHeader, Version, Flags};
    use bytes::Bytes;
    use bytes::BytesMut;
    use bytes::BufMut;

    #[test]
    fn test_prudp_v0_exact_bytes() {
        // Construct expected bytes for v0: ver(0), src(1), dst(2), flags(HAS_SIZE|SIGNED), sess(7), seq(0x2A), size(3), sig(1,2,3,4), payload(9,9,9)
        let mut b = BytesMut::new();
        b.put_u8(0u8); // version
        b.put_u8(1u8); // src
        b.put_u8(2u8); // dst
        b.put_u8((Flags::HAS_SIZE | Flags::SIGNED).bits());
        b.put_u8(7u8); // session
        b.put_u16_le(42u16); // seq
        b.put_u16_le(3u16); // size
        b.extend_from_slice(&[1u8,2u8,3u8,4u8]); // signature
        b.extend_from_slice(&[9u8,9u8,9u8]); // payload

        let bytes = b.freeze();
        let pkt = PRUDPPacket::parse(bytes.clone()).expect("parse v0");
        assert_eq!(pkt.header.version as u8, 0u8);
        assert_eq!(pkt.payload.len(), 3);
        // Roundtrip encode
        let out = pkt.to_bytes();
        assert_eq!(&out[..], &bytes[..]);
    }

    #[test]
    fn test_prudp_v1_exact_bytes() {
        // v1: same as v0 but with v1_extra after signature and before payload
        let mut b = BytesMut::new();
        b.put_u8(1u8); // version v1
        b.put_u8(1u8); // src
        b.put_u8(2u8); // dst
        b.put_u8((Flags::HAS_SIZE | Flags::SIGNED).bits());
        b.put_u8(7u8); // session
        b.put_u16_le(100u16); // seq
        b.put_u16_le(3u16); // size
        b.extend_from_slice(&[5u8,6u8,7u8,8u8]); // signature
        b.put_u32_le(0xdeadbeefu32); // v1_extra
        b.extend_from_slice(&[9u8,9u8,9u8]); // payload

        let bytes = b.freeze();
        let pkt = PRUDPPacket::parse(bytes.clone()).expect("parse v1");
        assert_eq!(pkt.header.version as u8, 1u8);
        assert_eq!(pkt.header.v1_extra.unwrap(), 0xdeadbeefu32);
        let out = pkt.to_bytes();
        assert_eq!(&out[..], &bytes[..]);
    }
}
