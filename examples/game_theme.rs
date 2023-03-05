use async_std::task;
use igdb_rs::client::IGDBClient;

fn main() {
    task::block_on(async {
        use std::env;
        let client_id =
            env::var("IGDB_CLIENT_ID").expect("You nee to set the IGDB_CLIENT_ID variable");
        let token = env::var("IGDB_TOKEN").expect("You nee to set the IGDB_TOKEN variable");
        let igdb_client = IGDBClient::new(&client_id, &token);

        let games_client = igdb_client.games();

        let game = games_client
            .get_first_by_name("Battleground 7: Bull Run")
            .await
            .unwrap();

        let themes_client = igdb_client.themes();

        for theme in game.themes {
            let results = themes_client.get_by_id(theme as usize, 10).await.unwrap();
            for r in results {
                println!("{:?}", r);
            }
        }

        //Theme { id: 22, created_at: 1322524800, name: "Historical", slug: "historical", url: "https://www.igdb.com/themes/historical" }
        //Theme { id: 39, created_at: 1345420800, name: "Warfare", slug: "warfare",  url: "https://www.igdb.com/themes/warfare" }
    })
}
