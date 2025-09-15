
#[cfg(test)]
mod sliding_port_tests {
    use crate::prudp::sliding::SlidingWindow;
    use crate::prudp::packet::PRUDPPacket;
    use crate::prudp::packet::PRUDPHeader;
    use crate::prudp::packet::{Version, Flags};
    use bytes::Bytes;
    use std::time::{Duration, Instant};

    #[test]
    fn test_nak_suppression_and_backoff() {
        let mut w = SlidingWindow::new(8);
        // first: receive 1 then 3 -> NAK for 2
        let (_, n1) = w.offer_receive_stream(Some(0), 1, vec![1]);
        assert!(n1.is_none());
        let (_, n2) = w.offer_receive_stream(Some(0), 3, vec![3]);
        assert!(n2.is_some());
        // immediately receiving 3 again should not produce another NAK due to backoff (can_nak)
        let (_, n3) = w.offer_receive_stream(Some(0), 3, vec![3]);
        assert!(n3.is_none());
    }

    #[test]
    fn test_multi_ack_clears_pending() {
        let mut w = SlidingWindow::new(16);
        // simulate pending entries 5..10
        for s in 5u16..10u16 {
            w.pending.insert(s, Instant::now() - Duration::from_secs(10));
        }
        // multi-ack starting at 5 with mask bits 0..3 -> clears 5,6,7,8
        w.mark_acked_range(5, 0b0000_0000_0000_0000_0000_0000_0000_1111);
        assert!(!w.pending.contains_key(&5));
        assert!(!w.pending.contains_key(&6));
        assert!(!w.pending.contains_key(&7));
        assert!(!w.pending.contains_key(&8));
        // 9 should remain
        assert!(w.pending.contains_key(&9));
    }

    #[test]
    fn test_duplicate_suppression_ttl() {
        let mut w = SlidingWindow::new(8);
        // receive seq 1
        let (_, n1) = w.offer_receive_stream(Some(0), 1, vec![1]);
        assert!(n1.is_none());
        // duplicate 1 should be suppressed (no ready)
        let (ready, _) = w.offer_receive_stream(Some(0), 1, vec![1]);
        assert!(ready.is_empty());
    }
}
