# IGDB Rust  <img src="https://cdn-images-1.medium.com/max/1200/1*8KkfyqgM4LCruOS5DGUeCA.jpeg" alt="idgb" width="8%" height="8%"/>

## Non-Official Internet Game Database Api Client written in Rust

## Documentation
Check the documentation here : [docs](https://docs.rs/igdb-rs)

## Examples
You can find some code samples snippets here: [examples](https://github.com/CarlosLanderas/igdb-rs/tree/master/examples)

## Quickstart

Add the following lines to you `Cargo.toml`:

```toml
[dependencies]
igdb-rs = "0.0.1"
```

Or use [cargo add][cargo-add] if you have it installed:

```sh
$ cargo add igdb-rs
```

[cargo-add]: https://github.com/killercup/cargo-edit


## Code samples

With igdb-rs you can easily query the Internet Game Database.

You just need to create an IGDBClient object with your api key, you can sign and get one here:
https://api.igdb.com/


```rust
let igdb_client = IGDBClient::new("user-key");

```


### Game by name
```rust
let games_client = IGDBClient::new("user-key").games();
let game = games_client.get_first_by_name("Witcher 3").await.unwrap();

println!("Name: {}", game.name);
println!("Summary: {} ...", &game.summary[..150]);
println!("Story line: {}", game.storyline);
println!("Url: {}", game.url);

//  Name: The Witcher 3: Wild Hunt - Hearts of Stone
//  Summary: Hired by the Merchant of Mirrors, Geralt is tasked with overcoming Olgierd von Everec -- a ruthless bandit captain enchanted with the power of immorta ...
//  Story line: Professional monster slayer is hired to deal with a ruthless bandit captain who possesses the power of immortality.
//  Url: https://www.igdb.com/games/the-witcher-3-wild-hunt-hearts-of-stone
```

### Games by name
```rust
 let games_client = IGDBClient::new("user-key").games();
        // Get ten first results containing Borderlands in it's name
        let games_results = games_client.get_by_name("Borderlands", 10).await.unwrap();

        for game in games_results {
            println!("Name: {}", game.name);
            println!("Story line: {}", game.storyline);
            println!("Url: {}", game.url);
        }

        //  Name: Borderlands: The Pre-Sequel - Lady Hammerlock The Baroness
        //  Story line:
        //  Url: https://www.igdb.com/games/borderlands-the-pre-sequel-lady-hammerlock-the-baroness
        //  Name: Borderlands Legends
        //  Story line:
        //  Url: https://www.igdb.com/games/borderlands-legends
        //  Name: Tales from the Borderlands: Episode 3 - Catch a Ride
        //  Story line:
        //  Url: https://www.igdb.com/games/tales-from-the-borderlands-episode-3-catch-a-ride
        //  Name: Borderlands 2: Game of the Year Edition
        //  Story line:
        //  Url: https://www.igdb.com/games/borderlands-2-game-of-the-year-edition

        //Omitted for brevity...
    })
```

### Game characters
```rust
let characters_client = igdb_client.characters();

 //Get Witcher 3 characters
 for ch in characters_client.get_by_game_id(1942, 10).await.unwrap() {
   println!("name: {}, slug: {}, url: {}", ch.name, ch.slug, ch.url);
 }

//  name: Dandelion, slug: dandelion, url: https://www.igdb.com/characters/dandelion
//  name: Jaskier, slug: jaskier, url: https://www.igdb.com/characters/jaskier
//  name: Emhyr Var Empreis, slug: emhyr-var-empreis, url: https://www.igdb.com/characters/emhyr-var-empreis
//  name: Ciri, slug: ciri, url: https://www.igdb.com/characters/ciri
//  name: Avallac'h, slug: avallach, url: https://www.igdb.com/characters/avallach

//Omitted for brevity...
```



### Game engine info
```rust
  let igdb_client = IGDBClient::new("user-key");

  let games_client = igdb_client.games();
  let game = games_client
     .get_first_by_name("Riders of Asgard")
     .await
     .unwrap();

 let engine_id = game.game_engines.first().unwrap();

 let engines_client = igdb_client.game_engines();
 let engine = engines_client
     .get_first_by_id(*engine_id as usize)
     .await
     .unwrap();

 println!(
         "name: {}, url: {}, companies: {:?}",
         engine.name, engine.url, engine.companies
        );

    // name: Unreal Engine 4, url: https://www.igdb.com/game_engines/unreal-engine-4--1,
    // companies: [168, 11060]
```

### Game release data
```rust
  let igdb_client = IGDBClient::new("user-key");

  let release_client = igdb_client.release_dates();

  //Get releases for Borderlands3 with id 19164
  let releases = release_client.get_by_game_id(19164, 10).await.unwrap();

  let platform_client = igdb_client.platforms();

  for release in releases {
    let platform = platform_client
        .get_first_by_id(release.platform as usize)
        .await
        .unwrap();

    println!(
            "platform: {} release date: {}",
            platform.name, release.human
            );
    }

    //  platform: Xbox One release date: 2019-Sep-13
    //  platform: PC (Microsoft Windows) release date: 2019-Sep-13
    //  platform: PlayStation 4 release date: 2019-Sep-13
    //  platform: Google Stadia release date: 2019-Sep-13
```



igdb-rs supports the following endpoints at this moment:

| Endpoint  | Description |
| ------------- | ------------- |
| Artworks  | Official artworks (resolution and aspect ratio may vary)  |
| Characters  | Video game characters |
| Companies | Video game companies. Both publishers & developers |
| Covers | The cover art of games |
| Games | Video Games! |
| Game Engines | Video game engines such as unreal engine. |
| Game Modes | Single player, Multiplayer etc |
| Game Videos | Videos associated with games |
| Multiplayer Modes | Data about the supported multiplayer types|
| Platforms |  The hardware used to run the game or game delivery network |
| Release Dates |  A handy endpoint that extends game release dates. Used to dig deeper into release dates, platforms and versions. |
| Screenshots | Screenshots of games |
| Websites | A website url, usually associated with a game |


Clients are automatically generated with macros, so adding new endpoints is straightforward, if you need some other endpoint to be added feel free to request it or collaborate with the library submitting a pull request.









