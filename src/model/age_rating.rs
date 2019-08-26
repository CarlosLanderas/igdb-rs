use crate::model::enums::{Rating, RatingCategory};

#[derive(Deserialize, Debug, Clone)]
pub struct AgeRating {
    #[serde(default)]
    pub id: usize,
    #[serde(default)]
    pub category: RatingCategory,
    #[serde(default)]
    pub content_descriptions: Vec<usize>,
    pub rating: Rating,
    #[serde(default)]
    pub rating_cover_url: String,
    #[serde(default)]
    pub synopsis: String,
}
