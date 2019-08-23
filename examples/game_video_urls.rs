#![feature(async_await)]
use async_std::task;
use igdb_client::client::IGDBClient;
use igdb_client::request_builder::Equality;

fn main() {
    task::block_on(async {

        let igdb_client = IGDBClient::new("user-key");

        let videos_client = igdb_client.game_videos();

        let mut request = IGDBClient::create_request();

        //Query all youtube videos for Witcher 3
        let video_request = request
                .all_fields()
                .add_where("game", Equality::Equal, "1942")
                .limit(10);

        for video in videos_client.get(request).await.unwrap() {
            println!("{:?}", video);
        }

    })
}