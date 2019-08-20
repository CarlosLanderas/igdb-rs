#[derive(Deserialize, Debug)]
pub struct Character {
    #[serde(default)]
    pub akas: Vec<String>,
    #[serde(default)]
    pub country_name : String,
    #[serde(default)]
    pub created_at: u64,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub games: Vec<usize>,
    #[serde(default)]
    pub gender: usize,
    #[serde(default)]
    pub mug_shot: usize,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub people: Vec<usize>,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub species: usize,
    #[serde(default)]
    pub updated_at: u64,
    #[serde(default)]
    pub url: String


}
