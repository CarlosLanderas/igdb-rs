#![feature(async_await)]
use async_std::fs::File;
use async_std::task;
use futures::AsyncWriteExt;
use http::Uri;
use igdb_client::client::IGDBClient;
use igdb_client::request_builder::Equality;
use igdb_client::media_quality::MediaQuality;


fn main() {
    task::block_on(async {

        let igdb_client = IGDBClient::new("586677e082e930d4c44a59962420e9d1");
        let games_client = igdb_client.games();
        let witcher = games_client.get_by_name("Witcher 3").await.unwrap();

        let covers_client = igdb_client.covers();

        let covers_response = covers_client.get_by_game_id(witcher.id).await.unwrap();
        let first_cover = covers_response.first().unwrap();

        let screenshots_client = igdb_client.screenshots();
        let screenshots_response = screenshots_client.get_by_game_id(witcher.id).await.unwrap();
        let first_screenshot = screenshots_response.first().unwrap();

        covers_client.download_by_id(first_cover.id.to_string(), "cover.jpg", MediaQuality::ScreenshotHuge).await;
        screenshots_client.download_by_id(first_screenshot.id.to_string(), "screenshot.jpg", MediaQuality::ScreenshotHuge).await;
    })
}
