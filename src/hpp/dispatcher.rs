
use bytes::Bytes;
use async_trait::async_trait;
use crate::Result;
use tracing::info;
use crate::hpp::packet::HppPacket;
use crate::hpp::rmc::RmcMessage;
use std::sync::Arc;
use std::collections::HashMap;
use parking_lot::Mutex;

#[async_trait]
pub trait HppHandler: Send + Sync + 'static {
    async fn on_hpp_message(&self, data: Bytes) -> Result<Option<Bytes>>;
    async fn handle_method(&self, method_id: u32, body: Bytes) -> Result<Bytes> {
        // default: echo back empty response
        Ok(Bytes::from(vec![]))
    }
}

pub struct SimpleHandler {
    // method registry: method_id -> handler boxed fn
    registry: Mutex<HashMap<u32, Arc<dyn Fn(Bytes) -> futures::future::BoxFuture<'static, Result<Bytes>> + Send + Sync>>>,
}

impl SimpleHandler {
    pub fn new() -> Self {
        Self { registry: Mutex::new(HashMap::new()) }
    }

    pub fn register<F, Fut>(&self, method_id: u32, f: F) 
    where F: Fn(Bytes) -> Fut + Send + Sync + 'static, Fut: std::future::Future<Output = Result<Bytes>> + Send + 'static {
        let boxed = Arc::new(move |b: Bytes| -> futures::future::BoxFuture<'static, Result<Bytes>> {
            Box::pin(f(b))
        }) as Arc<dyn Fn(Bytes) -> futures::future::BoxFuture<'static, Result<Bytes>> + Send + Sync>;
        self.registry.lock().insert(method_id, boxed);
    }
}

#[async_trait]
impl HppHandler for SimpleHandler {
    async fn on_hpp_message(&self, data: Bytes) -> Result<Option<Bytes>> {
        // Try to parse HPP frame and RMC message
        let res = HppPacket::parse(data.clone().into()).and_then(|p| RmcMessage::parse(p.payload).map_err(|e| e.into()));
        match res {
            Ok(rmc) => {
                info!(method = rmc.method_id, "Received RMC message via HPP");
                // dispatch to registered method if exists
                if let Some(cb) = self.registry.lock().get(&rmc.method_id) {
                    let cb = cb.clone();
                    let resp = cb(rmc.body).await?;
                    info!(resp_len = resp.len(), "Method handler produced response");
                    return Ok(Some(resp));
                } else {
                    info!("No handler registered for method {}", rmc.method_id);
                }
            }
            Err(e) => {
                info!(len = data.len(), "HPP SimpleHandler received raw message: {:?}", e);
            }
        }
        Ok(None)
    }
}
