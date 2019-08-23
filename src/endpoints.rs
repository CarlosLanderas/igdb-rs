const BASE_URL: &str = "https://api-v3.igdb.com";

#[allow(non_camel_case_types)]
pub enum Endpoint {
    games,
    game_engines,
    companies,
    websites,
    artworks,
    characters,
    game_modes,
    multiplayer_modes,
    covers,
    screenshots,
    release_dates,
    game_videos,
    platforms,
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
        Endpoint::screenshots => format!("{}/screenshots", BASE_URL),
        Endpoint::release_dates => format!("{}/release_dates", BASE_URL),
        Endpoint::game_engines => format!("{}/game_engines", BASE_URL),
        Endpoint::game_videos => format!("{}/game_videos", BASE_URL),
        Endpoint::platforms => format!("{}/platforms", BASE_URL),
    }
}
