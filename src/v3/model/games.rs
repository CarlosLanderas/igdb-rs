use crate::v3::model::rating::AgeRating;
use crate::v3::model::{Category, Status};
use crate::v3::request_builder::RequestBuilder;
use std::collections::HashMap;
use std::vec::Vec;

#[derive(Deserialize)]
pub struct GameResponse {
    // pub age_ratings: AgeRating,
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
    pub category: Category,
    #[serde(default)]
    pub collection: u32,
    #[serde(default)]
    pub cover: u32,
    #[serde(default)]
    pub created_at: u64,
    #[serde(default)]
    pub dlcs: u32,
    #[serde(default)]
    pub external_games: Vec<u32>,
    #[serde(default)]
    pub first_release_date: u64,
    #[serde(default)]
    pub follows: u32,
    #[serde(default)]
    pub franchise: u32,
    #[serde(default)]
    pub franchises: Vec<u32>,
    #[serde(default)]
    pub game_engines: Vec<u32>,
    #[serde(default)]
    pub game_modes: Vec<u32>,
    #[serde(default)]
    pub genres: Vec<u32>,
    #[serde(default)]
    pub hypes: u32,
    #[serde(default)]
    pub involved_companies: Vec<u32>,
    #[serde(default)]
    pub keywords: Vec<u32>,
    #[serde(default)]
    pub multiplayer_modes: Vec<u32>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub parent_game: u32,
    #[serde(default)]
    pub platforms: Vec<u32>,
    #[serde(default)]
    pub player_perspectives: Vec<u32>,
    #[serde(default)]
    pub popularity: f32,
    #[serde(default)]
    pub pulse_count: u32,
    #[serde(default)]
    pub rating: f32,
    #[serde(default)]
    pub rating_count: u32,
    #[serde(default)]
    pub release_dates: Vec<u32>,
    #[serde(default)]
    pub screenshots: Vec<u32>,
    #[serde(default)]
    pub similar_games: u32,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub standalone_expansions: u32,
    #[serde(default)]
    pub status: Status,
    #[serde(default)]
    pub storyline: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub tags: Vec<u32>,
    #[serde(default)]
    pub themes: Vec<u32>,
    #[serde(default)]
    pub time_to_beat: usize,
    #[serde(default)]
    pub total_rating: f32,
    #[serde(default)]
    pub total_rating_count: u32,
    #[serde(default)]
    pub updated_at: u64,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub version_parent: u32,
    #[serde(default)]
    pub version_title: String,
    #[serde(default)]
    pub videos: u32,
    #[serde(default)]
    pub websites: u32,
}
