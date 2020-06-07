#[macro_use]
extern crate diesel;

extern crate chrono;
extern crate encoding_rs;
extern crate futures;
extern crate indicatif;
extern crate lazy_static;
extern crate regex;
extern crate reqwest;
extern crate select;
extern crate tokio;

pub mod jockey;
pub mod postgres;
pub mod race;
pub mod schema;
pub mod utils;
