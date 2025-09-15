
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::Result;
use crate::prudp::connection::Connection as PrudpConnection;
use tracing::info;

// #[derive(Debug)]
pub struct Session {
    pub id: u32,
    pub conn: Arc<PrudpConnection>,
    pub established_at: std::time::Instant,
}

pub struct RVManager {
    pub sessions: Mutex<HashMap<u32, Arc<PrudpConnection>>>,
    pub next_session_id: Mutex<u32>,
}

impl RVManager {
    pub fn new() -> Self {
        Self { sessions: Mutex::new(HashMap::new()), next_session_id: Mutex::new(1) }
    }

    pub async fn create_session(&self, conn: Arc<PrudpConnection>) -> u32 {
        let mut id_lock = self.next_session_id.lock().await;
        let id = *id_lock;
        *id_lock += 1;
        self.sessions.lock().await.insert(id, conn);
        info!("session {} created", id);
        id
    }

    pub async fn get_session(&self, id: u32) -> Option<Arc<PrudpConnection>> {
        self.sessions.lock().await.get(&id).cloned()
    }

    pub async fn remove_session(&self, id: u32) {
        self.sessions.lock().await.remove(&id);
        info!("session {} removed", id);
    }
}
