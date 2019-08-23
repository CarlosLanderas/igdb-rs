use async_std::task;
use igdb_client::client::IGDBClient;

fn main() {
    task::block_on(async {
        let igdb_client = IGDBClient::new("586677e082e930d4c44a59962420e9d1");

        let release_client = igdb_client.release_dates();
        let releases = release_client.get_by_game_id(1942, 10).await.unwrap();

        let platform_client = igdb_client.platforms();

        for release in releases {
            let platform = platform_client
                .get_first_by_id(release.platform as usize)
                .await
                .unwrap();

            println!(
                "plataform: {} release date: {}",
                platform.name, release.human
            );
        }
    })
}
