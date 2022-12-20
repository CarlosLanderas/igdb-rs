use async_std::task;
use igdb_rs::client::IGDBClient;
fn main() {
    task::block_on(async {
        use std::env;
        let client_id = env::var("IGDB_CLIENT_ID").expect("You nee to set the IGDB_CLIENT_ID variable");
        let token = env::var("IGDB_TOKEN").expect("You nee to set the IGDB_TOKEN variable");
        let igdb_client = IGDBClient::new(&client_id, &token);

        let games_client = igdb_client.games();
        let game = games_client
            .get_first_by_name("Always Sometimes Monsters")
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

        // name: RPG Maker VX Ace, url: https://www.igdb.com/game_engines/rpg-maker-vx-ace, companies: []
    })
}
