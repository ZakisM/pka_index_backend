use axum::extract::Extension;
use axum::routing::get;
use axum::Router;

use crate::error::ApiError;
use crate::models::custom_path::CustomPath;
use crate::models::json_response::JsonResponse;
use crate::models::pka_episode::PkaEpisode;
use crate::models::pka_episode_with_all::PkaEpisodeWithAll;
use crate::models::pka_event::PkaEvent;
use crate::models::pka_youtube_details::PkaYoutubeDetails;
use crate::Db;

pub const EPISODE_ENDPOINT: &str = "/episode";

pub fn episode_routes() -> Router {
    Router::new().route("/watch/:number", get(watch_pka_episode))
}

async fn watch_pka_episode(
    CustomPath(number): CustomPath<f32>,
    Extension(db): Extension<Db>,
) -> Result<JsonResponse<PkaEpisodeWithAll>, ApiError> {
    let episode = sqlx::query_as!(
        PkaEpisode,
        "SELECT * FROM pka_episode WHERE number = ?",
        number
    )
    .fetch_one(&*db)
    .await?;

    // Using unchecked as i32 not being recognized as correct type.
    let youtube_details = sqlx::query_as_unchecked!(
        PkaYoutubeDetails,
        "SELECT * FROM pka_youtube_details WHERE episode_number = ?",
        number,
    )
    .fetch_one(&*db)
    .await?;

    // Using unchecked as i32 not being recognized as correct type.
    let events = sqlx::query_as_unchecked!(
        PkaEvent,
        "SELECT * FROM pka_event WHERE episode_number = ? ORDER BY timestamp ASC",
        number,
    )
    .fetch_all(&*db)
    .await?;

    let res = PkaEpisodeWithAll::new(episode, youtube_details, events);

    Ok(JsonResponse::success(res))
}
