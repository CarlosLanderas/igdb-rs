use crate::client::{ArtworksClient, ScreenshotsClient, CoversClient};
use crate::request_builder::{RequestBuilder, Equality};
use crate::model::artwork::Artwork;
use crate::model::screenshot::Screenshot;
use crate::media_quality::MediaQuality;
use async_std::fs::File;
use async_std::io::{Result, Write};
use crate::model::cover::Cover;

impl ArtworksClient {
    pub async fn get_by_game_id(&self, game_id: usize) -> Option<Vec<Artwork>> {
        let mut request = RequestBuilder::new();
        request
            .all_fields()
            .add_where("game", Equality::Equal, game_id.to_string());

        let artworks_response = self.get(request).await;

        match artworks_response {
            Ok(aw) => Some(aw),
            Err(_) => None,
        }
    }

    pub async fn download_by_id<S : Into<String>>(&self, artwork_id : String, path: S, media_quality : MediaQuality) -> Result<()>  {
        let mut request = RequestBuilder::new();
        request
            .all_fields()
            .add_where("id", Equality::Equal, artwork_id.to_string());

        //TODO -> Implement std::ops::Try trait in macro clients
        let artworks_response = self.get(request).await.unwrap();
        let artwork = artworks_response.first().unwrap();

        let quality = media_quality.get_value();

        download_resource(path.into(), artwork.url.clone()).await
    }
}

impl ScreenshotsClient {
    pub async fn get_by_game_id(&self, game_id: usize) -> Option<Vec<Screenshot>> {
        let mut request = RequestBuilder::new();
        request
            .all_fields()
            .add_where("game", Equality::Equal, game_id.to_string());

        let screenshot_response = self.get(request).await;

        match screenshot_response {
            Ok(screenshots) => Some(screenshots),
            Err(_) => None,
        }
    }

    pub async fn download_by_id<S : Into<String>>(&self, screenshot_id : String, path: S, media_quality : MediaQuality) -> Result<()>  {
        let mut request = RequestBuilder::new();
        request
            .all_fields()
            .add_where("id", Equality::Equal, screenshot_id.to_string());

        //TODO -> Implement std::ops::Try trait in macro clients
        let screen_response = self.get(request).await.unwrap();
        let screenshot = screen_response.first().unwrap();

        let quality = media_quality.get_value();


        download_resource(path.into(), screenshot.url.clone()).await
    }
}

impl CoversClient {

    pub async fn get_by_game_id(&self, game_id: usize) -> Option<Vec<Cover>> {
        let mut request = RequestBuilder::new();
        request
            .all_fields()
            .add_where("game", Equality::Equal, game_id.to_string());

        let covers_response = self.get(request).await;

        match covers_response {
            Ok(covers) => Some(covers),
            Err(_) => None,
        }
    }

    pub async fn download_by_id<S : Into<String>>(&self, cover_id : String, path: S, media_quality : MediaQuality) -> Result<()>  {
        let mut request = RequestBuilder::new();
        request
            .all_fields()
            .add_where("id", Equality::Equal, cover_id.to_string());

        //TODO -> Implement std::ops::Try trait in macro clients
        let covers_response = self.get(request).await.unwrap();
        let cover = covers_response.first().unwrap();

        let quality = media_quality.get_value();

        download_resource(path.into(), cover.url.clone()).await
    }
}

async fn download_resource(path: String, url: String) -> Result<()> {

    let mut parsed_url = match url {
        _ if !url.starts_with("http") => format!("{}{}", "http://", url),
        _ => url.to_owned(),
    };

    parsed_url = parsed_url.replace("thumb", "screenshot_med");

    let content = surf::get(parsed_url).recv_bytes().await.unwrap();
    let mut file = File::create(path).await?;
    file.write(&content[..]).await.unwrap();

    Ok(())
}

