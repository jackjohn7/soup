use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SoupError {
    #[error("invalid header provided (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: &'static str,
        found: String,
    },

    /// Indicates that a particular header required for a route has not
    /// been provided
    /// ```rust
    /// pub fn get() -> Result<(), SoupError> {
    ///     Err(SoupError::MissingHeader("ACCEPT"))
    /// }
    /// ```
    #[error("required header \"{0}\" not provided")]
    MissingHeader(&'static str),

    #[error("not implemented")]
    NotImplemented,

    #[error("an error occurred with the database")]
    DatabaseError,
}

impl IntoResponse for SoupError {
    fn into_response(self) -> Response {
        match self {
            Self::InvalidHeader { .. } => (StatusCode::BAD_REQUEST, self.to_string()),
            Self::MissingHeader(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            Self::NotImplemented => (StatusCode::NOT_IMPLEMENTED, self.to_string()),
            Self::DatabaseError => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        }
        .into_response()
    }
}
