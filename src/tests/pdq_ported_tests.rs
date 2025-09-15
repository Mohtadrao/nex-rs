
#[cfg(test)]
mod pdq_ported_tests {
    use crate::prudp::packet_dispatch_queue::{PacketDispatchQueue, DispatchItem};
    use std::time::Instant;

    #[test]
    fn test_pdq_eviction_behavior() {
        let mut pdq = PacketDispatchQueue::new(vec![1,2,3], 2); // max size 2
        pdq.push(DispatchItem { priority: 2, data: vec![1], created: Instant::now() });
        pdq.push(DispatchItem { priority: 3, data: vec![2], created: Instant::now() });
        // pushing another should evict lowest priority (3)
        pdq.push(DispatchItem { priority: 1, data: vec![3], created: Instant::now() });
        assert_eq!(pdq.total_len(), 2);
        // pop should return priority 1 then 2
        let a = pdq.pop().unwrap(); assert_eq!(a.priority, 1);
        let b = pdq.pop().unwrap(); assert_eq!(b.priority, 2);
    }
}
