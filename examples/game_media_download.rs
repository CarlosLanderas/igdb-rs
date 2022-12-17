use async_std::task;
use igdb_rs::client::IGDBClient;
use igdb_rs::media_quality::MediaQuality;

fn main() {
    task::block_on(async {
        // All media clients like screenshots, covers, artworks, etc can be downloaded to disk
        // by using download_by_id method
        // Yoy can also read them as a bytes buffer, (game_media_read sample)

        let igdb_client = IGDBClient::new("client_id", "token");
        let games_client = igdb_client.games();
        let witcher = games_client.get_first_by_name("Witcher 3").await.unwrap();

        //Get the first 3 covers for the Witcher 3 game
        let covers_client = igdb_client.covers();
        let covers_response = covers_client.get_by_game_id(witcher.id, 3).await.unwrap();

        //Get the first 3 screenshots for the Witcher 3 game
        let screenshots_client = igdb_client.screenshots();
        let screenshots_response = screenshots_client
            .get_by_game_id(witcher.id, 3)
            .await
            .unwrap();

        //Get the first 3 artworks for the Witcher 3 game
        let artworks_client = igdb_client.artworks();
        let artworks_response = artworks_client.get_by_game_id(witcher.id, 3).await.unwrap();

        for (i, cover) in covers_response.iter().enumerate() {
            covers_client
                .download_by_id(cover.id, format!("cover{}.jpg", i), MediaQuality::Original)
                .await
                .unwrap();
        }

        for (i, screenshot) in screenshots_response.iter().enumerate() {
            screenshots_client
                .download_by_id(
                    screenshot.id,
                    format!("screenshot{}.jpg", i),
                    MediaQuality::ScreenshotHuge,
                )
                .await
                .unwrap();
        }

        for (i, artwork) in artworks_response.iter().enumerate() {
            artworks_client
                .download_by_id(artwork.id, format!("artwork{}.jpg", i), MediaQuality::HD)
                .await
                .unwrap();
        }
    })
}
