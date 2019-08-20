#![feature(async_await)]
#[macro_use]
extern crate serde_derive;

mod endpoint_client;
mod endpoints;

#[macro_use]
mod macros;

pub mod client;
pub mod model;
pub mod request_builder;
