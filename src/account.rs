use crate::types::pid::PID;
use serde::{Serialize, Deserialize};

/// Game server account (separate from platform accounts).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub pid: PID,
    pub username: String,
    pub password: String,
    pub requires_token_auth: bool,
}

impl Account {
    pub fn new(pid: PID, username: impl Into<String>, password: impl Into<String>, requires_token_auth: bool) -> Self {
        Self {
            pid,
            username: username.into(),
            password: password.into(),
            requires_token_auth,
        }
    }
}
