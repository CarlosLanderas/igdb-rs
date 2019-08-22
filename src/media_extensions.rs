use crate::client::ArtworksClient;
use crate::client::ScreenshotsClient;
use crate::request_builder::{RequestBuilder, Equality};
use crate::model::artwork::Artwork;
use crate::model::screenshot::Screenshot;
use crate::media_quality::MediaQuality;
use crate::endpoints::Endpoint::artworks;
use async_std::fs::File;
use async_std::io::{Result, Write};


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

        let mut parsed_url = match &artwork.url {
            url if !url.starts_with("http") => format!("{}{}", "http://", url),
            url => url.to_owned(),
        };

        parsed_url = parsed_url.replace("thumb", quality);
        download_resource(path.into(), parsed_url).await
    }
}

impl ScreenshotsClient {
    pub async fn get_by_game_id(&self, game_id: usize) -> Option<Vec<Screenshot>> {
        let mut request = RequestBuilder::new();
        request
            .all_fields()
            .add_where("id", Equality::Equal, game_id.to_string());

        let screenshot_response = self.get(request).await;

        match screenshot_response {
            Ok(screenshots) => Some(screenshots),
            Err(_) => None,
        }
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

