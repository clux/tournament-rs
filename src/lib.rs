#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

pub mod errors;
pub mod game;

pub use errors::{TRes, TErr};
