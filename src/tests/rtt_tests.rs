
#[cfg(test)]
mod rtt_tests {
    use crate::rtt::RTT;
    use std::time::Duration;

    #[test]
    fn test_estimate_retry_timeout() {
        let r = RTT::new();
        // update with a sample 100ms
        r.update(Duration::from_millis(100));
        let to = r.estimate_retry_timeout();
        assert!(to.as_millis() >= 50);
    }
}
