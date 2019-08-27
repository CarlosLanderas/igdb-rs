#[derive(Deserialize, Clone, Debug)]
pub struct CharacterMugshot {
    pub id: usize,
    pub alpha_channel: bool,
    pub animated: bool,
    pub height: usize,
    pub image_id: String,
    pub url: String,
    pub width: usize,
}
