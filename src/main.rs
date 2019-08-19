#![feature(async_await)]

use igdb_client::client::IGDBClient;
use igdb_client::model::games::GameResponse;
use igdb_client::request_builder::Equality;

fn main() {
    async_std::task::block_on(async {
        let games_client = IGDBClient::new("586677e082e930d4c44a59962420e9d1").games();

        let request = games_client
            .request()
            .add_fields(vec!["name", "summary", "time_to_beat", "status", "url", "total_rating"])
            .search("Witcher")
            .limit(3);

        let result = games_client.get(&request).await;

        for game in result {
            println!("Name: {}", game.name);
            println!("Summary: {}", game.summary);
            println!("Time to beat: {}", game.time_to_beat);
            println!("Url: {}", game.url);
            println!("Total rating: {}", game.total_rating);
        }

    })
}
