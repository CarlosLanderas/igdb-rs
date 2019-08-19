#![feature(async_await)]

use igdb_client::client::IGDBClient;
use igdb_client::model::games::GameResponse;
use igdb_client::request_builder::Equality;

fn main() {
    async_std::task::block_on(async {
        let games_client = IGDBClient::new("586677e082e930d4c44a59962420e9d1").games();

        let witcher_request = games_client
            .request()
            .add_fields(vec!["name", "summary", "time_to_beat", "status", "url", "total_rating"])
            .search("Witcher")
            .limit(3);

        let result = games_client.get(&witcher_request).await;
        show_games(result);

        let resident_evil = games_client
            .request()
            .add_fields(vec!["name", "summary"])
            .search("Resident Evil")
            .limit(1);

        let result2 = games_client.get(&resident_evil).await;
        show_games(result2);
    })
}

fn show_games(games: Vec<GameResponse>) {
    for game in games {
        println!("Name: {}", game.name);
        println!("Summary: {}", game.summary);
        println!("Time to beat: {}", game.time_to_beat);
        println!("Url: {}", game.url);
        println!("Total rating: {}", game.total_rating);
    }
}

