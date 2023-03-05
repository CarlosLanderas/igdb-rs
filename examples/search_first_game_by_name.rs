use async_std::task;
use igdb_rs::client::IGDBClient;
fn main() {
    task::block_on(async {
        use std::env;
        let client_id =
            env::var("IGDB_CLIENT_ID").expect("You nee to set the IGDB_CLIENT_ID variable");
        let token = env::var("IGDB_TOKEN").expect("You nee to set the IGDB_TOKEN variable");
        let igdb_client = IGDBClient::new(&client_id, &token);

        let games_client = igdb_client.games();

        let game = games_client.get_first_by_name("Witcher 3").await.unwrap();

        println!("Name: {}", game.name);
        println!("Summary: {} ...", &game.summary);
        println!("Story line: {}", game.storyline);
        println!("Url: {}", game.url);

        //  Name: The Witcher 3: Wild Hunt - Hearts of Stone
        //  Summary: Hired by the Merchant of Mirrors, Geralt is tasked with overcoming Olgierd von Everec -- a ruthless bandit captain enchanted with the power of immorta ...
        //  Story line: Professional monster slayer is hired to deal with a ruthless bandit captain who possesses the power of immortality.
        //  Url: https://www.igdb.com/games/the-witcher-3-wild-hunt-hearts-of-stone
    })
}
