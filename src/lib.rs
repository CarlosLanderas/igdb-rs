#![feature(async_await)]
#[macro_use]
extern crate serde_derive;
mod endpoint_client;
mod endpoints;

#[macro_use]
mod macros;

pub mod client;
pub mod game_extensions;
pub mod media_extensions;
pub mod model;
pub mod request_builder;
pub mod request_filters;
pub mod media_quality;
