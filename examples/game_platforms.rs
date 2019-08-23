use async_std::task;
use igdb_client::client::IGDBClient;
fn main() {
    task::block_on(async {
        let igdb_client = IGDBClient::new("586677e082e930d4c44a59962420e9d1");

        let games_client = igdb_client.games();
        let game = games_client.get_first_by_name("Witcher 3").await.unwrap();

        let platforms_client = igdb_client.platforms();

        let mut game_platforms = vec![];
        for p_id in game.platforms {
            game_platforms.push(
                platforms_client
                    .get_first_by_id(p_id as usize)
                    .await
                    .unwrap(),
            );
        }

        for platform in game_platforms {
            println!("name: {}, url: {}", platform.name, platform.url);
        }

        //  name: PC (Microsoft Windows), url: https://www.igdb.com/platforms/win
        //  name: PlayStation Network, url: https://www.igdb.com/platforms/psn
        //  name: PlayStation 4, url: https://www.igdb.com/platforms/ps4--1
        //  name: Xbox One, url: https://www.igdb.com/platforms/xboxone
    })
}
