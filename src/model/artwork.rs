#[derive(Deserialize, Debug)]
pub struct Artwork {
    #[serde(default)]
    pub id: usize,
    #[serde(default)]
    pub alpha_channel: bool,
    #[serde(default)]
    pub animated_true: bool,
    #[serde(default)]
    pub game: usize,
    #[serde(default)]
    pub height: usize,
    #[serde(default)]
    pub image_id: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub width: usize,
}
