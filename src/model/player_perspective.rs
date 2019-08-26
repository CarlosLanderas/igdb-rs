#[derive(Deserialize, Debug, Clone)]
pub struct PlayerPerspective {
    #[serde(default)]
    pub id: usize,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub updated_at: u64,
    #[serde(default)]
    pub url: String,
}
