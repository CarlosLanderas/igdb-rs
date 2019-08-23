#[macro_use]
extern crate serde_derive;
mod endpoint_client;
mod endpoints;

#[macro_use]
mod client_macros;
#[macro_use]
mod media_macros;

pub mod client;
pub mod game_extensions;
pub mod media_quality;
pub mod model;
pub mod request_builder;
pub mod request_filters;
