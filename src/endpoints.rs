const BASE_URL: &str = "https://api-v3.igdb.com";

pub enum Endpoint {
    Games,
    Companies,
}

pub(crate) fn get_endpoint_url(endpoint: &Endpoint) -> String {
    match endpoint {
        Endpoint::Games => format!("{}/games", BASE_URL),
        Endpoint::Companies => format!("{}/companies", BASE_URL),
    }
}
