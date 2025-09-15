
#[cfg(test)]
mod integration {
    use super::*;
    use crate::prudp::fragmentation::Reassembler;
    use crate::prudp::fragmentation::fragment_into_packets;
    use crate::prudp::packet::PRUDPHeader;
    use bytes::Bytes;

    #[test]
    fn integration_fragment_reassembly() {
        let payload = b"integration test payload to fragment".to_vec();
        let header = PRUDPHeader { version:1, packet_type: 2, flags: crate::prudp::packet::Flags::RELIABLE, session_id: 1, sequence_id: 0, ack_id:0, payload_len: None };
        let packets = fragment_into_packets(&payload, 16, &header).unwrap();
        let mut r = Reassembler::new(std::time::Duration::from_secs(5));
        for p in &packets {
            if let Some(data) = r.offer(p) {
                assert_eq!(data, payload);
                return;
            }
        }
        panic!("did not reassemble");
    }
}
