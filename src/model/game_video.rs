#[derive(Deserialize, Debug, Clone)]
pub struct GameVideo {
    #[serde(default)]
    pub id: usize,
    #[serde(default)]
    pub game: usize,
    #[serde(default)]
    pub video_id: String,
}
