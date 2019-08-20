macro_rules! create_client {
    ($i: ident, $j: ident, $k: ident) => {
        pub struct $i {
            endpoint_client: EndpointClient,
        }

        impl $i {
            pub fn request(&self) -> RequestBuilder {
                RequestBuilder::new(
                    self.endpoint_client.api_key.clone().into(),
                    get_endpoint_url(&self.endpoint_client.endpoint),
                )
            }

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
