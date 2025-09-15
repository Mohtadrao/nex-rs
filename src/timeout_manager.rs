use std::{future::Future, time::Duration};
use tokio::{task::JoinHandle, time};

pub struct TimeoutManager;

impl TimeoutManager {
    pub fn new() -> Self { Self }

    pub async fn with_timeout<F, T>(&self, dur: Duration, fut: F) -> crate::Result<T>
    where
        F: Future<Output = T>,
    {
        match time::timeout(dur, fut).await {
            Ok(v) => Ok(v),
            Err(_) => Err(crate::error::Error::Timeout),
        }
    }

    pub fn spawn<F>(fut: F) -> JoinHandle<()>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        tokio::spawn(fut)
    }
}