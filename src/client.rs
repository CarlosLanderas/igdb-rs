use crate::{
    endpoint_client::EndpointClient,
    endpoints::{get_endpoint_url, Endpoint},
    model::company::Company,
    model::games::Game,
    request_builder::RequestBuilder,
};

use surf::Exception;

//create_client! macro automatically generates clients 
//for different endpoints and extends IGDBClient struct
//Params (client name, entity type, IGDB client function)

create_client!(GameClient, Game, games);
create_client!(CompanyClient, Company, companies);

pub struct IGDBClient {
    api_key: String,
}

impl IGDBClient {
    pub fn new<S: Into<String>>(api_key: S) -> IGDBClient {
        IGDBClient {
            api_key: api_key.into(),
        }
    }
}
