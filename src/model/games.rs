use crate::model::enums::{GameCategory, Status};
use std::vec::Vec;

#[derive(Deserialize, Debug, Clone)]
pub struct Game {
    #[serde(default)]
    pub age_ratings: Vec<usize>,
    #[serde(default)]
    pub aggregated_rating: f32,
    #[serde(default)]
    pub aggregated_rating_count: usize,
    #[serde(default)]
    pub alternative_names: Vec<usize>,
    #[serde(default)]
    pub artworks: Vec<usize>,
    #[serde(default)]
    pub bundles: Vec<usize>,
    #[serde(default)]
    pub category: GameCategory,
    #[serde(default)]
    pub collection: usize,
    #[serde(default)]
    pub cover: usize,
    #[serde(default)]
    pub created_at: u64,
    #[serde(default)]
    pub dlcs: Vec<usize>,
    #[serde(default)]
    pub external_games: Vec<usize>,
    #[serde(default)]
    pub first_release_date: u64,
    #[serde(default)]
    pub follows: usize,
    #[serde(default)]
    pub franchise: usize,
    #[serde(default)]
    pub franchises: Vec<usize>,
    #[serde(default)]
    pub game_engines: Vec<usize>,
    #[serde(default)]
    pub game_modes: Vec<usize>,
    #[serde(default)]
    pub genres: Vec<usize>,
    #[serde(default)]
    pub hypes: usize,
    #[serde(default)]
    pub id: usize,
    #[serde(default)]
    pub involved_companies: Vec<usize>,
    #[serde(default)]
    pub keywords: Vec<usize>,
    #[serde(default)]
    pub multiplayer_modes: Vec<usize>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub parent_game: usize,
    #[serde(default)]
    pub platforms: Vec<usize>,
    #[serde(default)]
    pub player_perspectives: Vec<usize>,
    #[serde(default)]
    pub popularity: f32,
    #[serde(default)]
    pub pulse_count: usize,
    #[serde(default)]
    pub rating: f32,
    #[serde(default)]
    pub rating_count: usize,
    #[serde(default)]
    pub release_dates: Vec<usize>,
    #[serde(default)]
    pub screenshots: Vec<usize>,
    #[serde(default)]
    pub similar_games: Vec<usize>,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub standalone_expansions: Vec<usize>,
    #[serde(default)]
    pub status: Status,
    #[serde(default)]
    pub storyline: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub tags: Vec<usize>,
    #[serde(default)]
    pub themes: Vec<usize>,
    #[serde(default)]
    pub time_to_beat: usize,
    #[serde(default)]
    pub total_rating: f32,
    #[serde(default)]
    pub total_rating_count: usize,
    #[serde(default)]
    pub updated_at: u64,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub version_parent: usize,
    #[serde(default)]
    pub version_title: String,
    #[serde(default)]
    pub videos: Vec<usize>,
    #[serde(default)]
    pub websites: Vec<usize>,
}
