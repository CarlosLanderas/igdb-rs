//! # Non-Official Internet Game Database Api Client written in Rust
//!
//! With igdb-rs you can easily retrieve video game related information such as:
//!
//! Games, Franchises, Videos, Artworks, Covers, Screenshots, Release Dates, Multiplayer Options, Game Engines, Websites and much more!.
//!
//! Use out of the box client methods or build your own queries to retrieve the exact data that you are looking for.
//!
//!
//! ## Links
//!  [Repository]
//!
//!  [README]
//!
//!  [IGDB Site]
//!
//!  [IGDB Api]
//!
//! [Repository]: https://github.com/CarlosLanderas/igdb-rs
//! [IGDB Site]: https://github.com/TyOverby/bincode
//! [IGDB Api]: https://api-docs.igdb.com/#endpoints
//! [examples]: https://github.com/CarlosLanderas/igdb-rs/tree/master/examples
//! [README]: https://github.com/CarlosLanderas/igdb-rs/blob/develop/README.md
//!
//! ## Code Examples
//! You can check some code samples here: [examples]
//!
//! ## Examples
//!
//! **Get game by name**
//!
//! ```no_run
//!use async_std::task;
//!use igdb_rs::client::IGDBClient;
//!
//!task::block_on(async {
//!    let games_client = IGDBClient::new("user-key").games();
//!    let games_results = games_client.get_by_name("Borderlands", 10).await.unwrap();
//!
//!    for game in games_results {
//!        println!("Name: {}", game.name);
//!        println!("Story line: {}", game.storyline);
//!        println!("Url: {}", game.url);
//!    }
//!
//!   //  Name: The Witcher 3: Wild Hunt - Hearts of Stone
//!   //  Summary: Hired by the Merchant of Mirrors, Geralt is tasked with overcoming Olgierd von Everec -- a ruthless bandit captain enchanted with the power of immorta ...
//!   //  Story line: Professional monster slayer is hired to deal with a ruthless bandit captain who possesses the power of immortality.
//!   //  Url: https://www.igdb.com/games/the-witcher-3-wild-hunt-hearts-of-stone
//!
//!})
//!```
//!
//! **Get games by name**
//!
//!```no_run
//!use async_std::task;
//!use igdb_rs::client::IGDBClient;
//!
//!   task::block_on(async {
//!        let games_client = IGDBClient::new("user-key").games();
//!        // Get ten first games containing word Borderlands
//!        let games_results = games_client.get_by_name("Borderlands", 10).await.unwrap();
//!
//!        for game in games_results {
//!            println!("Name: {}", game.name);
//!            println!("Story line: {}", game.storyline);
//!            println!("Url: {}", game.url);
//!        }
//!
//!        //        Name: Borderlands: The Pre-Sequel - Lady Hammerlock The Baroness
//!        //        Story line:
//!        //            Url: https://www.igdb.com/games/borderlands-the-pre-sequel-lady-hammerlock-the-baroness
//!        //            Name: Borderlands Legends
//!        //        Story line:
//!        //            Url: https://www.igdb.com/games/borderlands-legends
//!        //            Name: Tales from the Borderlands: Episode 3 - Catch a Ride
//!        //        Story line:
//!        //            Url: https://www.igdb.com/games/tales-from-the-borderlands-episode-3-catch-a-ride
//!        //            Name: Borderlands 2: Game of the Year Edition
//!
//!        // Omitted for brevity...
//!    })
//!```
//!
//! **Game release date**
//!
//!```no_run
//!use async_std::task;
//!use igdb_rs::client::IGDBClient;
//!
//!   task::block_on(async {
//!        let igdb_client = IGDBClient::new("user-key");
//!
//!        let release_client = igdb_client.release_dates();
//!
//!       //Get releases for Borderlands3 with id 19164
//!        let releases = release_client.get_by_game_id(19164, 10).await.unwrap();
//!
//!        let platform_client = igdb_client.platforms();
//!
//!        for release in releases {
//!            let platform = platform_client
//!                .get_first_by_id(release.platform as usize)
//!                .await
//!                .unwrap();
//!
//!            println!(
//!                "platform: {} release date: {}",
//!                platform.name, release.human
//!            );
//!        }
//!
//!        //  platform: Xbox One release date: 2019-Sep-13
//!        //  platform: PC (Microsoft Windows) release date: 2019-Sep-13
//!        //  platform: PlayStation 4 release date: 2019-Sep-13
//!        //  platform: Google Stadia release date: 2019-Sep-13
//!    })
//!```
//!
//! **Download Screenshots and Covers for a game**
//!
//! ```no_run
//!
//!use async_std::task;
//!use igdb_rs::client::IGDBClient;
//!use igdb_rs::media_quality::MediaQuality;
//!
//!
//! task::block_on(async {
//!        let igdb_client = IGDBClient::new("user-key");
//!        let games_client = igdb_client.games();
//!        let witcher = games_client.get_first_by_name("Witcher 3").await.unwrap();
//!
//!        //Get the first 3 covers for the Witcher 3 game
//!        let covers_client = igdb_client.covers();
//!        let covers_response = covers_client.get_by_game_id(witcher.id, 3).await.unwrap();
//!
//!        //Get the first 3 screenshots for the Witcher 3 game
//!        let screenshots_client = igdb_client.screenshots();
//!        let screenshots_response = screenshots_client
//!            .get_by_game_id(witcher.id, 3)
//!            .await
//!            .unwrap();
//!
//!        for (i, cover) in covers_response.iter().enumerate() {
//!            covers_client
//!                .download_by_id(
//!                    cover.id,
//!                    format!("cover{}.jpg", i),
//!                    MediaQuality::ScreenshotHuge,
//!                )
//!                .await
//!                .unwrap();
//!        }
//!
//!        for (i, screenshot) in screenshots_response.iter().enumerate() {
//!            screenshots_client
//!                .download_by_id(
//!                    screenshot.id,
//!                    format!("screenshot{}.jpg", i),
//!                    MediaQuality::CoverBig,
//!                )
//!                .await
//!                .unwrap();
//!        }
//!    })
//! ```
//!
//! **You can read more samples here: [examples]**
#[macro_use]
extern crate serde_derive;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

mod endpoint_client;
mod endpoints;

#[macro_use]
mod client_macros;
#[macro_use]
mod media_macros;

pub mod client;
pub mod extensions;
pub mod media_helpers;
pub mod media_quality;
pub mod model;
pub mod request_builder;
pub mod request_filters;
