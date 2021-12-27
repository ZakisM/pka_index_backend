use std::env;
use std::net::SocketAddr;

use anyhow::Result;
use axum::handler::Handler;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use chrono::Utc;
use serde::Serialize;
use thiserror::Error;
use tower_http::trace::TraceLayer;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "pka_index_backend=trace,tower_http=trace");
    }

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(handler))
        .fallback(handler_404.into_service())
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 1234));

    info!("Server started at: {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server.");

    Ok(())
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello World!</h1>")
}

#[derive(Error, Debug, Serialize)]
enum ApiError {
    #[error("This route was not found")]
    NotFound,
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
        let error = match self {
            ApiError::NotFound => UserError::new(StatusCode::NOT_FOUND, None, self.to_string()),
        };

        Json(error).into_response()
    }
}

async fn handler_404() -> impl IntoResponse {
    ApiError::NotFound
}
