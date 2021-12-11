//! # [Day 3: Binary Diagnostic](https://adventofcode.com/2021/day/3)
//!
//! The submarine has been making some odd creaking noises, so you ask it to produce a
//! diagnostic report just in case.
//!
//! The diagnostic report (your puzzle input) consists of a list of binary numbers which,
//! when decoded properly, can tell you many useful things about the conditions of the submarine.
//! The first parameter to check is the power consumption.
//!
//! You need to use the binary numbers in the diagnostic report to generate two new binary numbers
//! (called the gamma rate and the epsilon rate).
//! The power consumption can then be found by multiplying the gamma rate by the epsilon rate.
//!
//! Each bit in the gamma rate can be determined by finding the most common bit in the
//! corresponding position of all numbers in the diagnostic report. For example,
//! given the following diagnostic report:
//!
//! ```plain
//! 00100
//! 11110
//! 10110
//! 10111
//! 10101
//! 01111
//! 00111
//! 11100
//! 10000
//! 11001
//! 00010
//! 01010
//! ```
//!
//! Considering only the first bit of each number, there are five `0` bits and seven `1` bits.
//! Since the most common bit is `1`, the first bit of the gamma rate is `1`.
//!
//! The most common second bit of the numbers in the diagnostic report is `0`,
//! so the second bit of the gamma rate is `0`.
//!
//! The most common value of the third, fourth, and fifth bits are `1`, `1`, and `0`,
//! respectively, and so the final three bits of the gamma rate are `110`.
//!
//! So, the gamma rate is the binary number `10110`, or `22` in decimal.
//!
//! The epsilon rate is calculated in a similar way; rather than use the most common bit,
//! the least common bit from each position is used. So, the epsilon rate is `01001`,
//! or `9` in decimal. Multiplying the gamma rate (`22`) by the epsilon rate (`9`)
//! produces the power consumption, `198`.
//!
//! Use the binary numbers in your diagnostic report to calculate the gamma rate and epsilon rate,
//! then multiply them together.
//!
//! **What is the power consumption of the submarine?**
//!
//! (Be sure to represent your answer in decimal, not binary.)
//!
//! # Part Two
//!
//! Next, you should verify the life support rating, which can be determined by multiplying the
//! oxygen generator rating by the CO2 scrubber rating.
//!
//! Both the oxygen generator rating and the CO2 scrubber rating are values that can be found in
//! your diagnostic report - finding them is the tricky part. Both values are located using a
//! similar process that involves filtering out values until only one remains. Before searching
//! for either rating value, start with the full list of binary numbers from your diagnostic report
//! and consider just the first bit of those numbers. Then:
//!
//! -   Keep only numbers selected by the bit criteria for the type of rating value for which you
//!     are searching. Discard numbers which do not match the bit criteria.
//! -   If you only have one number left, stop; this is the rating value for which you
//!     are searching.
//! -   Otherwise, repeat the process, considering the next bit to the right.
//!
//! The bit criteria depends on which type of rating value you want to find:
//!
//! -   To find oxygen generator rating, determine the most common value (`0` or `1`) in the
//! current bit position, and keep only numbers with that bit in that position. If `0` and `1`
//! are equally common, keep values with a `1` in the position being considered.
//! -   To find CO2 scrubber rating, determine the least common value (`0` or `1`) in the current
//! bit position, and keep only numbers with that bit in that position. If `0` and `1` are equally
//! common, keep values with a `0` in the position being considered.
//!
//! For example, to determine the oxygen generator rating value using the same example
//! diagnostic report from above:
//!
//! -   Start with all 12 numbers and consider only the first bit of each number.
//!     There are more `1` bits (7) than `0` bits (5), so keep only the 7 numbers with a `1` in
//!     the first position: `11110`, `10110`, `10111`, `10101`, `11100`, `10000`, and `11001`.
//! -   Then, consider the second bit of the 7 remaining numbers: there are more `0` bits (4)
//!     than `1` bits (3), so keep only the 4 numbers with a `0` in the second position:
//!     `10110`, `10111`, `10101`, and `10000`.
//! -   In the third position, three of the four numbers have a `1`, so keep those three:
//!     `10110`, `10111`, and `10101`.
//! -   In the fourth position, two of the three numbers have a `1`, so keep those two:
//!     `10110` and `10111`.
//! -   In the fifth position, there are an equal number of `0` bits and `1` bits (one each).
//!     So, to find the oxygen generator rating, keep the number with a `1` in that position:
//!     `10111`.
//! -   As there is only one number left, stop;
//!     the oxygen generator rating is `10111`, or `23` in decimal.
//!
//! Then, to determine the CO2 scrubber rating value from the same example above:
//!
//! -   Start again with all 12 numbers and consider only the first bit of each number.
//!     There are fewer `0` bits (5) than `1` bits (7), so keep only the 5 numbers with a `0`
//!     in the first position:
//!     `00100`, `01111`, `00111`, `00010`, and `01010`.
//! -   Then, consider the second bit of the 5 remaining numbers: there are fewer `1` bits (2)
//!     than `0` bits (3), so keep only the 2 numbers with a `1` in the second position:
//!     `01111` and `01010`.
//! -   In the third position, there are an equal number of `0` bits and `1` bits (one each).
//!     So, to find the CO2 scrubber rating, keep the number with a `0` in that position: `01010`.
//! -   As there is only one number left, stop; the CO2 scrubber rating is `01010`,
//!     or `10` in decimal.
//!
//! Finally, to find the life support rating, multiply the oxygen generator rating (`23`) by the
//! CO2 scrubber rating (`10`) to get `230`.
//!
//! Use the binary numbers in your diagnostic report to calculate the oxygen generator rating and
//! CO2 scrubber rating, then multiply them together.
//!
//! **What is the life support rating of the submarine?**
//!
//! (Be sure to represent your answer in decimal, not binary.)

use bitlab::*;

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect()
}

/// Part 1:
/// Use the binary numbers in your diagnostic report to calculate the gamma rate and epsilon rate,
/// then multiply them together.
/// What is the power consumption of the submarine?
#[aoc(day3, part1)]
fn part1(input: &Vec<u32>) -> u64 {
    gamma(input) as u64 * epsilon(input) as u64
}

/// Part 2:
/// What is the life support rating of the submarine?
/// Use the binary numbers in your diagnostic report to calculate the oxygen generator rating and
/// CO2 scrubber rating, then multiply them together.
#[aoc(day3, part2)]
fn part2(input: &Vec<u32>) -> u64 {
    oxygen(input) as u64 * co2(input) as u64
}

///  To find oxygen generator rating, determine the most common value (`0` or `1`) in the
///  current bit position, and keep only numbers with that bit in that position. If `0` and `1`
///  are equally common, keep values with a `1` in the position being considered.
fn oxygen(input: &Vec<u32>) -> u32 {
    let max = *input.iter().max().unwrap();
    let start = significant_bitcount(max).unwrap();

    let mut remaining = input.clone();
    for i in start..32 {
        let (count_zeros, count_ones) = count_ones_zeros_at(&remaining, i);
        if count_ones >= count_zeros {
            remaining = remaining
                .into_iter()
                .filter(|v| v.get_bit(i).unwrap())
                .collect();
        } else {
            remaining = remaining
                .into_iter()
                .filter(|v| !v.get_bit(i).unwrap())
                .collect();
        }
        if remaining.len() == 1 {
            return remaining[0];
        }
    }
    0
}

/// To find CO2 scrubber rating, determine the least common value (`0` or `1`) in the current
/// bit position, and keep only numbers with that bit in that position. If `0` and `1` are equally
/// common, keep values with a `0` in the position being considered.
fn co2(input: &Vec<u32>) -> u32 {
    let max = *input.iter().max().unwrap();
    let start = significant_bitcount(max).unwrap();

    let mut remaining = input.clone();
    for i in start..32 {
        let (count_zeros, count_ones) = count_ones_zeros_at(&remaining, i);
        if count_zeros <= count_ones {
            remaining = remaining
                .into_iter()
                .filter(|v| !v.get_bit(i).unwrap())
                .collect();
        } else {
            remaining = remaining
                .into_iter()
                .filter(|v| v.get_bit(i).unwrap())
                .collect();
        }
        if remaining.len() == 1 {
            return remaining[0];
        }
    }
    0
}

fn count_ones_zeros_at(input: &Vec<u32>, pos: u32) -> (u32, u32) {
    let mut count_zeros = 0;
    let mut count_ones = 0;
    for v in input {
        if v.get_bit(pos).unwrap() {
            count_ones += 1;
        } else {
            count_zeros += 1;
        }
    }
    (count_zeros, count_ones)
}

/// The gamma rate is calculated by using the least common bit from each position.
fn gamma(input: &Vec<u32>) -> u32 {
    let mut ret = 0;
    for i in 0..32 {
        let (count_zeros, count_ones) = count_ones_zeros_at(input, i);
        if count_ones >= count_zeros {
            ret = ret.set_bit(i).unwrap();
        }
    }
    ret
}

fn significant_bitcount(value: u32) -> Option<u32> {
    for i in 0..32 {
        if value.get_bit(i).unwrap() {
            return Some(i);
        }
    }
    None
}

/// The epsilon rate is calculated in a similar way; rather than use the most common bit,
/// the least common bit from each position is used.
fn epsilon(input: &Vec<u32>) -> u32 {
    let gamma = gamma(input);
    let max = *input.iter().max().unwrap();
    let start = significant_bitcount(max).unwrap();
    let mut ret = 0;
    for i in start..32 {
        if !gamma.get_bit(i).unwrap() {
            ret = ret.set_bit(i).unwrap()
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn part1_examples() {
        // So, the gamma rate is the binary number `10110`, or `22` in decimal.
        assert_eq!(gamma(&parse_input(EXAMPLE)), 0b1_0110);
        assert_eq!(gamma(&parse_input(EXAMPLE)), 22);

        // So, the epsilon rate is `01001`, or `9` in decimal.
        assert_eq!(epsilon(&parse_input(EXAMPLE)), 0b1001);
        assert_eq!(epsilon(&parse_input(EXAMPLE)), 9);
    }

    #[test]
    fn part2_examples() {
        // the oxygen generator rating is `10111`, or `23` in decimal.
        assert_eq!(oxygen(&parse_input(EXAMPLE)), 0b1_0111);
        assert_eq!(oxygen(&parse_input(EXAMPLE)), 23);

        // the CO2 scrubber rating is `01010`, or `10` in decimal.
        assert_eq!(co2(&parse_input(EXAMPLE)), 0b0_1010);
        assert_eq!(co2(&parse_input(EXAMPLE)), 10);
    }
}
