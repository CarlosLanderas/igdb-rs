#[derive(Deserialize, Debug, Clone)]
pub struct PlatformLogo {
    #[serde(default)]
    pub alpha_channel: bool,
    #[serde(default)]
    pub animated: bool,
    #[serde(default)]
    pub height: usize,
    #[serde(default)]
    pub image_id: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub width: usize,
}
