use crate::client::CharactersClient;
use crate::model::character::Character;
use crate::request_builder::RequestBuilder;
use crate::Error;

impl CharactersClient {
    ///Retrieves the characters for a given game id
    ///
    /// # Examples
    /// ```
    /// use igdb_rs::client::IGDBClient;
    ///
    /// let igdb_client = IGDBClient::new("api-key");
    /// let characters_client = igdb_client.characters();
    /// ///Retrieve 8 first characters for The Witcher 3 game
    /// let characters = characters_client.get_by_game_id(1942, 8);
    /// ```
    pub async fn get_by_game_id(
        &self,
        game_id: usize,
        limit: usize,
    ) -> Result<Vec<Character>, Error> {
        let mut request = RequestBuilder::new();
        request
            .all_fields()
            .add_where_in("games".to_owned(), vec![game_id.to_string()])
            .limit(limit);

        self.get(request).await
    }
}
