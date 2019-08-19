#[derive(Deserialize, Default)]
pub struct Keyword {
    created_at: u64,
    name: String,
    slug: String,
    updated_at: u64,
    url: String,
}
