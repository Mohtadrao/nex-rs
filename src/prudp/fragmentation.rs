
use crate::prudp::packet::{PRUDPPacket, PRUDPHeader};
use bytes::Bytes;
use anyhow::Result;
use rand::Rng;

pub fn fragment_into_packets(payload: &[u8], mtu: usize, base_header: &PRUDPHeader) -> Result<Vec<PRUDPPacket>> {
    let frags = if mtu == 0 { vec![payload.to_vec()] } else {
        let mut out = Vec::new();
        let mut i = 0usize;
        while i < payload.len() {
            let end = std::cmp::min(i + mtu, payload.len());
            out.push(payload[i..end].to_vec());
            i = end;
        }
        out
    };

    let mut packets = Vec::new();
    let fragment_id: u32 = rand::thread_rng().gen();
    let count = frags.len() as u16;
    for (idx, frag) in frags.into_iter().enumerate() {
        let mut header = base_header.clone();
        header.payload_len = Some(frag.len());
        let pkt = PRUDPPacket {
            header,
            payload: Bytes::from(frag),
            fragment_id: Some(fragment_id),
            fragment_index: Some(idx as u16),
            fragment_count: Some(count),
        };
        packets.push(pkt);
    }
    Ok(packets)
}

use std::collections::HashMap;
use std::time::{Instant, Duration};

pub struct ReassemblyState {
    pub fragment_count: u16,
    pub received: Vec<Option<Vec<u8>>>,
    pub first_seen: Instant,
    pub timeout: Duration,
}

impl ReassemblyState {
    pub fn new(count: u16, timeout: Duration) -> Self {
        Self {
            fragment_count: count,
            received: vec![None; count as usize],
            first_seen: Instant::now(),
            timeout,
        }
    }

    pub fn insert(&mut self, index: usize, data: Vec<u8>) {
        if index < self.received.len() {
            self.received[index] = Some(data);
        }
    }

    pub fn is_complete(&self) -> bool {
        self.received.iter().all(|x| x.is_some())
    }

    pub fn reassemble(&self) -> Vec<u8> {
        let mut out = Vec::new();
        for part in &self.received {
            if let Some(p) = part { out.extend_from_slice(p); }
        }
        out
    }
}

pub struct Reassembler {
    pub map: HashMap<u32, ReassemblyState>,
    pub timeout: Duration,
}

impl Reassembler {
    pub fn new(timeout: Duration) -> Self {
        Self { map: HashMap::new(), timeout }
    }

    pub fn offer(&mut self, pkt: &PRUDPPacket) -> Option<Vec<u8>> {
        if let (Some(fid), Some(idx), Some(count)) = (pkt.fragment_id, pkt.fragment_index, pkt.fragment_count) {
            let state = self.map.entry(fid).or_insert_with(|| ReassemblyState::new(count, self.timeout));
            state.insert(idx as usize, pkt.payload.to_vec());
            if state.is_complete() {
                let data = state.reassemble();
                self.map.remove(&fid);
                return Some(data);
            }
        }
        None
    }

    pub fn purge_timeouts(&mut self) {
        let now = Instant::now();
        let keys: Vec<u32> = self.map.iter()
            .filter_map(|(k, s)| if now.duration_since(s.first_seen) > s.timeout { Some(*k) } else { None })
            .collect();
        for k in keys { self.map.remove(&k); }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::prudp::packet::PRUDPHeader;
    use bytes::Bytes;

    #[test]
    fn test_fragment_reassemble_roundtrip() {
        let payload = b"The quick brown fox jumps over the lazy dog".to_vec();
        let header = PRUDPHeader { version:1, packet_type: 2, flags: crate::prudp::packet::Flags::RELIABLE, session_id: 1, sequence_id: 1, ack_id:0, payload_len: None };
        let packets = fragment_into_packets(&payload, 10, &header).unwrap();
        assert!(packets.len() > 1);
        let mut re = Reassembler::new(std::time::Duration::from_secs(5));
        let mut got = None;
        for p in &packets {
            if let Some(data) = re.offer(p) {
                got = Some(data);
            }
        }
        assert!(got.is_some());
        assert_eq!(got.unwrap(), payload);
    }
}
