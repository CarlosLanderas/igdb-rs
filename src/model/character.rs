use crate::model::enums::{Gender, Species};

#[derive(Deserialize, Debug, Clone)]
pub struct Character {
    #[serde(default)]
    pub id: usize,
    #[serde(default)]
    pub akas: Vec<String>,
    #[serde(default)]
    pub country_name: String,
    #[serde(default)]
    pub created_at: u64,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub games: Vec<usize>,
    #[serde(default)]
    pub gender: Gender,
    #[serde(default)]
    pub mug_shot: usize,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub people: Vec<usize>,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub species: Species,
    #[serde(default)]
    pub updated_at: u64,
    #[serde(default)]
    pub url: String,
}
