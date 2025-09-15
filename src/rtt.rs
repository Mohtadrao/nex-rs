
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

pub struct RTT {
    pub smoothed: AtomicU64, // stored as milliseconds
}

impl RTT {
    pub fn new() -> Self { Self { smoothed: AtomicU64::new(200) } }

    pub fn sample_rtt(&self, ms: u64) {
        self.smoothed.store(ms, Ordering::Relaxed);
    }

    pub fn estimate(&self) -> Duration {
        let v = self.smoothed.load(Ordering::Relaxed);
        Duration::from_millis(v)
    }
}
