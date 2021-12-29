use axum::extract::Extension;
use axum::routing::get;
use axum::Router;

use crate::error::ApiError;
use crate::models::custom_path::CustomPath;
use crate::models::json_response::JsonResponse;
use crate::models::pka_episode_with_all::PkaEpisodeWithAll;
use crate::{conduit, Db};

pub const EPISODE_ENDPOINT: &str = "/episode";
pub const WATCH_ENDPOINT: &str = "/watch";

pub fn episode_routes() -> Router {
    Router::new()
        .nest(WATCH_ENDPOINT, watch_routes())
        .route("/youtube_link/:number", get(youtube_link))
}

pub fn watch_routes() -> Router {
    Router::new()
        .route("/:number", get(watch_number))
        .route("/latest", get(watch_latest))
        .route("/random", get(watch_random))
}

async fn watch_number(
    CustomPath(number): CustomPath<f32>,
    Extension(db): Extension<Db>,
) -> Result<JsonResponse<PkaEpisodeWithAll>, ApiError> {
    let res = conduit::pka_episode::find_with_all(&db, number).await?;

    Ok(JsonResponse::success(res))
}

async fn watch_latest(
    Extension(db): Extension<Db>,
) -> Result<JsonResponse<PkaEpisodeWithAll>, ApiError> {
    let number = conduit::pka_episode::latest_number(&db).await?;

    let res = conduit::pka_episode::find_with_all(&db, number).await?;

    Ok(JsonResponse::success(res))
}

async fn watch_random(
    Extension(db): Extension<Db>,
) -> Result<JsonResponse<PkaEpisodeWithAll>, ApiError> {
    let number = conduit::pka_episode::random_number(&db).await?;

    let res = conduit::pka_episode::find_with_all(&db, number).await?;

    Ok(JsonResponse::success(res))
}

async fn youtube_link(
    CustomPath(number): CustomPath<f32>,
    Extension(db): Extension<Db>,
) -> Result<JsonResponse<String>, ApiError> {
    let res = conduit::pka_episode::youtube_link(&db, number).await?;

    Ok(JsonResponse::success(res))
}
