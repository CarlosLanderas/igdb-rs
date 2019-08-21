#![feature(async_await)]
use async_std::task;
use igdb_client::client::IGDBClient;
fn main() {
    task::block_on(async {
        let games_client = IGDBClient::new("user-key").games();

        let game_request = games_client
            .request()
            .add_field("name")
            .add_field("summary")
            .search("Witcher 3");

        let results = games_client.get(&game_request).await.unwrap();
        let game = results.first().unwrap();

        println!("Name: {}", game.name);
        println!("Summary: {} ...", &game.summary[..100]);
    })
}
