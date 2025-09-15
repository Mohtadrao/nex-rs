
#[cfg(test)]
mod sliding_nak_tests {
    use crate::prudp::sliding::SlidingWindow;

    #[test]
    fn test_nak_mask_generation() {
        let mut w = SlidingWindow::new(16);
        // receive seq 1 and 3
        let (_, n1) = w.offer_receive(1, vec![1]);
        assert!(n1.is_none());
        let (_, n2) = w.offer_receive(3, vec![3]);
        // missing 2 should cause a NAK suggestion
        assert!(n2.is_some());
        if let Some((base, mask)) = n2 {
            // base should be the current recv_base which is 2 after delivering 1
            assert_eq!(base, 2);
            // mask should include bit 0 (seq 2) and bit 1 (seq 3 is present, so not set) - because we consider holes
            // In our implementation, we mark missing positions; ensure mask is a u32.
            assert!(mask > 0);
        }
    }
}
