
#[cfg(test)]
mod tests {
    use crate::rtt::RTT;
    use std::time::Duration;

    #[test]
    fn test_rtt_update_and_rto() {
        let r = RTT::new();
        assert!(!r.initialized());
        let (avg, var) = r.update(Duration::from_millis(100));
        assert!(r.initialized());
        assert!(avg > 0.0);
        let rto = r.estimate_retry_timeout();
        assert!(rto.as_millis() >= 50);
    }
}
