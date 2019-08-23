#[derive(Deserialize, Debug, Clone)]
pub struct Engine {
    #[serde(default)]
    pub id : usize,
    #[serde(default)]
    pub companies: Vec<usize>,
    #[serde(default)]
    pub created_at : u64,
    #[serde(default)]
    pub description : String,
    #[serde(default)]
    pub logo: usize,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub platforms: Vec<usize>,
    #[serde(default)]
    pub url: String

}