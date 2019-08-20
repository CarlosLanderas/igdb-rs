#[derive(Deserialize, Debug)]
pub struct Website {
    #[serde(default)]
    pub category: usize,
    #[serde(default)]
    pub game: usize,
    #[serde(default)]
    pub trusted: bool,
    #[serde(default)]
    pub url: String,
}
