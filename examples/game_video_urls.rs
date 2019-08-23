use async_std::task;
use igdb_client::client::IGDBClient;
use igdb_client::request_builder::Equality;

fn main() {
    task::block_on(async {
        let igdb_client = IGDBClient::new("user-key");
        let videos_client = igdb_client.game_videos();

        //Query first 8 youtube videos for Witcher 3
        let response = videos_client.get_by_game_id(1942, 8).await.unwrap();

        for video in response {
            println!("{:?}", video);
        }
    })
}
