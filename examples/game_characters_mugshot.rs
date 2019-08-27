use async_std::task;
use igdb_rs::client::IGDBClient;
use igdb_rs::media_quality::MediaQuality;
use igdb_rs::request_builder::Equality;
use std::cmp::Ordering::Equal;

fn main() {
    task::block_on(async {
        let igdb_client = IGDBClient::new("user-key");
        let characters_client = igdb_client.characters();

        //Get characters for God of War: Ghost of Sparta and download mugshots
        let mut mug_shot_request = IGDBClient::create_request();
        mug_shot_request
            .add_fields(vec!["name", "mug_shot"])
            .add_where_in("games".to_string(), vec!["550".to_string()])
            .add_where("mug_shot", Equality::NotEqual, "null")
            .limit(10);

        let mugshots_client = igdb_client.character_mug_shots();

        for ch in characters_client.get(mug_shot_request).await.unwrap() {
            println!("{:?}", ch);
            mugshots_client
                .download_by_id(
                    ch.mug_shot as usize,
                    format!("{}.png", ch.name),
                    MediaQuality::Original,
                )
                .await
                .unwrap();
        }
    })
}
