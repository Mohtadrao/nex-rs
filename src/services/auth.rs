
use crate::services::Service;
use async_trait::async_trait;
use bytes::Bytes;
use crate::Result;

pub struct AuthService;

impl AuthService {
    pub fn new() -> Self { Self{} }
}

#[async_trait]
impl Service for AuthService {
    async fn handle(&self, method: u16, body: Bytes) -> Result<Bytes> {
        // method 1 = login (example)
        match method {
            1 => {
                // parse body (naive): username\0password
                let v = body.to_vec();
                // respond with dummy success token
                Ok(Bytes::from(vec![1,2,3,4]))
            }
            _ => Ok(Bytes::from(vec![]))
        }
    }
}
