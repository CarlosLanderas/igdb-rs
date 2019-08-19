use crate::endpoints::Endpoint;
use crate::request_builder::RequestBuilder;
use serde::de::DeserializeOwned;

pub(crate) struct EndpointClient {
    pub(crate) api_key: String,
    pub(crate) endpoint: Endpoint,
}

impl EndpointClient {
    pub fn new(api_key: String, endpoint: Endpoint) -> EndpointClient {
        EndpointClient { api_key, endpoint }
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
}
