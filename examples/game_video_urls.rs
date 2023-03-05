use async_std::task;
use igdb_rs::client::IGDBClient;

fn main() {
    task::block_on(async {
        use std::env;
        let client_id =
            env::var("IGDB_CLIENT_ID").expect("You nee to set the IGDB_CLIENT_ID variable");
        let token = env::var("IGDB_TOKEN").expect("You nee to set the IGDB_TOKEN variable");
        let igdb_client = IGDBClient::new(&client_id, &token);

        let videos_client = igdb_client.game_videos();

        //Query first 8 youtube videos for Witcher 3
        let response = videos_client.get_by_game_id(1942, 8).await.unwrap();

        for video in response {
            println!("{:?}", video);
        }

        //  GameVideo { id: 5993, game: 1942, video_id: "xQGam9OHSUo" }
        //  GameVideo { id: 5989, game: 1942, video_id: "_IBAovRNCuA" }
        //  GameVideo { id: 5995, game: 1942, video_id: "8ZLfJjlZKvc" }
        //  GameVideo { id: 5987, game: 1942, video_id: "5nLipy-Z4yo" }
        //  GameVideo { id: 5991, game: 1942, video_id: "6f8TbvsZ5Mk" }
        //  GameVideo { id: 5994, game: 1942, video_id: "p14dHAwLOmo" }
        //  GameVideo { id: 5990, game: 1942, video_id: "QrwGXAcE6ZA" }
        //  GameVideo { id: 5996, game: 1942, video_id: "sb81f-ejNSI" }
    })
}
