#![feature(async_await)]
use async_std::task;
use igdb_client::client::IGDBClient;
fn main() {
    task::block_on(async {

        let games_client = IGDBClient::new("586677e082e930d4c44a59962420e9d1").games();
        let game = games_client.get_by_name("Witcher 3")
            .await
            .unwrap();

        println!("Name: {}", game.name);
        println!("Summary: {} ...", &game.summary[..100]);

    })
}
