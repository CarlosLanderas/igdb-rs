#[derive(Deserialize, Debug)]
pub struct Website {
    pub category: usize,
    pub game: usize,
    pub trusted: bool,
    pub url: String
}