#![feature(async_await)]

use igdb_client::v3::endpoints::Endpoint;
use igdb_client::v3::model::games::GameResponse;
use igdb_client::v3::request_builder::{Equality, RequestBuilder};
use igdb_client::v3::IGDBClient;

fn main() {
    async_std::task::block_on(async {
        let client = IGDBClient::new("586677e082e930d4c44a59962420e9d1");

        let game_request = client
            .request(Endpoint::Games)
            .add_fields(vec!["name", "summary", "time_to_beat", "status"])
            .add_where("id", Equality::Equal, "1942");

        let result: Vec<GameResponse> = client.get(&game_request).await;
        println!("{}", result.first().unwrap().name);
        println!("{}", result.first().unwrap().summary);
    })
}
