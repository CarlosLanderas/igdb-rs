#![feature(async_await)]
#[macro_use]
extern crate serde_derive;

mod endpoints;
mod endpoint_client;
pub mod client;
pub mod model;
pub mod request_builder;
