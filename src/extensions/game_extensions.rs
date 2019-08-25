use crate::client::GameClient;
use crate::model::games::Game;
use crate::request_builder::RequestBuilder;
use std::future::Future;

impl GameClient {
    ///Returns a games collection containing the given name
    pub async fn get_by_name<S: Into<String>>(
        &self,
        name: S,
        limit: usize,
    ) -> Result<Vec<Game>, surf::Exception> {
        let mut request = RequestBuilder::new();
        request
            .all_fields()
            .contains("name", &name.into())
            .limit(limit);

        self.get(request).await
    }
    ///Returns the first game found inside a collection for a given name
    pub async fn get_first_by_name<S: Into<String>>(&self, name: S) -> Option<Game> {
        get_game_result(self.get_by_name(name, 1)).await
    }
    /// Returns a games collection containing the specified field value
    pub async fn contains<S: Into<String>>(
        &mut self,
        field: S,
        value: S,
        limit: usize,
    ) -> Result<Vec<Game>, surf::Exception> {
        let mut request = RequestBuilder::new();
        request.all_fields().contains(field, value).limit(limit);

        self.get(request).await
    }

    ///Returns a games collection searching by the given name
    pub async fn search<S: Into<String>>(
        &self,
        name: S,
        limit: usize,
    ) -> Result<Vec<Game>, surf::Exception> {
        let mut request = RequestBuilder::new();
        request.all_fields().search(name).limit(limit);

        self.get(request).await
    }
}

async fn get_game_result(
    games_future: impl Future<Output = Result<Vec<Game>, surf::Exception>>,
) -> Option<Game> {
    match games_future.await {
        Ok(games) => Some(games[0].clone()),
        Err(e) => {
            log::error!("{}", e);
            None
        }
    }
}
