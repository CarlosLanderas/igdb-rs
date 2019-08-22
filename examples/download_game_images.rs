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

        let artwork_client = igdb_client.artworks();

        let artworks_response = artwork_client.get_by_game_id(witcher.id).await.unwrap();
        let first_art = artworks_response.first().unwrap();

        artwork_client.download_by_id(first_art.id.to_string(), "artwork.jpg", MediaQuality::HD).await;
    })
}
