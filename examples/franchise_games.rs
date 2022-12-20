use async_std::task;
use igdb_rs::client::IGDBClient;

fn main() {
    task::block_on(async {
        use std::{env, thread, time};
        let client_id = env::var("IGDB_CLIENT_ID").expect("You need to set the IGDB_CLIENT_ID variable");
        let token = env::var("IGDB_TOKEN").expect("You need to set the IGDB_TOKEN variable");
        let igdb_client = IGDBClient::new(&client_id, &token);
        let franchises_client = igdb_client.franchises();
        let games_client = igdb_client.games();

        for franchise in franchises_client.get_by_name("Lego", 5).await.unwrap() {
            for game in &franchise.games {
                let game_info = games_client.get_first_by_id(*game as usize).await.unwrap();
                println!("Name: {}", game_info.name);
                // avoid hitting the rate limits (4 request per second)
                thread::sleep(time::Duration::from_millis(300));
            }
        }

        // Name: Lego Indiana Jones 2: The Adventure Continues
        // Name: Lego Indiana Jones: The Original Adventures
        // Name: LEGO Star Wars II: The Original Trilogy
        // Name: Lego Racers 2
        // Name: LEGO Racers
        // Omitted for brevity...
    })
}
