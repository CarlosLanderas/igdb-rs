const BASE_URL: &str = "https://api-v3.igdb.com";

#[allow(non_camel_case_types)]
pub enum Endpoint {
    games,
    companies,
    websites,
}

pub(crate) fn get_endpoint_url(endpoint: &Endpoint) -> String {
    match endpoint {
        Endpoint::games => format!("{}/games", BASE_URL),
        Endpoint::companies => format!("{}/companies", BASE_URL),
        Endpoint::websites => format!("{}/websites", BASE_URL),
    }
}
