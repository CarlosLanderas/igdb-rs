use crate::v3::endpoints::{get_endpoint_url, Endpoint};
use crate::v3::model::games::GameResponse;
use crate::v3::request_builder::{Equality, RequestBuilder};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::borrow::{Borrow, BorrowMut};
use surf::middleware::HttpClient;

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

    pub fn request(&self, endpoint: Endpoint) -> RequestBuilder {
        RequestBuilder::new(self.api_key.clone().into(), get_endpoint_url(endpoint))
    }

    pub async fn get<T: DeserializeOwned>(&self, request_builder: &RequestBuilder) -> Vec<T> {
        let request = request_builder.build();
        let url = request.url().clone().into_string();

        let response_str = request.recv_string().await.unwrap();
        for (i, l) in response_str.lines().enumerate() {
            println!("{}:{}", i, l);
        }
        serde_json::from_str::<Vec<T>>(&response_str).unwrap()
    }

    //    pub async fn get_single<T: DeserializeOwned>(&self, request_builder : &RequestBuilder) ->  T {
    //
    //        let result : Vec<T> = self.get(request_builder).await;
    //        result.first().unwrap()
    //    }
}
