use crate::client::GameClient;
use crate::model::games::Game;
use crate::request_builder::RequestBuilder;
use std::future::Future;

impl GameClient {
    pub async fn get_by_name<S: Into<String>>(&self, name: S) -> Option<Game> {
        let mut request = RequestBuilder::new();
        request.all_fields().contains("name", &name.into());

        get_game_result(self.get(request)).await

    }

    pub async fn contains<S: Into<String>>(&mut self, field: S, value: S) -> Option<Game> {
        let mut request = RequestBuilder::new();
        request.all_fields().contains(field, value).limit(1);

        get_game_result(self.get(request)).await
    }

    pub async fn search<S: Into<String>>(&self, name: S, limit: usize) -> Option<Game> {
        let mut request = RequestBuilder::new();
        request.all_fields().search(name).limit(limit);

        get_game_result(self.get(request)).await
    }
}

async fn get_game_result(
    games_future: impl Future<Output = Result<Vec<Game>, surf::Exception>>,
) -> Option<Game> {
    match games_future.await {
        Ok(games) => Some(games[0].clone()),
        Err(_) => None,
    }
}
