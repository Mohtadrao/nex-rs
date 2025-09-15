
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct WindowEntry {
    pub seq: u16,
    pub data: Vec<u8>,
    pub last_sent: Instant,
    pub retries: u32,
}

#[derive(Debug)]
pub struct SlidingWindow {
    pub size: usize,
    pub send_base: u16,
    pub next_seq: u16,
    pub rto: Duration,
    pub in_flight: BTreeMap<u16, WindowEntry>,
}

impl SlidingWindow {
    pub fn new(size: usize, initial_seq: u16, rto: Duration) -> Self {
        Self {
            size,
            send_base: initial_seq,
            next_seq: initial_seq,
            rto,
            in_flight: BTreeMap::new(),
        }
    }

    #[inline]
    fn seq_lte(a: u16, b: u16) -> bool {
        // a <= b with wrap-around semantics (half-range rule)
        b.wrapping_sub(a) < 0x8000
    }

    pub fn can_send(&self) -> bool {
        self.in_flight.len() < self.size
    }

    /// Enqueue data into the window and return the sequence assigned.
    pub fn on_send(&mut self, data: Vec<u8>, now: Instant) -> u16 {
        let seq = self.next_seq;
        self.next_seq = self.next_seq.wrapping_add(1);
        self.in_flight.insert(seq, WindowEntry { seq, data, last_sent: now, retries: 0 });
        seq
    }

    /// Mark a sequence as just transmitted (refresh timer).
    pub fn mark_sent(&mut self, seq: u16, now: Instant) {
        if let Some(e) = self.in_flight.get_mut(&seq) {
            e.last_sent = now;
            e.retries = e.retries.saturating_add(1);
        }
    }

    /// Apply cumulative ACK up to and including `ack_seq`.
    pub fn on_ack(&mut self, ack_seq: u16) {
        // Remove everything <= ack_seq with wrap compare
        let keys: Vec<u16> = self.in_flight.keys().copied().collect();
        for k in keys {
            if Self::seq_lte(k, ack_seq) {
                self.in_flight.remove(&k);
            }
        }
        // slide base if needed
        if Self::seq_lte(self.send_base, ack_seq) {
            self.send_base = ack_seq.wrapping_add(1);
        }
    }

    /// Return the list of seqs that should be retransmitted (timeout exceeded).
    pub fn timeouts(&self, now: Instant) -> Vec<u16> {
        self.in_flight
            .iter()
            .filter_map(|(&seq, e)| if now.duration_since(e.last_sent) >= self.rto { Some(seq) } else { None })
            .collect()
    }

    /// Fetch a copy of bytes for a given sequence (for retransmit).
    pub fn get(&self, seq: u16) -> Option<&[u8]> {
        self.in_flight.get(&seq).map(|e| e.data.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_flow() {
        let now = Instant::now();
        let mut win = SlidingWindow::new(4, 100, Duration::from_millis(100));
        assert!(win.can_send());
        let s1 = win.on_send(vec![1], now);
        assert_eq!(s1, 100);
        assert_eq!(win.in_flight.len(), 1);
        win.on_ack(s1);
        assert!(win.in_flight.is_empty());
        assert_eq!(win.send_base, 101);
    }
}
