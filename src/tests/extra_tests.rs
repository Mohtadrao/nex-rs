
#[cfg(test)]
mod extra_tests {
    use crate::prudp::sliding::SlidingWindow;
    use crate::prudp::packet_dispatch_queue::{PacketDispatchQueue, DispatchItem};
    use std::time::Instant;

    #[test]
    fn test_sliding_wraparound() {
        let mut w = SlidingWindow::new(8);
        // simulate sequence numbers near u16 wrap (use high numbers)
        w.recv_base = 65530u16;
        // receive 65530 and 65532 (missing 65531)
        let (_, n1) = w.offer_receive(65530, vec![1]);
        assert!(n1.is_none());
        let (_, n2) = w.offer_receive(65532, vec![3]);
        // Expect a NAK for 65531 (base should be 65531)
        assert!(n2.is_some());
        if let Some((base, mask)) = n2 {
            assert_eq!(base, 65531u16);
            assert!(mask != 0);
        }
    }

    #[test]
    fn test_pdq_starvation_eviction() {
        let mut pdq = PacketDispatchQueue::new(vec![1,2,3], 3);
        // Fill with lower priority items
        pdq.push(DispatchItem { priority: 3, data: vec![1], created: Instant::now() });
        pdq.push(DispatchItem { priority: 3, data: vec![2], created: Instant::now() });
        pdq.push(DispatchItem { priority: 3, data: vec![3], created: Instant::now() });
        // Push a high priority item; eviction should remove one low-priority to make space
        pdq.push(DispatchItem { priority: 1, data: vec![9], created: Instant::now() });
        assert_eq!(pdq.total_len(), 3);
        // Ensure highest priority is returned first
        let it = pdq.pop().unwrap();
        assert_eq!(it.priority, 1);
    }
}
