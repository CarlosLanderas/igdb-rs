use crate::{
    endpoint_client::EndpointClient,
    endpoints::{get_endpoint_url, Endpoint},
    model::artwork::Artwork,
    model::company::Company,
    model::games::Game,
    model::website::Website,
    request_builder::RequestBuilder,
};

use surf::Exception;

//create_client! macro automatically generates clients
//for different endpoints and extends IGDBClient struct

//params: (ClientName, EntityResult, IGDB client impl method)

create_client!(GameClient, Game, games);
create_client!(CompanyClient, Company, companies);
create_client!(WebsitesClient, Website, websites);
create_client!(ArtworksClient, Artwork, artworks);

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
