use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PkaEpisode {
    pub number: f32,
    pub name: String,
    #[serde(skip_serializing)]
    pub youtube_link: String,
    pub upload_date: i64,
}

impl PkaEpisode {
    pub fn new(number: f32, name: String, youtube_link: String, upload_date: i64) -> Self {
        PkaEpisode {
            number,
            name,
            youtube_link,
            upload_date,
        }
    }
}
