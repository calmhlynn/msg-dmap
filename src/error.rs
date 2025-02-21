use thiserror::Error;
use tokio::io;

#[derive(Error, Debug)]
#[non_exhaustive]
pub(super) enum Error {
    #[error("Network connection failed: {0}")]
    Network(String),

    #[error("String parsing failed: {0}")]
    Parse(String),

    #[error("No messages available in the mpsc channel")]
    EmptyQueue,

    #[error("unknown error")]
    Unknown,
}
