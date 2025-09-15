use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug)]
pub struct Counter {
    value: AtomicU64,
}

impl Counter {
    pub fn new(start: u64) -> Self {
        Self { value: AtomicU64::new(start) }
    }
    pub fn get(&self) -> u64 {
        self.value.load(Ordering::Relaxed)
    }
    pub fn inc(&self) -> u64 {
        self.value.fetch_add(1, Ordering::Relaxed) + 1
    }
    pub fn dec(&self) -> u64 {
        self.value.fetch_sub(1, Ordering::Relaxed) - 1
    }
    pub fn set(&self, v: u64) {
        self.value.store(v, Ordering::Relaxed);
    }
}