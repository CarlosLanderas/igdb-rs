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
            .get_first_by_name("Modern Warfare 3")
            .await
            .unwrap();

        println!(
            "Game: {}, rating: {}, total votes: {}",
            game.name, game.total_rating as usize, game.total_rating_count
        );

        //Game: Call of Duty: Modern Warfare 3, rating: 80, total votes: 442
    })
}
