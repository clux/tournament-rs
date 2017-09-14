#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod errors;
pub mod game;

pub mod duel;

pub use errors::{TRes, TErr};
