
#[cfg(test)]
mod hpp_rmc_tests {
    use crate::hpp::packet::HppPacket;
    use crate::hpp::rmc::RmcMessage;
    use bytes::Bytes;

    #[test]
    fn test_hpp_rmc_roundtrip() {
        // build RMC body: method id (4 bytes little endian) + payload
        let mut b = vec![];
        b.extend_from_slice(&123u32.to_le_bytes());
        b.extend_from_slice(&[9,9,9]);
        let payload = Bytes::from(b);
        let framed = HppPacket::encode(&payload);
        let parsed = HppPacket::parse(framed.clone()).expect("hpp parse");
        let rmc = RmcMessage::parse(parsed.payload).expect("rmc parse");
        assert_eq!(rmc.method_id, 123u32);
        assert_eq!(rmc.body.len(), 3);
    }
}
