//! # Advent of Code 2021 Solutions in Rust
//!
//! - ❔ [About Advent of Code](https://adventofcode.com/about)
//! - 📆 [List of Problems](https://adventofcode.com/2021)

#[allow(unused_imports)]
#[macro_use]
extern crate anyhow;
#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate aoc_runner_derive;
extern crate aoc_runner;

mod day1;

aoc_lib! { year = 2021 }
