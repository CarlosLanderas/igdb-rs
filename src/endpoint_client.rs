use crate::endpoints::{get_endpoint_url, Endpoint};
use crate::request_builder::RequestBuilder;
use crate::Error;
use serde::de::DeserializeOwned;

pub(crate) struct EndpointClient {
    pub(crate) client_id: String,
    pub(crate) token: String,
    pub(crate) endpoint: Endpoint,
}

impl EndpointClient {
    pub(crate) fn new(client_id: String, token: String, endpoint: Endpoint) -> EndpointClient {
        EndpointClient {
            client_id,
            token,
            endpoint,
        }
    }

    pub(crate) async fn get<T: DeserializeOwned>(
        &self,
        request_builder: RequestBuilder,
    ) -> Result<Vec<T>, Error> {
        let request = request_builder.build(
            &self.client_id,
            &self.token,
            &get_endpoint_url(&self.endpoint),
        );
        let response = request.await;

        match response {
            Ok(resp) => Ok(resp.json().await?),
            Err(e) => {
                log::error!("{}", e);
                Err(Box::new(e))
            }
        }
    }
}
