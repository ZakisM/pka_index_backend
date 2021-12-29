use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PkaYoutubeDetails {
    pub video_id: String,
    #[serde(skip_serializing)]
    pub episode_number: f32,
    pub title: String,
    pub length_seconds: i32,
}

impl PkaYoutubeDetails {
    pub fn new(video_id: String, episode_number: f32, title: String, length_seconds: i32) -> Self {
        PkaYoutubeDetails {
            video_id,
            episode_number,
            title,
            length_seconds,
        }
    }
}
