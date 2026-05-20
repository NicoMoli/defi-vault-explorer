//! API error type and its mapping to HTTP responses.
//!
//! Every error reaches the client as a typed body `{ "error", "message" }`.

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

/// All failure modes the API surfaces to clients.
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    /// The upstream provider failed and no cached data was available.
    /// This is the cold-start failure path; it maps to HTTP 503.
    #[error("upstream provider unavailable: {0}")]
    UpstreamUnavailable(String),

    /// A requested resource (e.g. a vault address) does not exist.
    #[error("not found: {0}")]
    NotFound(String),
}

impl ApiError {
    /// Convenience constructor for the cold-start upstream failure.
    pub fn upstream_unavailable(detail: impl Into<String>) -> Self {
        Self::UpstreamUnavailable(detail.into())
    }

    /// Machine-readable error code carried in the response body.
    fn code(&self) -> &'static str {
        match self {
            Self::UpstreamUnavailable(_) => "upstream_unavailable",
            Self::NotFound(_) => "not_found",
        }
    }

    /// HTTP status this error maps to.
    fn status(&self) -> StatusCode {
        match self {
            Self::UpstreamUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }
}

/// The typed error body returned for every `ApiError`.
#[derive(Debug, Serialize)]
struct ErrorBody {
    error: &'static str,
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = ErrorBody {
            error: self.code(),
            message: self.to_string(),
        };
        (self.status(), Json(body)).into_response()
    }
}
