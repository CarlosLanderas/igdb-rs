use crate::{
    endpoint_client::EndpointClient,
    endpoints::{get_endpoint_url, Endpoint},
    model::company::Company,
    model::games::Game,
    request_builder::RequestBuilder,
};

use surf::Exception;

create_client!(GameClient, Game);
create_client!(CompanyClient, Company);

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
