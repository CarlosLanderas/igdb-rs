use crate::client::GameClient;
use crate::model::games::Game;
use crate::request_builder::Equality;
use crate::request_builder::RequestBuilder;
use std::future::Future;

async fn get_game_result(games_future :  impl Future<Output=Result<Vec<Game>,surf::Exception>>) -> Option<Game> {
    match games_future.await {
        Ok(games) => Some(games[0].clone()),
        Err(_) => None,
    }
}

impl GameClient {
    pub async fn get_by_id(&self, id: usize) -> Option<Game> {
        let mut request = RequestBuilder::new();
        request
            .all_fields()
            .add_where("id", Equality::Equal, id.to_string());

        get_game_result(self.get(request)).await
    }

    pub async fn get_by_name<S: Into<String>>(&self, name: S) -> Option<Game> {
        let mut request = RequestBuilder::new();
        request
            .all_fields()
            .add_where("name", Equality::Equal, name);

        get_game_result(self.get(request)).await
    }
}
