use crate::client::CharacterClient;
use crate::model::character::Character;
use crate::request_builder::RequestBuilder;

impl CharacterClient {
    pub async fn get_by_game_id(
        &self,
        game_id: usize,
        limit: usize,
    ) -> Result<Vec<Character>, surf::Exception> {
        let mut request = RequestBuilder::new();
        request
            .all_fields()
            .add_where_in("games".to_owned(), vec![game_id.to_string()])
            .limit(limit);

        self.get(request).await
    }
}
