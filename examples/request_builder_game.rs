use async_std::task;
use igdb_rs::client::IGDBClient;
use igdb_rs::request_builder::{Equality, OrderBy};

fn main() {
    task::block_on(async {
        //Enable logging
        femme::with_level(log::LevelFilter::Debug);

        let igdb_client = IGDBClient::new("client_id", "token");

        let mut game_request = IGDBClient::create_request();
        game_request
            .add_field("name")
            .add_fields(vec!["storyline", "summary"])
            .contains("name", "Ast")
            .add_where("category", Equality::NotEqual, "0")
            .sort_by("name", OrderBy::Descending)
            .limit(3);

        // Generated query
        // fields name,storyline,summary; where name  ~ *"Ast"* & category != 0; sort name desc; limit 3;

        let game_client = igdb_client.games();
        let games = game_client.get(game_request).await.unwrap();

        for g in games {
            println!("Name: {}", g.name);
        }

        //Name: Yo-Kai Watch Blasters: Moon Rabbit Crew
        //Name: XCOM 2: Shen's Last Gift
        //Name: Warlock: Master of the Arcane - Armageddon
    })
}
