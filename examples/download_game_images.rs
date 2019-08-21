#![feature(async_await)]
use async_std::task;
use async_std::fs::File;
use igdb_client::client::IGDBClient;
use igdb_client::request_builder::Equality;
use std::path::PathBuf;
use futures::AsyncWriteExt;
use http::Uri;

async fn download_resource(path: &str, url: &str) {

    let parsed_url = match url {
        _ if !url.starts_with("http") => {
            format!("{}{}","http://", url)
        },
        _ => url.to_owned()
    };

    let content = surf::get(parsed_url).recv_bytes().await.unwrap();
    let mut file = File::create(path).await.unwrap();
    file.write(&content[..]).await.unwrap();
}

fn main() {
    task::block_on(async {

        let igdb_client = IGDBClient::new("user-key");
        let games_client = igdb_client.games();

        let game_req = igdb_client
            .games()
            .request()
            .add_field("id")
            .search("Modern Warfare 2");

        let game = games_client.get(&game_req).await.unwrap();

        let game_id = &game.first().unwrap().id.to_string();

        let covers_client = igdb_client.covers();
        let screenshots_client = igdb_client.screenshots();

        let scr_request = screenshots_client
            .request()
            .all_fields()
            .add_where("game", Equality::Equal, game_id);

        let cover_request = covers_client
            .request()
            .all_fields()
            .add_where("game", Equality::Equal, game_id);


        let covers = &screenshots_client.get(&scr_request).await.unwrap();
        let screens = &covers_client.get(&cover_request).await.unwrap();


        download_resource("cover.jpg", &covers.first().unwrap().url).await;
        download_resource("screen.jpg", &screens.first().unwrap().url).await;
    })
}
