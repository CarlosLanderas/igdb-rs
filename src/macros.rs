macro_rules! create_client {
    ($i: ident, $j: ident, $k: ident) => {
        pub struct $i {
            endpoint_client: EndpointClient,
        }

        impl $i {
            pub async fn get(
                &self,
                request_builder: &RequestBuilder,
            ) -> Result<Vec<$j>, Exception> {
                self.endpoint_client.get::<$j>(&request_builder).await
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

macro_rules! use_client_imports {
    () => {
        use crate::{
            endpoint_client::EndpointClient,
            endpoints::{get_endpoint_url, Endpoint},
            model::artwork::Artwork,
            model::character::Character,
            model::company::Company,
            model::cover::Cover,
            model::game_mode::GameMode,
            model::games::Game,
            model::multiplayer_mode::MultiplayerMode,
            model::release_date::ReleaseDate,
            model::screenshot::Screenshot,
            model::website::Website,
            request_builder::RequestBuilder,
        };
    };
}
