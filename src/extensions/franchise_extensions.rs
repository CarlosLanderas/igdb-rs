use crate::client::FranchisesClient;
use crate::model::franchise::Franchise;
use crate::request_builder::RequestBuilder;

impl FranchisesClient {
    pub async fn get_by_name<S: Into<String>>(
        &self,
        name: S,
        limit: usize,
    ) -> Result<Vec<Franchise>, surf::Exception> {
        let mut request = RequestBuilder::new();
        request
            .all_fields()
            .contains("name", &name.into())
            .limit(limit);

        self.get(request).await
    }
}
