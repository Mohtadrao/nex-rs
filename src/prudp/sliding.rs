
use std::collections::{BTreeMap, HashMap};
use std::time::{Instant, Duration};
use bytes::Bytes;
use anyhow::Result;
use parking_lot::Mutex;
use std::sync::Arc;

/// Information about a sent packet awaiting ACK
#[derive(Debug, Clone)]
pub struct SentEntry {
    pub seq: u16,
    pub payload: Bytes,
    pub sent_at: Instant,
    pub attempts: u8,
}

/// Receive-side state for a single logical stream.
#[derive(Debug)]
pub struct RecvState {
    pub recv_base: u16,
    pub recv_buffer: BTreeMap<u16, Bytes>,
    pub seen: HashMap<u16, Instant>,
}

impl RecvState {
    pub fn new(start: u16) -> Self {
        Self {
            recv_base: start,
            recv_buffer: BTreeMap::new(),
            seen: HashMap::new(),
        }
    }

    /// Offer a received sequence number and payload to the receive buffer.
    /// Returns Some((nak_base, nak_mask)) when a NAK should be generated (i.e., there's a gap),
    /// otherwise None.
    pub fn offer(&mut self, seq: u16, payload: Bytes) -> Option<(u16, u32)> {
        // If sequence is less than base, ignore (duplicate/old)
        if seq < self.recv_base {
            return None;
        }

        // Insert into buffer if not present
        self.recv_buffer.entry(seq).or_insert(payload);

        // record seen timestamp
        self.seen.insert(seq, Instant::now());

        // If we've just filled the next sequence, advance base while possible
        if self.recv_buffer.contains_key(&self.recv_base) {
            while let Some(_) = self.recv_buffer.remove(&self.recv_base) {
                self.recv_base = self.recv_base.wrapping_add(1);
            }
            return None;
        }

        // There's a gap: compute NAK base and mask covering up to 32 following sequences
        let base = self.recv_base.wrapping_add(1);
        let mut mask: u32 = 0;
        for i in 0..32u16 {
            let s = base.wrapping_add(i);
            if self.recv_buffer.contains_key(&s) {
                mask |= 1u32 << i;
            }
        }
        Some((base, mask))
    }
}

/// Sliding window used for send-side reliability and receive buffering.
#[derive(Debug)]
pub struct SlidingWindow {
    pub window_size: u16,
    pub send_base: u16,
    pub next_seq: u16,
    /// sent buffer maps sequence -> SentEntry
    pub sent_buffer: HashMap<u16, SentEntry>,
    /// Receive states per stream (stream id -> RecvState)
    pub recv_states: HashMap<u16, RecvState>,
    /// Retransmit timeout
    pub rto: Duration,
    /// Maximum retransmit attempts before giving up
    pub max_attempts: u8,
}

impl SlidingWindow {
    /// Compatibility: get next sequence (allocates) - panics if cannot send
    pub fn next_sequence(&mut self) -> u16 {
        self.allocate_sequence().expect("no sequence available")
    }

    /// Compatibility: mark a seq as sent (update timestamp)
    pub fn mark_sent(&mut self, seq: u16) {
        if let Some(e) = self.sent_buffer.get_mut(&seq) {
            e.sent_at = std::time::Instant::now();
        }
    }

    /// Return current retry timeout
    pub fn current_retry_timeout(&self) -> Duration { self.rto }

    /// Attach an RTT estimator (placeholder)
    pub fn attach_rtt(&mut self, _rtt: std::sync::Arc<crate::rtt::RTT>) { /* store if needed */ }

    pub fn new(window_size: u16) -> Self {
        Self {
            window_size,
            send_base: 0,
            next_seq: 1, // usually start at 1
            sent_buffer: HashMap::new(),
            recv_states: HashMap::new(),
            rto: Duration::from_millis(500),
            max_attempts: 5,
        }
    }

    /// Can we send a new reliable packet?
    pub fn can_send(&self) -> bool {
        let in_flight = self.next_seq.wrapping_sub(self.send_base) as u16;
        in_flight < self.window_size
    }

    /// Allocate next sequence number for sending (does NOT insert into sent_buffer).
    pub fn allocate_sequence(&mut self) -> Option<u16> {
        if !self.can_send() {
            return None;
        }
        let seq = self.next_seq;
        self.next_seq = self.next_seq.wrapping_add(1);
        Some(seq)
    }

    /// Queue a sent packet into the send buffer (mark as sent now).
    pub fn queue_sent(&mut self, seq: u16, payload: Bytes) {
        let entry = SentEntry {
            seq,
            payload,
            sent_at: Instant::now(),
            attempts: 1,
        };
        self.sent_buffer.insert(seq, entry);
    }

    /// Mark a sequence as acknowledged; slide window forward and remove entries.
    pub fn mark_acked(&mut self, ack_seq: u16) {
        // ack_seq acknowledges that all sequences up to ack_seq (inclusive) are received
        // We'll remove from send_base up to ack_seq
        let mut seq = self.send_base;
        while seq != ack_seq.wrapping_add(1) {
            self.sent_buffer.remove(&seq);
            seq = seq.wrapping_add(1);
        }
        self.send_base = ack_seq.wrapping_add(1);
    }

    /// Called when an ACK for a single sequence (or cumulative ack) is received.
    /// If packet is present, remove it from buffer.
    pub fn mark_acked_single(&mut self, seq: u16) {
        if self.sent_buffer.remove(&seq).is_some() {
            // adjust send_base if this was the lowest
            while !self.sent_buffer.contains_key(&self.send_base) && self.send_base != self.next_seq {
                self.send_base = self.send_base.wrapping_add(1);
            }
        }
    }

    /// Returns a list of sequence numbers that should be retransmitted now.
    pub fn retransmit_candidates(&mut self) -> Vec<u16> {
        let now = Instant::now();
        let mut out = Vec::new();
        let keys: Vec<u16> = self.sent_buffer.keys().copied().collect();
        for seq in keys {
            if let Some(entry) = self.sent_buffer.get_mut(&seq) {
                if entry.attempts >= self.max_attempts { continue; }
                if now.duration_since(entry.sent_at) >= self.rto {
                    entry.attempts = entry.attempts.saturating_add(1);
                    entry.sent_at = Instant::now();
                    out.push(seq);
                }
            }
        }
        out
    }

    /// Get payload for a sequence if present
    pub fn get_payload(&self, seq: u16) -> Option<Bytes> {
        self.sent_buffer.get(&seq).map(|e| e.payload.clone())
    }

    /// Offer a packet to a specified stream (stream_id optional). Returns optional NAK info.
    pub fn offer_receive_stream(&mut self, stream_id: Option<u16>, seq: u16, payload: Vec<u8>) -> (Option<Bytes>, Option<(u16,u32)>) {
        let sid = stream_id.unwrap_or(0);
        let state = self.recv_states.entry(sid).or_insert_with(|| RecvState::new(1));
        let bytes = Bytes::from(payload);
        let nak = state.offer(seq, bytes.clone());
        // If next expected is available, return it for delivery (not implemented full extraction here)
        // For simplicity, return None as delivery happens elsewhere
        (None, nak)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

    #[test]
    fn test_basic_send_flow() {
        let mut w = SlidingWindow::new(8);
        assert!(w.can_send());
        let seq = w.allocate_sequence().unwrap();
        assert_eq!(seq, 1);
        w.queue_sent(seq, Bytes::from(vec![1,2,3]));
        // nothing to retransmit immediately
        assert!(w.retransmit_candidates().is_empty());
    }

    #[test]
    fn test_receive_nak() {
        let mut w = SlidingWindow::new(8);
        let (_, n1) = w.offer_receive_stream(Some(0), 1, vec![1]);
        assert!(n1.is_none());
        let (_, n2) = w.offer_receive_stream(Some(0), 3, vec![3]);
        assert!(n2.is_some());
        if let Some((base, mask)) = n2 {
            assert_eq!(base, 2);
        }
    }
}
