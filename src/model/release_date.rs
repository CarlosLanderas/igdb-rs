use crate::model::enums::{Platform, Region};

#[derive(Deserialize, Debug)]
pub struct ReleaseDate {
    #[serde(default)]
    pub category: usize,
    #[serde(default)]
    pub created_at: u64,
    #[serde(default)]
    pub date: u64,
    #[serde(default)]
    pub game: usize,
    #[serde(default)]
    pub human: String,
    #[serde(default)]
    pub m: usize,
    #[serde(default)]
    pub platform: Platform,
    #[serde(default)]
    pub region: Region,
    #[serde(default)]
    pub updated_at: u64,
    #[serde(default)]
    pub y: usize,
}
