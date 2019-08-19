use crate::v3::request_builder::RequestBuilder;

pub mod endpoints;
pub mod model;
pub mod request_builder;

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
