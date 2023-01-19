use async_std::task;
use igdb_rs::client::IGDBClient;
use igdb_rs::media_quality::MediaQuality;
use std::io::Write;

fn main() {
    task::block_on(async {
        // All media clients like screenshots, covers, artworks, etc can read resources as bytes
        // by using get_resource_by_id method
        // Yoy can also directly download them to disk, (game_media_download sample)
        use std::env;
        let client_id =
            env::var("IGDB_CLIENT_ID").expect("You nee to set the IGDB_CLIENT_ID variable");
        let token = env::var("IGDB_TOKEN").expect("You nee to set the IGDB_TOKEN variable");
        let igdb_client = IGDBClient::new(&client_id, &token);

        let games_client = igdb_client.games();
        let witcher = games_client.get_first_by_name("Witcher 3").await.unwrap();

        //Get the first 3 screenshots for the Witcher 3 game
        let screenshots_client = igdb_client.screenshots();
        let screenshots_response = screenshots_client
            .get_by_game_id(witcher.id, 3)
            .await
            .unwrap();

        for screen in screenshots_response {
            let bytes = screenshots_client
                .get_resource_by_id(screen.id, MediaQuality::FullHD)
                .await
                .unwrap();

            debug_assert!(bytes.len() > 0);

            let mut f = std::fs::File::create(format!("{}.jpg", screen.id)).unwrap();
            f.write_all(&bytes).unwrap();
        }
    })
}
