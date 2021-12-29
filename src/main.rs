use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, Result};
use axum::error_handling::HandleErrorLayer;
use axum::handler::Handler;
use axum::http::Method;
use axum::response::IntoResponse;
use axum::{AddExtensionLayer, Router};
use sqlx::{Pool, Sqlite};
use tower::{BoxError, ServiceBuilder};
use tower_http::cors::{CorsLayer, Origin};
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::error::ApiError;
use crate::handler::episode::{episode_routes, EPISODE_ENDPOINT};
use crate::setup::setup_database;

type Db = Arc<Pool<Sqlite>>;

mod conduit;
mod error;
mod handler;
mod models;
mod setup;

const API_ENDPOINT: &str = "/v1/api";

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;

    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "pka_index_backend=info,tower_http=info");
    }

    tracing_subscriber::fmt::init();

    let database = setup_database().await?;

    let main_router = Router::new().nest(EPISODE_ENDPOINT, episode_routes());

    let app = Router::new()
        .nest(API_ENDPOINT, main_router)
        .fallback(handler_404.into_service())
        .layer(
            ServiceBuilder::new()
                .layer(
                    CorsLayer::new()
                        .allow_origin(Origin::exact(
                            "http://0.0.0.0:5678"
                                .parse()
                                .expect("Failed to parse local origin."),
                        ))
                        .allow_methods(vec![Method::GET, Method::POST]),
                )
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    let result: Result<ApiError, _> =
                        if error.is::<tower::timeout::error::Elapsed>() {
                            Err(ApiError::RequestTimeout)
                        } else {
                            Err(ApiError::InternalServerError(anyhow!(error)))
                        };

                    result
                }))
                .layer(TraceLayer::new_for_http())
                .layer(AddExtensionLayer::new(database))
                .timeout(Duration::from_secs(10))
                .into_inner(),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 1234));

    info!("Server started at: {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server.");

    Ok(())
}

async fn handler_404() -> impl IntoResponse {
    ApiError::RouteNotFound
}
