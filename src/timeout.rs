
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Backoff {
    pub base: Duration,
    pub max: Duration,
    pub factor: f32,
    pub attempt: u32,
}

impl Backoff {
    pub fn new(base: Duration, max: Duration, factor: f32) -> Self {
        Self { base, max, factor, attempt: 0 }
    }
    pub fn next(&mut self) -> Duration {
        let mul = self.factor.powi(self.attempt as i32);
        let dur = self.base.mul_f32(mul);
        self.attempt = self.attempt.saturating_add(1);
        if dur > self.max { self.max } else { dur }
    }
    pub fn reset(&mut self) { self.attempt = 0; }
}
