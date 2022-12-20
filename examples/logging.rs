use async_std::task;
use igdb_rs::client::IGDBClient;
use log::LevelFilter;

fn main() {
    task::block_on(async {
        use std::env;
        //Set default level to debug using femme crate
        femme::with_level(LevelFilter::Debug);
        let client_id = env::var("IGDB_CLIENT_ID").expect("You nee to set the IGDB_CLIENT_ID variable");
        let token = env::var("IGDB_TOKEN").expect("You nee to set the IGDB_TOKEN variable");
        let igdb_client = IGDBClient::new(&client_id, &token);
        let games_client = igdb_client.games();

        let games_results = games_client.get_by_name("Borderlands", 10).await.unwrap();

        for game in games_results {
            println!("Name: {}", game.name);
            println!("Story line: {}", game.storyline);
            println!("Url: {}", game.url);
        }
    })
}
