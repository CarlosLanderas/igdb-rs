use crate::client::{ArtworksClient, CoversClient, ScreenshotsClient};
use crate::model::artwork::Artwork;
use crate::model::cover::Cover;
use crate::model::screenshot::Screenshot;
use crate::request_builder::{Equality, RequestBuilder};
use std::string::ToString;

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
}
