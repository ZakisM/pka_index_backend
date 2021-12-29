use rand::seq::SliceRandom;

use crate::models::pka_episode::PkaEpisode;
use crate::models::pka_episode_with_all::PkaEpisodeWithAll;
use crate::models::pka_event::PkaEvent;
use crate::models::pka_youtube_details::PkaYoutubeDetails;
use crate::{ApiError, Db};

pub async fn find_with_all(db: &Db, number: f32) -> Result<PkaEpisodeWithAll, ApiError> {
    let episode = sqlx::query_as!(
        PkaEpisode,
        "SELECT * FROM pka_episode WHERE number = ?",
        number
    )
    .fetch_one(&**db)
    .await?;

    // Using unchecked as i32 not being recognized as correct type.
    let youtube_details = sqlx::query_as_unchecked!(
        PkaYoutubeDetails,
        "SELECT * FROM pka_youtube_details WHERE episode_number = ?",
        number,
    )
    .fetch_one(&**db)
    .await?;

    // Using unchecked as i32 not being recognized as correct type.
    let events = sqlx::query_as_unchecked!(
        PkaEvent,
        "SELECT * FROM pka_event WHERE episode_number = ? ORDER BY timestamp ASC",
        number,
    )
    .fetch_all(&**db)
    .await?;

    let res = PkaEpisodeWithAll::new(episode, youtube_details, events);

    Ok(res)
}

pub async fn latest_number(db: &Db) -> Result<f32, ApiError> {
    let res = sqlx::query!("SELECT (number) FROM pka_episode ORDER BY number DESC")
        .fetch_one(&**db)
        .await?;

    Ok(res.number)
}

pub async fn random_number(db: &Db) -> Result<f32, ApiError> {
    let all_res = sqlx::query!("SELECT (number) FROM pka_episode ORDER BY number DESC")
        .fetch_all(&**db)
        .await?;

    let res = if let Some(res) = all_res.choose(&mut rand::thread_rng()) {
        res.number
    } else {
        0.0
    };

    Ok(res)
}
