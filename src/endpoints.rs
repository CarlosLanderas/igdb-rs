const BASE_URL: &str = "https://api-v3.igdb.com";

#[allow(non_camel_case_types)]
pub enum Endpoint {
    games,
    companies,
    websites,
    artworks,
    characters,
    game_modes,
    multiplayer_modes,
    covers,
}

pub(crate) fn get_endpoint_url(endpoint: &Endpoint) -> String {
    match endpoint {
        Endpoint::games => format!("{}/games", BASE_URL),
        Endpoint::companies => format!("{}/companies", BASE_URL),
        Endpoint::websites => format!("{}/websites", BASE_URL),
        Endpoint::artworks => format!("{}/artworks", BASE_URL),
        Endpoint::characters => format!("{}/characters", BASE_URL),
        Endpoint::game_modes => format!("{}/game_modes", BASE_URL),
        Endpoint::multiplayer_modes => format!("{}/multiplayer_modes", BASE_URL),
        Endpoint::covers => format!("{}/covers", BASE_URL),
    }
}
