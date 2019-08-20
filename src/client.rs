use crate::{
    endpoint_client::EndpointClient,
    endpoints::{get_endpoint_url, Endpoint},
    model::company::Company,
    model::games::Game,
    request_builder::RequestBuilder,
};

use surf::Exception;

pub struct IGDBClient {
    api_key: String,
}

impl IGDBClient {
    pub fn new<S: Into<String>>(api_key: S) -> IGDBClient {
        IGDBClient {
            api_key: api_key.into(),
        }
    }

    pub fn games(&self) -> GameClient {
        GameClient {
            endpoint_client: EndpointClient::new(self.api_key.clone(), Endpoint::Games),
        }
    }

    pub fn companies(&self) -> CompanyClient {
        CompanyClient {
            endpoint_client: EndpointClient::new(self.api_key.clone(), Endpoint::Companies),
        }
    }
}

pub struct GameClient {
    endpoint_client: EndpointClient,
}

impl GameClient {
    pub fn request(&self) -> RequestBuilder {
        RequestBuilder::new(
            self.endpoint_client.api_key.clone().into(),
            get_endpoint_url(&self.endpoint_client.endpoint),
        )
    }
    pub async fn get(&self, request_builder: &RequestBuilder) -> Result<Vec<Game>, Exception> {
        self.endpoint_client.get::<Game>(&request_builder).await
    }
}

pub struct CompanyClient {
    endpoint_client: EndpointClient,
}

impl CompanyClient {
    pub fn request(&self) -> RequestBuilder {
        RequestBuilder::new(
            self.endpoint_client.api_key.clone().into(),
            get_endpoint_url(&self.endpoint_client.endpoint),
        )
    }
    pub async fn get(&self, request_builder: &RequestBuilder) -> Result<Vec<Company>, Exception> {
        self.endpoint_client.get::<Company>(&request_builder).await
    }
}
