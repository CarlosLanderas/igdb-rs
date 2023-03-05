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
