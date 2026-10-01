use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("data feed error: {0}")]
    Feed(String),

    #[error("auth error: {0}")]
    Auth(String),

    #[error("storage error: {0}")]
    Storage(String),

    #[error("calendar error: {0}")]
    Calendar(String),

    #[error("broker error: {0}")]
    Broker(String),

    #[error("config error: {0}")]
    Config(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
