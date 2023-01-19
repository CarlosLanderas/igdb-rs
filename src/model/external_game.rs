use crate::model::enums::{ExternalGameCategory, Media, PlatformCategory};
use std::vec::Vec;

#[derive(Deserialize, Debug, Clone)]
pub struct ExternalGame {
    #[serde(default)]
    pub category: ExternalGameCategory,
    #[serde(default)]
    pub uuid: usize,
    #[serde(default)]
    pub countries: Vec<u32>,
    #[serde(default)]
    pub created_at: u64,
    #[serde(default)]
    pub game: usize,
    #[serde(default)]
    pub media: Media,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub platform: PlatformCategory,
    #[serde(default)]
    pub uid: String,
    #[serde(default)]
    pub updated_at: u64,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub year: u16,
}
