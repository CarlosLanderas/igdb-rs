#![feature(async_await)]
use async_std::task;
use igdb_client::client::IGDBClient;
fn main() {
    task::block_on(async {
        let games_client = IGDBClient::new("586677e082e930d4c44a59962420e9d1").games();

        let mut game_request = IGDBClient::create_request();
        game_request
            .add_field("name")
            .add_field("summary")
            .search("Witcher 3");

        let results = games_client.get(&game_request).await.unwrap();
        let game = results.first().unwrap();

        println!("Name: {}", game.name);
        println!("Summary: {} ...", &game.summary[..100]);
    })
}
