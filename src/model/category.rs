#[derive(Deserialize, Debug)]
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
