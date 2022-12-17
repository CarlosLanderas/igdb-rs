use async_std::task;
use igdb_rs::client::IGDBClient;
use log::LevelFilter;

fn main() {
    femme::with_level(LevelFilter::Debug);

    task::block_on(async {
        let igdb_client = IGDBClient::new("client_id", "token");
        let games_client = igdb_client.games();
        let age_rating_client = igdb_client.age_ratings();

        let game = games_client
            .get_first_by_name("Call of Duty: Modern Warfare 3")
            .await
            .unwrap();

        for age_rating in game.age_ratings {
            //Get a maximum of 3 age ratings for Modern Warfare 3

            let ratings = age_rating_client
                .get_by_id(age_rating as usize, 3)
                .await
                .unwrap();

            for rating in ratings {
                println!(
                    "Game: {}, Category: {:?}, Rating: {:?}",
                    game.name, rating.category, rating.rating
                );
            }

            // Game: Call of Duty: Modern Warfare 3, Category: PEGI, Rating: Eighteen
        }
    })
}
