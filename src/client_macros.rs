macro_rules! create_client {
    ($i: ident, $j: ident, $k: ident) => {
        pub struct $i {
            endpoint_client: EndpointClient,
        }

        impl $i {
            pub async fn get(&self, request_builder: RequestBuilder) -> Result<Vec<$j>, Exception> {
                self.endpoint_client.get::<$j>(request_builder).await
            }
            pub async fn get_by_id(&self, id: usize, limit : usize) -> Option<$j> {
                let mut request = RequestBuilder::new();
                request
                    .all_fields()
                    .add_where("id", Equality::Equal, id.to_string())
                    .limit(1);

                match self.get(request).await {
                    Ok(res) => Some(res[0].clone()),
                    Err(_) => None,
                }
            }
        }

        impl IGDBClient {
            pub fn $k(&self) -> $i {
                $i {
                    endpoint_client: EndpointClient::new(self.api_key.clone(), Endpoint::$k),
                }
            }
        }
    };
}

macro_rules! expand_get_by_game_id {
    ($i: ident, $j: ident) => {
      impl $i {
          pub async fn get_by_game_id(&self, game_id: usize, limit : usize) -> Option<Vec<$j>> {
            let mut request = RequestBuilder::new();
                request
                .all_fields()
                .add_where("game", Equality::Equal, game_id.to_string())
                .limit(limit);

            match self.get(request).await {
              Ok(d) => Some(d),
              Err(_) => None,
            }

         }
      }
   };
}

#[allow(unused_macros)]
macro_rules! request {
    () => {
        IGDBClient::create_request()
    };
}

macro_rules! use_client_imports {
    () => {
        use crate::{
            endpoint_client::EndpointClient, endpoints::Endpoint, media_quality::MediaQuality,
            model::artwork::Artwork, model::character::Character, model::company::Company,
            model::cover::Cover, model::engine::Engine, model::game_mode::GameMode,
            model::game_video::GameVideo, model::games::Game,
            model::multiplayer_mode::MultiplayerMode, model::release_date::ReleaseDate,
            model::screenshot::Screenshot, model::website::Website, request_builder::Equality,
            request_builder::RequestBuilder,
        };
    };
}
