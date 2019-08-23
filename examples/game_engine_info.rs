use async_std::task;
use igdb_client::client::IGDBClient;
fn main() {
    task::block_on(async {
        let igdb_client = IGDBClient::new("586677e082e930d4c44a59962420e9d1");

        let games_client = igdb_client.games();
        let game = games_client
            .get_first_by_name("Riders of Asgard")
            .await
            .unwrap();
        let engine_id = game.game_engines.first().unwrap();

        let engines_client = igdb_client.game_engines();
        let engine = engines_client
            .get_first_by_id(*engine_id as usize)
            .await
            .unwrap();

        println!(
            "name: {}, url: {}, companies: {:?}",
            engine.name, engine.url, engine.companies
        );

        // name: Unreal Engine 4, url: https://www.igdb.com/game_engines/unreal-engine-4--1,
        // companies: [168, 11060]
    })
}
