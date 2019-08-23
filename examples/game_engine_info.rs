#![feature(async_await)]
use async_std::task;
use igdb_client::client::IGDBClient;
fn main() {
    task::block_on(async {
        let igdb_client = IGDBClient::new("user-key");

        let games_client = igdb_client.games();
        let game = games_client.get_by_name("Riders of Asgard").await.unwrap();
        let engine_id = game.game_engines.first().unwrap();

        let engines_client = igdb_client.game_engines();
        let engine = engines_client.get_by_id(*engine_id as usize).await.unwrap();

        println!("{:?}", engine);

    })
}
