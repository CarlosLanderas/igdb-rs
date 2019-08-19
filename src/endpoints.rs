const BASE_URL: &str = "https://api-v3.igdb.com";

pub enum Endpoint {
    Games,
    Achievements,
}

pub(crate) fn get_endpoint_url(endpoint: &Endpoint) -> String {
    match endpoint {
        Endpoint::Games => games(),
        Endpoint::Achievements => achievements(),
    }
}

fn games() -> String {
    format!("{}/games", BASE_URL)
}

fn achievements() -> String {
    format!("{}/achievements", BASE_URL)
}
