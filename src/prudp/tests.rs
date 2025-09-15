
#[cfg(test)]
mod tests {
    use super::*;
    use crate::prudp::packet::PRUDPHeader;
    use bytes::Bytes;

    #[tokio::test]
    async fn test_fragment_send_and_reassemble_flow() {
        // simulate fragmentation and reassembly using fragment_into_packets and Reassembler
        let payload = b"Hello world this is a longer payload to fragment".to_vec();
        let header = PRUDPHeader { version:1, packet_type: 2, flags: crate::prudp::packet::Flags::RELIABLE, session_id: 1, sequence_id: 0, ack_id:0, payload_len: None };
        let packets = crate::prudp::fragmentation::fragment_into_packets(&payload, 10, &header).unwrap();
        assert!(packets.len() > 1);
        let mut re = crate::prudp::fragmentation::Reassembler::new(std::time::Duration::from_secs(5));
        for p in &packets {
            let out = re.offer(p);
            if out.is_some() {
                assert_eq!(out.unwrap(), payload);
                return;
            }
        }
        panic!("reassembly didn't complete");
    }
}
