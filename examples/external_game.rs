use async_std::task;
use igdb_rs::client::IGDBClient;

fn main() {
    task::block_on(async {
        use std::env;
        let client_id =
            env::var("IGDB_CLIENT_ID").expect("You nee to set the IGDB_CLIENT_ID variable");
        let token = env::var("IGDB_TOKEN").expect("You nee to set the IGDB_TOKEN variable");
        let igdb_client = IGDBClient::new(&client_id, &token);

        let mut game_request =IGDBClient::create_request();
        game_request
            .all_fields()
            .contains("url", "https://store.steampowered.com/app/1869200");
        
        let external_games_client = igdb_client.external_games();

        let game = external_games_client.get(game_request)
            .await
            .unwrap();

        println!(
            "Game: {}",
            game[0].name
        );

        //Game: The Adventures of Mr. Hat
    })
}
