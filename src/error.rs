use axum::extract::rejection::PathRejection;
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
    RouteNotFound,
    #[error("Your request timed out.")]
    RequestTimeout,
    #[error("An internal error has occurred.")]
    InternalServerError(anyhow::Error),
    #[error("A database error occurred.")]
    DatabaseError(sqlx::Error),
    #[error("This path is invalid.")]
    PathRejection(axum::extract::rejection::PathRejection),
}

#[derive(Debug, Serialize)]
pub struct UserError {
    timestamp: String,
    status: u16,
    error: &'static str,
    message: String,
}

impl UserError {
    pub fn new<S: AsRef<str>>(
        status_code: StatusCode,
        custom_error: Option<&'static str>,
        message: S,
    ) -> Self {
        let status = status_code.as_u16();

        let error = if let Some(error) = custom_error {
            error
        } else {
            status_code.canonical_reason().unwrap_or("Unknown Error")
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
            ApiError::RouteNotFound => {
                let status = StatusCode::NOT_FOUND;

                (status, UserError::new(status, None, self.to_string()))
            }
            ApiError::RequestTimeout => {
                let status = StatusCode::REQUEST_TIMEOUT;

                (status, UserError::new(status, None, self.to_string()))
            }
            ApiError::InternalServerError(ref e) => {
                error!("{}", e);

                let status = StatusCode::INTERNAL_SERVER_ERROR;

                (status, UserError::new(status, None, self.to_string()))
            }
            ApiError::DatabaseError(ref e) => match e {
                sqlx::Error::RowNotFound => {
                    let status = StatusCode::NOT_FOUND;

                    (
                        status,
                        UserError::new(status, None, "This data was not found in the database."),
                    )
                }
                _ => {
                    let status = StatusCode::INTERNAL_SERVER_ERROR;

                    (status, UserError::new(status, None, self.to_string()))
                }
            },
            ApiError::PathRejection(e) => {
                let message = e.to_string();

                let status = match e {
                    PathRejection::FailedToDeserializePathParams(inner) => {
                        let kind = inner.into_kind();

                        match &kind {
                            axum::extract::path::ErrorKind::UnsupportedType { .. } => {
                                StatusCode::INTERNAL_SERVER_ERROR
                            }
                            _ => StatusCode::BAD_REQUEST,
                        }
                    }
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };

                (status, UserError::new(status, None, message))
            }
        };

        (status, Json(error)).into_response()
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(sqlx_err: sqlx::Error) -> Self {
        Self::DatabaseError(sqlx_err)
    }
}
