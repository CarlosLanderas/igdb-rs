use async_std::task;
use igdb_client::client::IGDBClient;
use igdb_client::request_builder::Equality;

fn main() {
    task::block_on(async {
        let igdb_client = IGDBClient::new("586677e082e930d4c44a59962420e9d1");
        let videos_client = igdb_client.game_videos();

        //Query first 8 youtube videos for Witcher 3
        let response = videos_client.get_by_game_id(1942, 8).await.unwrap();

        for video in response {
            println!("{:?}", video);
        }
    })
}
