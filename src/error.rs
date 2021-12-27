use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use chrono::Utc;
use serde::Serialize;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("This route was not found.")]
    NotFound,
    #[error("Your request timed out.")]
    Timeout,
    #[error("An internal error has occurred.")]
    InternalError(anyhow::Error),
}

#[derive(Debug, Serialize)]
struct UserError<'a> {
    timestamp: String,
    status: u16,
    error: &'a str,
    message: String,
}

impl<'a> UserError<'a> {
    pub fn new<S: AsRef<str>>(
        status_code: StatusCode,
        custom_error: Option<&'a str>,
        message: S,
    ) -> Self {
        let status = status_code.as_u16();

        let error = if let Some(error) = custom_error {
            error
        } else {
            status_code.canonical_reason().unwrap_or("Unknown error")
        };

        let timestamp = Utc::now().to_rfc3339();

        let message = message.as_ref().to_owned();

        UserError {
            timestamp,
            status,
            error,
            message,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error) = match self {
            ApiError::NotFound => {
                let status = StatusCode::NOT_FOUND;

                (status, UserError::new(status, None, self.to_string()))
            }
            ApiError::Timeout => {
                let status = StatusCode::REQUEST_TIMEOUT;

                (status, UserError::new(status, None, self.to_string()))
            }
            ApiError::InternalError(ref e) => {
                error!("{}", e);

                let status = StatusCode::INTERNAL_SERVER_ERROR;

                (status, UserError::new(status, None, self.to_string()))
            }
        };

        (status, Json(error)).into_response()
    }
}
