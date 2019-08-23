use async_std::task;
use igdb_client::client::IGDBClient;
use igdb_client::media_quality::MediaQuality;

fn main() {
    task::block_on(async {
        let igdb_client = IGDBClient::new("586677e082e930d4c44a59962420e9d1");
        let games_client = igdb_client.games();
        let witcher = games_client.get_by_name("Minecraft").await.unwrap();

        let covers_client = igdb_client.covers();
        let covers_response = covers_client.get_by_game_id(witcher.id, 1).await.unwrap();

        let cover = covers_response.first().unwrap();

        let screenshots_client = igdb_client.screenshots();
        let screenshots_response = screenshots_client.get_by_game_id(witcher.id, 1).await.unwrap();

        let screenshot = screenshots_response.first().unwrap();

        covers_client
            .download_by_id(
                cover.id.to_string(),
                "cover.jpg",
                MediaQuality::ScreenshotHuge,
            )
            .await
            .unwrap();

        screenshots_client
            .download_by_id(
                screenshot.id.to_string(),
                "screenshot.jpg",
                MediaQuality::ScreenshotHuge,
            )
            .await
            .unwrap();
    })
}
