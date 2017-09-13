use std::fmt;
use std::io;
use serde_json;
// TODO: error-chain?

#[derive(Debug)]
pub enum TErr {
    Io(io::Error),
    Parse(serde_json::error::Error),
}
pub type TRes<T> = Result<T, TErr>;

// Format implementation used when printing an error
impl fmt::Display for TErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TErr::Io(ref err) => err.fmt(f),
            TErr::Parse(ref err) => err.fmt(f),
        }
    }
}

// Absorb error types
impl From<io::Error> for TErr {
    fn from(err: io::Error) -> TErr {
        TErr::Io(err)
    }
}
impl From<serde_json::error::Error> for TErr {
    fn from(err: serde_json::error::Error) -> TErr {
        TErr::Parse(err)
    }
}
