use async_std::task;
use igdb_rs::client::IGDBClient;
use igdb_rs::media_quality::MediaQuality;

fn main() {
    femme::with_level(log::LevelFilter::Debug);

    task::block_on(async {
        use std::env;
        let client_id = env::var("IGDB_CLIENT_ID").expect("You nee to set the IGDB_CLIENT_ID variable");
        let token = env::var("IGDB_TOKEN").expect("You nee to set the IGDB_TOKEN variable");
        let igdb_client = IGDBClient::new(&client_id, &token);

        let games_client = igdb_client.games();
        let game = games_client
            .get_first_by_name("Borderlands 2")
            .await
            .unwrap();

        let platforms_client = igdb_client.platforms();
        let platform_logos_client = igdb_client.platform_logos();

        for p_id in game.platforms {
            let platform = platforms_client
                .get_first_by_id(p_id as usize)
                .await
                .unwrap();

            platform_logos_client
                .download_by_id(
                    platform.platform_logo,
                    format!("{}.png", platform.name),
                    MediaQuality::Original,
                )
                .await
                .unwrap();
        }
    })
}
