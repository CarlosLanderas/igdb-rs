#[derive(Deserialize, Debug, Clone)]
pub struct MultiplayerMode {
    #[serde(default)]
    pub id: usize,
    #[serde(default)]
    pub campaingcoop: bool,
    #[serde(default)]
    pub dropin: bool,
    #[serde(default)]
    pub game: usize,
    #[serde(default)]
    pub lancoop: bool,
    #[serde(default)]
    pub offlinecoop: bool,
    #[serde(default)]
    pub offlinecoopmax: usize,
    #[serde(default)]
    pub offlinemax: usize,
    #[serde(default)]
    pub onlinecoop: bool,
    #[serde(default)]
    pub onlinecoopmax: usize,
    #[serde(default)]
    pub onlinemax: usize,
    #[serde(default)]
    pub platform: usize,
    #[serde(default)]
    pub splitscreen: bool,
    #[serde(default)]
    pub splitscreenonline: bool,
}
