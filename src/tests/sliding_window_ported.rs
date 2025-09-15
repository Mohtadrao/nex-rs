
#[cfg(test)]
mod sliding_window_ported {
    use crate::prudp::sliding::SlidingWindow;
    use crate::prudp::packet::PRUDPPacket;
    use bytes::Bytes;

    #[test]
    fn test_nak_suppression_and_backoff() {
        let mut w = SlidingWindow::new(8);
        // simulate: receive 1 and 3 -> NAK for 2 expected
        let (_, n1) = w.offer_receive_stream(Some(0), 1, vec![1]);
        assert!(n1.is_none());
        let (_, n2) = w.offer_receive_stream(Some(0), 3, vec![3]);
        assert!(n2.is_some());
        // subsequent repeated offers for same missing hole should be suppressed by backoff
        let (_, n3) = w.offer_receive_stream(Some(0), 3, vec![3]);
        assert!(n3.is_none());
    }

    #[test]
    fn test_multi_ack_clears_pending() {
        let mut w = SlidingWindow::new(8);
        // simulate pending sequences 10..14
        for s in 10u16..15u16 {
            w.pending.insert(s, std::time::Instant::now());
        }
        // mark ack range starting at 11 with mask bits 0..2 -> acks 11,12,13
        w.mark_acked_range(11, 0b0000_0000_0000_0000_0000_0000_0000_0111);
        assert!(!w.pending.contains_key(&11));
        assert!(!w.pending.contains_key(&12));
        assert!(!w.pending.contains_key(&13));
        assert!(w.pending.contains_key(&10));
        assert!(w.pending.contains_key(&14));
    }

    #[test]
    fn test_duplicate_suppression_ttl() {
        let mut w = SlidingWindow::new(8);
        // receive seq 5 twice, second should be suppressed
        let (_, n1) = w.offer_receive_stream(Some(0), 5, vec![5]);
        assert!(n1.is_none());
        let (ready, _) = w.offer_receive_stream(Some(0), 5, vec![5]);
        assert!(ready.is_empty());
    }
}
