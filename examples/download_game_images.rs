use async_std::task;
use igdb_rs::client::IGDBClient;
use igdb_rs::media_quality::MediaQuality;

fn main() {
    task::block_on(async {
        let igdb_client = IGDBClient::new("user-key");
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

        for (i, cover) in covers_response.iter().enumerate() {
            covers_client
                .download_by_id(cover.id, format!("cover{}.jpg", i), MediaQuality::CoverBig)
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
    })
}
