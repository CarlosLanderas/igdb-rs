use async_std::task;
use igdb_rs::client::IGDBClient;
use log::LevelFilter;

fn main() {
    task::block_on(async {
        //Set default level to debug using femme crate
        femme::with_level(LevelFilter::Debug);

        let games_client = IGDBClient::new("client_id", "token").games();
        let games_results = games_client.get_by_name("Borderlands", 10).await.unwrap();

        for game in games_results {
            println!("Name: {}", game.name);
            println!("Story line: {}", game.storyline);
            println!("Url: {}", game.url);
        }
    })
}
