# IGDB Rust  <img src="https://cdn-images-1.medium.com/max/1200/1*8KkfyqgM4LCruOS5DGUeCA.jpeg" alt="idgb" width="5%" height="5%"/>

## Non-Official Internet Game Database Api Client written in Rust

With igdb-rs you can easily retrieve video game related information such as:

Games, Videos, Artworks, Covers, Screenshots, Release Dates, Multiplayer Options, Game Engines, Websites
and much more!.

Use out of the box client methods or build your own queries to retrieve the exact data that you are looking for.

You have a [Code Samples](#code-samples) section below to check some samples


## IGDB site

https://www.igdb.com/

## IGDB Api end Endpoints information site

https://api-docs.igdb.com/#endpoints

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


## Endpoints

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



**Note**: Clients are automatically generated with macros, so adding new endpoints is straightforward, if you need some other endpoint to be added feel free to request it or collaborate with the library submitting a pull request.



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

### Game Videos Urls
```rust
  let igdb_client = IGDBClient::new("user-key");
  let videos_client = igdb_client.game_videos();

  //Query first 8 youtube videos for Witcher 3
  let response = videos_client.get_by_game_id(1942, 8).await.unwrap();

  for video in response {
    println!("{:?}", video);
 }

 // Youtube links for Witcher 3 Game

 //  GameVideo { id: 5993, game: 1942, video_id: "xQGam9OHSUo" }
 //  GameVideo { id: 5989, game: 1942, video_id: "_IBAovRNCuA" }
 //  GameVideo { id: 5995, game: 1942, video_id: "8ZLfJjlZKvc" }
 //  GameVideo { id: 5987, game: 1942, video_id: "5nLipy-Z4yo" }
 //  GameVideo { id: 5991, game: 1942, video_id: "6f8TbvsZ5Mk" }
 //  GameVideo { id: 5994, game: 1942, video_id: "p14dHAwLOmo" }
 //  GameVideo { id: 5990, game: 1942, video_id: "QrwGXAcE6ZA" }
 //  GameVideo { id: 5996, game: 1942, video_id: "sb81f-ejNSI" }
```

### Screenshots and Covers download


```rust

    let igdb_client = IGDBClient::new("user-key");
    let games_client = igdb_client.games();
    let witcher = games_client.get_first_by_name("Witcher 3").await.unwrap();

    //Get the first 3 covers for the Witcher 3 game
    let covers_client = igdb_client.covers();
    let covers_response = covers_client.get_by_game_id(witcher.id, 3).await.unwrap();

    //Get the first 3 screenshots for the Witcher 3 game
    let screenshots_client = igdb_client.screenshots();
    let screenshots_response = screenshots_client
        .get_by_game_id(witcher.id, 3)
        .await
        .unwrap();

    for (i, cover) in covers_response.iter().enumerate() {
        covers_client
            .download_by_id(
                cover.id,
                format!("cover{}.jpg", i),
                MediaQuality::ScreenshotHuge,
            )
            .await
            .unwrap();
    }

    for (i, screenshot) in screenshots_response.iter().enumerate() {
        screenshots_client
        .download_by_id(
                screenshot.id,
                format!("screenshot{}.jpg", i),
                MediaQuality::ScreenshotHuge,
            )
            .await
            .unwrap();
    }

```


### Custom Queries

Get Borderlands 2 multiplayer information building a custom query

``` rust
   let idbg_client = IGDBClient::new("user-key");

   let games_client = idbg_client.games();

    let mut games_req = IGDBClient::create_request();
    games_req
        .add_field("id")
        .add_field("name")
        .search("Borderlands 2")
        .limit(3);

    let result = games_client.get(games_req).await.unwrap();

    let ids: Vec<String> = result.iter().map(|g| g.id.to_string()).collect();
    let names: Vec<String> = result.iter().map(|g| g.name.clone()).collect();

    let multiplayer_client = idbg_client.multiplayer_modes();

    let mut mul_request = IGDBClient::create_request();

    mul_request.all_fields().add_where_in("id".to_owned(), ids);

    let results = multiplayer_client.get(mul_request).await.unwrap();

    for (i, m) in results.iter().enumerate() {
        println!("{} has online coop: {}", names[i], m.onlinecoop);
        println!("{} has local coop: {}", names[i], m.lancoop);
    }

    //  Prints:
    //  Borderlands 2 has online coop: true
    //  Borderlands 2 has local coop: false
```







