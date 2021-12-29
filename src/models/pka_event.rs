use serde::Serialize;

use crate::models::sqlx_types::SqliteI32;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PkaEvent {
    #[serde(skip_serializing)]
    pub event_id: String,
    #[serde(skip_serializing)]
    pub episode_number: f32,
    pub timestamp: SqliteI32,
    pub description: String,
    pub length_seconds: SqliteI32,
    pub upload_date: i64,
}

impl PkaEvent {
    pub fn new(
        event_id: String,
        episode_number: f32,
        timestamp: i32,
        description: String,
        length_seconds: i32,
        upload_date: i64,
    ) -> Self {
        PkaEvent {
            event_id,
            episode_number,
            timestamp: SqliteI32(timestamp),
            description,
            length_seconds: SqliteI32(length_seconds),
            upload_date,
        }
    }
}
