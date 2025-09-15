
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use futures::channel::oneshot;
use crate::Result;
use bytes::Bytes;

#[derive(Debug)]
pub struct RmcCall {
    pub call_id: u32,
    pub service_id: u16,
    pub method_id: u16,
    pub payload: Bytes,
}

pub struct RmcDispatcher {
    pending: Mutex<HashMap<u32, oneshot::Sender<Result<Bytes>>>>,
    next_call_id: Mutex<u32>,
}

impl RmcDispatcher {
    pub fn new() -> Self {
        Self { pending: Mutex::new(HashMap::new()), next_call_id: Mutex::new(1) }
    }

    pub async fn call(&self) -> Result<Vec<u8>> {
        // placeholder: real implementation would build RMC packet, send it and wait for response via oneshot.
        Ok(vec![])
    }

    pub async fn register_pending(&self, call_id: u32) -> oneshot::Receiver<Result<Bytes>> {
        let (tx, rx) = oneshot::channel();
        self.pending.lock().unwrap().insert(call_id, tx);
        rx
    }

    pub fn resolve(&self, call_id: u32, payload: Result<Bytes>) {
        if let Some(tx) = self.pending.lock().unwrap().remove(&call_id) {
            let _ = tx.send(payload);
        }
    }
}
