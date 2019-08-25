use crate::endpoints::{get_endpoint_url, Endpoint};
use crate::request_builder::RequestBuilder;
use serde::de::DeserializeOwned;
use surf::Exception;

pub(crate) struct EndpointClient {
    pub(crate) api_key: String,
    pub(crate) endpoint: Endpoint,
}

impl EndpointClient {
    pub(crate) fn new(api_key: String, endpoint: Endpoint) -> EndpointClient {
        EndpointClient { api_key, endpoint }
    }

    pub(crate) async fn get<T: DeserializeOwned>(
        &self,
        request_builder: RequestBuilder,
    ) -> Result<Vec<T>, Exception> {
        let request = request_builder.build(&self.api_key, &get_endpoint_url(&self.endpoint));
        let mut response = request.await;

        match response {
            Ok(ref mut resp) => {
                let response_str: String = resp.body_string().await.unwrap();
                Ok(serde_json::from_str::<Vec<T>>(&response_str).unwrap())
            }
            Err(e) => {
                eprintln!("{}", e);
                Err(e)
            }
        }
    }
}
