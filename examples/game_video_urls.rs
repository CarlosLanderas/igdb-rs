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
        //        GameVideo { id: 5993, game: 1942, video_id: "xQGam9OHSUo" }
        //        GameVideo { id: 5989, game: 1942, video_id: "_IBAovRNCuA" }
        //        GameVideo { id: 5995, game: 1942, video_id: "8ZLfJjlZKvc" }
        //        GameVideo { id: 5987, game: 1942, video_id: "5nLipy-Z4yo" }
        //        GameVideo { id: 5991, game: 1942, video_id: "6f8TbvsZ5Mk" }
        //        GameVideo { id: 5994, game: 1942, video_id: "p14dHAwLOmo" }
        //        GameVideo { id: 5990, game: 1942, video_id: "QrwGXAcE6ZA" }
        //        GameVideo { id: 5996, game: 1942, video_id: "sb81f-ejNSI" }
    })
}
