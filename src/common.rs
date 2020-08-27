//! Common types for requests and responses
use strum_macros::{Display, EnumString};
use thiserror::Error;

/// HTTP versions in the format `Http<major><minor>`
#[derive(Debug, Display, EnumString)]
pub enum HttpVersion {
    /// HTTP 0.9
    #[strum(serialize = "HTTP/0.9")]
    Http09,
    /// HTTP 1.0
    #[strum(serialize = "HTTP/1.0")]
    Http10,
    /// HTTP 1.1
    #[strum(serialize = "HTTP/1.1")]
    Http11,
    /// HTTP 2.0
    #[strum(serialize = "HTTP/2.0")]
    Http20,
    /// HTTP 3.0
    #[strum(serialize = "HTTP/3.0")]
    Http30,
}

/// Error types for the server
#[derive(Error, Debug)]
pub enum HttpError<S: Into<String> + std::fmt::Debug + std::fmt::Display> {
    #[error("Field `{0}` was not found")]
    FieldNotFound(S),
}
