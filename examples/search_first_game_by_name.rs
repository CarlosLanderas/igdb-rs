use async_std::task;
use igdb_client::client::IGDBClient;
fn main() {
    task::block_on(async {
        let games_client = IGDBClient::new("586677e082e930d4c44a59962420e9d1").games();
        let game = games_client.get_first_by_name("Witcher 3").await.unwrap();

        println!("Name: {}", game.name);
        println!("Summary: {} ...", &game.summary[..150]);
        println!("Story line: {}", game.storyline);
        println!("Url: {}", game.url);

        //  Name: The Witcher 3: Wild Hunt - Hearts of Stone
        //  Summary: Hired by the Merchant of Mirrors, Geralt is tasked with overcoming Olgierd von Everec -- a ruthless bandit captain enchanted with the power of immorta ...
        //  Story line: Professional monster slayer is hired to deal with a ruthless bandit captain who possesses the power of immortality.
        //  Url: https://www.igdb.com/games/the-witcher-3-wild-hunt-hearts-of-stone
    })
}
