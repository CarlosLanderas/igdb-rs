#[derive(Deserialize, Debug)]
pub struct AgeRating {
    category: Category,
    content_descriptions: usize,
    rating: Rating,
    rating_cover_url: String,
    synopsis: String,
}
