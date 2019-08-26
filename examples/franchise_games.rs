use async_std::task;
use igdb_rs::client::IGDBClient;

fn main() {
    task::block_on(async {
        let igdb_client = IGDBClient::new("user-key");
        let franchises_client = igdb_client.franchises();
        let games_client = igdb_client.games();

        for franchise in franchises_client.get_by_name("Lego", 5).await.unwrap() {
            for game in &franchise.games {
                let game_info = games_client.get_first_by_id(*game as usize).await.unwrap();

                println!("Name: {}", game_info.name);
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
