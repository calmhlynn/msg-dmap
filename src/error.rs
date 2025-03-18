use thiserror::Error;
use tokio::io;

#[derive(Error, Debug)]
#[non_exhaustive]
pub(super) enum Error {
    #[error("Network connection failed: {0}")]
    NetworkConnectionFailed(String),

    #[error("String parsing failed: {0}")]
    StringParsingFailed(String),

    #[error("No messages available in the mpsc channel")]
    NoMessagesInQueue,

    #[error("unknown error")]
    UnknownError,
}
