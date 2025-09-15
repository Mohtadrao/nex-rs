
#[cfg(test)]
mod multi_ack_tests {
    use crate::prudp::packet::{PRUDPPacket, PRUDPHeader, Flags, Version};
    use bytes::BytesMut;
    use bytes::BufMut;
    use bytes::Bytes;

    #[test]
    fn test_multi_ack_mask_roundtrip() {
        // Construct an ACK packet with MULTI_ACK mask set for bits 0..3
        let header = PRUDPHeader {
            version: Version::V0,
            src: 5,
            dst: 6,
            flags: Flags::ACK | Flags::MULTI_ACK,
            session_id: 9,
            seq: 100,
            payload_len: Some(0),
            signature: None,
            multi_ack_mask: Some(0b1111),
        };
        let pkt = PRUDPPacket { header, payload: Bytes::from(vec![]) };
        let b = pkt.to_bytes();
        let parsed = PRUDPPacket::parse(b).expect("parse ok");
        assert!(parsed.header.flags.contains(Flags::MULTI_ACK));
        assert_eq!(parsed.header.multi_ack_mask.unwrap(), 0b1111);
    }
}
