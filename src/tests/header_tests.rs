
#[cfg(test)]
mod header_tests {
    use crate::prudp::packet::{PRUDPHeader, PRUDPPacket, Flags, Version};
    use bytes::Bytes;

    #[test]
    fn test_header_parse_with_optional_fields() {
        // build header manually: version(0), src,dst,flags(HAS_SIZE|SIGNED), session, seq, size(3), signature(4)
        let mut b = vec![];
        b.push(0u8); b.push(1u8); b.push(2u8); b.push((Flags::HAS_SIZE | Flags::SIGNED).bits()); b.push(7u8);
        b.extend_from_slice(&42u16.to_le_bytes());
        b.extend_from_slice(&3u16.to_le_bytes());
        b.extend_from_slice(&[1,2,3,4]);
        // payload bytes
        b.extend_from_slice(&[9,9,9]);
        let bytes = Bytes::from(b);
        let pkt = PRUDPPacket::parse(bytes).expect(\"parse ok\");
        assert_eq!(pkt.header.seq, 42u16);
        assert_eq!(pkt.payload.len(), 3);
    }
}
