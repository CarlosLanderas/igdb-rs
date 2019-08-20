#[derive(Deserialize, Debug)]
pub struct Company {
    #[serde(default)]
    pub id: usize,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub published: usize,
    #[serde(default)]
    pub created_at: u64,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub url: String,
}
