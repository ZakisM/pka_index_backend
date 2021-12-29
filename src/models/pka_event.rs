use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PkaEvent {
    #[serde(skip_serializing)]
    pub event_id: String,
    #[serde(skip_serializing)]
    pub episode_number: f32,
    pub timestamp: i32,
    pub description: String,
    pub length_seconds: i32,
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
            timestamp,
            description,
            length_seconds,
            upload_date,
        }
    }
}
