use async_std::task;
use igdb_rs::client::IGDBClient;
fn main() {
    task::block_on(async {
        let games_client = IGDBClient::new("586677e082e930d4c44a59962420e9d1").games();
        let games_results = games_client.get_by_name("Borderlands", 10).await.unwrap();

        for game in games_results {
            println!("Name: {}", game.name);
            println!("Story line: {}", game.storyline);
            println!("Url: {}", game.url);
        }

        //        Name: Borderlands: The Pre-Sequel - Lady Hammerlock The Baroness
        //        Story line:
        //            Url: https://www.igdb.com/games/borderlands-the-pre-sequel-lady-hammerlock-the-baroness
        //            Name: Borderlands Legends
        //        Story line:
        //            Url: https://www.igdb.com/games/borderlands-legends
        //            Name: Tales from the Borderlands: Episode 3 - Catch a Ride
        //        Story line:
        //            Url: https://www.igdb.com/games/tales-from-the-borderlands-episode-3-catch-a-ride
        //            Name: Borderlands 2: Game of the Year Edition
        //        Story line:
        //            Url: https://www.igdb.com/games/borderlands-2-game-of-the-year-edition
        //            Name: Tales from the Borderlands: Episode 2 - Atlas Mugged
        //        Story line:
        //            Url: https://www.igdb.com/games/tales-from-the-borderlands-episode-2-atlas-mugged
        //            Name: Borderlands 2 VR
        //        Story line:
        //            Url: https://www.igdb.com/games/borderlands-2-vr
        //            Name: Borderlands Legends HD
        //        Story line:
        //            Url: https://www.igdb.com/games/borderlands-legends-hd
        //            Name: Borderlands: The Pre-Sequel - The Holodome Onslaught
        //        Story line:
        //            Url: https://www.igdb.com/games/borderlands-the-pre-sequel-the-holodome-onslaught--1
        //            Name: Borderlands: The Pre-Sequel - Claptastic Voyage
        //        Story line:
        //            Url: https://www.igdb.com/games/borderlands-the-pre-sequel-claptastic-voyage--1
        //            Name: Borderlands 2 Deluxe Vault Hunter's Edition
        //        Story line:
        //            Url: https://www.igdb.com/games/borderlands-2-deluxe-vault-hunters-edition
    })
}
