
use std::collections::BTreeMap;
use bytes::Bytes;
use crate::prudp::packet::PRUDPPacket;
use anyhow::Result;

/// PacketDispatchQueue sequences incoming PRUDP packets and yields them in order.
/// It stores packets keyed by sequence id and only returns the next contiguous packet.
pub struct PacketDispatchQueue {
    queue: BTreeMap<u16, PRUDPPacket>,
    pub next_expected_sequence_id: u16,
}

impl PacketDispatchQueue {
    pub fn new(start: u16) -> Self {
        Self {
            queue: BTreeMap::new(),
            next_expected_sequence_id: start,
        }
    }

    /// Queue an incoming packet. Duplicates (same seq) are ignored.
    pub fn queue_packet(&mut self, pkt: PRUDPPacket) {
        let seq = pkt.header.sequence_id;
        if seq < self.next_expected_sequence_id {
            // old/duplicate
            return;
        }
        self.queue.entry(seq).or_insert(pkt);
    }

    /// Get the next packet ready to be dispatched, if available.
    /// Does not remove it from the queue; use `dispatched` after processing.
    pub fn get_next_to_dispatch(&self) -> Option<PRUDPPacket> {
        if let Some(p) = self.queue.get(&self.next_expected_sequence_id) {
            return Some(p.clone());
        }
        None
    }

    /// Mark a packet as dispatched (removes it and advances expected sequence)
    pub fn dispatched(&mut self, pkt: &PRUDPPacket) {
        let seq = pkt.header.sequence_id;
        if self.queue.remove(&seq).is_some() {
            // advance until next missing
            loop {
                self.next_expected_sequence_id = self.next_expected_sequence_id.wrapping_add(1);
                if !self.queue.contains_key(&self.next_expected_sequence_id) {
                    break;
                }
            }
        }
    }

    /// Purge old entries (optional)
    pub fn purge_older_than(&mut self, threshold_seq: u16) {
        let keys: Vec<u16> = self.queue.keys().copied().collect();
        for k in keys {
            if k < threshold_seq {
                self.queue.remove(&k);
            }
        }
    }
}
