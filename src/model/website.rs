#[derive(Deserialize, Debug, Clone)]
pub struct Website {
    #[serde(default)]
    pub id: usize,
    #[serde(default)]
    pub category: usize,
    #[serde(default)]
    pub game: usize,
    #[serde(default)]
    pub trusted: bool,
    #[serde(default)]
    pub url: String,
}
