use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{message} (code: {result_code})")]
    Nex { result_code: i32, message: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Timeout")]
    Timeout,

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl Error {
    pub fn new(result_code: i32, message: impl Into<String>) -> Self {
        Self::Nex { result_code, message: message.into() }
    }
}