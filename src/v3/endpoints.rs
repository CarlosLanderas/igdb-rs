const BASE_URL: &str = "https://api-v3.igdb.com";

pub fn games() -> String {
    format!("{}/games", BASE_URL)
}

pub fn achievements() -> String {
    format!("{}/achievements", BASE_URL)
}
