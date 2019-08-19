pub mod games;
pub mod rating;

#[derive(Deserialize)]
pub enum Category {
    MainGame = 0,
    DlcAddon = 1,
    Expansion = 2,
    Bundle = 3,
    StandaloneExpansion = 4,
}

impl Default for Category {
    fn default() -> Self {
        Category::MainGame
    }
}

#[derive(Deserialize)]
pub enum Status {
    Released = 0,
    Alpha = 2,
    Beta = 3,
    EarlyAccess = 4,
    Offline = 5,
    Cancelled = 6,
}

impl Default for Status {
    fn default() -> Self {
        Status::Released
    }
}
#[derive(Deserialize, Default)]
pub struct Keyword {
    created_at: u64,
    name: String,
    slug: String,
    updated_at: u64,
    url: String,
}
