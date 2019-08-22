#[derive(Deserialize, Debug)]
pub struct Screenshot {
    #[serde(default)]
    pub id: usize,
    #[serde(default)]
    pub alpha_channel: bool,
    #[serde(default)]
    pub animated: bool,
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
