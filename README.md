# tournament
[![build status](https://secure.travis-ci.org/clux/tournament.svg)](http://travis-ci.org/clux/tournament)
[![coverage status](http://img.shields.io/coveralls/clux/tournament.svg)](https://coveralls.io/r/clux/tournament)
[![crates status](https://img.shields.io/crates/v/tournament.svg)](https://crates.io/crates/tournament)

An experiment rewriting [tournament-js](https://github.com/clux/tournament) to rust.
The original implementation had lots of strictness additions around the API that went quite a bit beyond what native javascript makes natural. Rust could be a better fit for the functional and generic style and have a lot of the same guarantees validated at compile time.

We will see. Goals are to have an equal serialization format and a similar-ish API.


## Usage
Add [tournament](https://crates.io/crates/tournament) to `Cargo.toml`.

## [documentation](http://clux.github.io/tournament)

## License
MIT-Licensed. See LICENSE file for details.
