use async_std::task;
use igdb_rs::client::IGDBClient;
use igdb_rs::media_quality::MediaQuality;

fn main() {
    femme::start(log::LevelFilter::Debug).unwrap();

    task::block_on(async {
        let igdb_client = IGDBClient::new("user-key");

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
