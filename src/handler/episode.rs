use axum::Router;

pub const EPISODE_ENDPOINT: &str = "/episode";

pub fn episode_routes() -> Router {
    Router::new()
}
