//! # Advent of Code 2021 Solutions in Rust
//!
//! - ❔ [About Advent of Code](https://adventofcode.com/about)
//! - 📆 [List of Problems](https://adventofcode.com/2021)

#[allow(unused_imports)]
#[macro_use]
extern crate anyhow;
#[allow(unused_imports)]
// #[macro_use]
// extern crate serde_json;
#[macro_use]
extern crate aoc_runner_derive;
extern crate aoc_runner;

mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

aoc_lib! { year = 2021 }
