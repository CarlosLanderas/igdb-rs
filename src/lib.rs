#![feature(async_await)]
#[macro_use]
extern crate serde_derive;

pub mod client;
mod endpoint_client;
mod endpoints;
pub mod model;
pub mod request_builder;
