use async_std::task;
use igdb_rs::client::IGDBClient;

fn main() {
    task::block_on(async {
        use std::env;
        let client_id =
            env::var("IGDB_CLIENT_ID").expect("You nee to set the IGDB_CLIENT_ID variable");
        let token = env::var("IGDB_TOKEN").expect("You nee to set the IGDB_TOKEN variable");
        let igdb_client = IGDBClient::new(&client_id, &token);
        let characters_client = igdb_client.characters();

        for ch in characters_client.get_by_game_id(1942, 10).await.unwrap() {
            println!("name: {}, slug: {}, url: {}", ch.name, ch.slug, ch.url);
        }

        //name: Dandelion, slug: dandelion, url: https://www.igdb.com/characters/dandelion
        //name: Jaskier, slug: jaskier, url: https://www.igdb.com/characters/jaskier
        // name: Emhyr Var Empreis, slug: emhyr-var-empreis, url: https://www.igdb.com/characters/emhyr-var-empreis
        //name: Ciri, slug: ciri, url: https://www.igdb.com/characters/ciri
        //name: Avallac'h, slug: avallach, url: https://www.igdb.com/characters/avallach
        //name: Trolls, slug: trolls, url: https://www.igdb.com/characters/trolls
        //name: Dijkstra, slug: dijkstra, url: https://www.igdb.com/characters/dijkstra
        //name: Zoltan, slug: zoltan, url: https://www.igdb.com/characters/zoltan
        //name: Crache An Craite, slug: crache-an-craite, url: https://www.igdb.com/characters/crache-an-craite
        //name: Hjalmar An Craite, slug: hjalmar-an-craite, url: https://www.igdb.com/characters/hjalmar-an-craite
    })
}
