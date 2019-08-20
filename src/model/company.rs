#[derive(Deserialize, Debug)]
pub struct Company {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub published: usize,
}
