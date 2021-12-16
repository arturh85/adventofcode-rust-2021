//! # [Day 8: Seven Segment Search](https://adventofcode.com/2021/day/8)
//!
//! You barely reach the safety of the cave when the whale smashes into the cave mouth,
//! collapsing it. Sensors indicate another exit to this cave at a much greater depth,
//! so you have no choice but to press on.
//!
//! As your submarine slowly makes its way through the cave system, you notice that the
//! four-digit [seven-segment displays](https://en.wikipedia.org/wiki/Seven-segment_display) in
//! your submarine are malfunctioning; they must have been damaged during the escape. You'll be in a
//! lot of trouble without them, so you'd better figure out what's wrong.
//!
//! Each digit of a seven-segment display is rendered by turning on or off any of seven segments
//! named `a` through `g`:
//!
//! ```plain
//!   0:      1:      2:      3:      4:
//!  aaaa    ....    aaaa    aaaa    ....
//! b    c  .    c  .    c  .    c  b    c
//! b    c  .    c  .    c  .    c  b    c
//!  ....    ....    dddd    dddd    dddd
//! e    f  .    f  e    .  .    f  .    f
//! e    f  .    f  e    .  .    f  .    f
//!  gggg    ....    gggg    gggg    ....
//!
//!   5:      6:      7:      8:      9:
//!  aaaa    aaaa    aaaa    aaaa    aaaa
//! b    .  b    .  .    c  b    c  b    c
//! b    .  b    .  .    c  b    c  b    c
//!  dddd    dddd    ....    dddd    dddd
//! .    f  e    f  .    f  e    f  .    f
//! .    f  e    f  .    f  e    f  .    f
//!  gggg    gggg    ....    gggg    gggg
//! ```
//!
//! So, to render a `1`, only segments `c` and `f` would be turned on; the rest would be off.
//! To render a `7`, only segments `a`, `c`, and `f` would be turned on.
//!
//! The problem is that the signals which control the segments have been mixed up on each display.
//! The submarine is still trying to display numbers by producing output on signal wires `a`
//! through `g`, but those wires are connected to segments randomly. Worse, the wire/segment
//! connections are mixed up separately for each four-digit display! (All of the digits within a
//! display use the same connections, though.)
//!
//! So, you might know that only signal wires `b` and `g` are turned on, but that doesn't mean
//! egments `b` and `g` are turned on: the only digit that uses two segments is `1`, so it must
//! mean segments `c` and `f` are meant to be on. With just that information, you still can't tell
//! which wire (`b`/`g`) goes to which segment (`c`/`f`). For that, you'll need to collect
//! more information.
//!
//! For each display, you watch the changing signals for a while, make a note of all ten unique
//! signal patterns you see, and then write down a single four digit output value
//! (your puzzle input). Using the signal patterns, you should be able to work out
//! which pattern corresponds to which digit.
//!
//! For example, here is what you might see in a single entry in your notes:
//!
//! ```plain
//! acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
//! cdfeb fcadb cdfeb cdbaf
//! ```
//!
//! (The entry is wrapped here to two lines so it fits; in your notes,
//! it will all be on a single line.)
//!
//! Each entry consists of ten unique signal patterns, a `|` delimiter, and finally the four digit
//! output value. Within an entry, the same wire/segment connections are used (but you don't know
//! what the connections actually are). The unique signal patterns correspond to the ten different
//! ways the submarine tries to render a digit using the current wire/segment connections.
//! Because `7` is the only digit that uses three segments, `dab` in the above example means that
//! to render a `7`, signal lines `d`, `a`, and `b` are on. Because `4` is the only digit that uses
//! four segments, `eafb` means that to render a `4`, signal lines `e`, `a`, `f`, and `b` are on.
//!
//! Using this information, you should be able to work out which combination of signal wires
//! corresponds to each of the ten digits. Then, you can decode the four digit output value.
//! Unfortunately, in the above example, all of the digits in the output
//! value (`cdfeb fcadb cdfeb cdbaf`) use five segments and are more difficult to deduce.
//!
//! For now, focus on the easy digits. Consider this larger example:
//!
//! ```plain
//! be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
//! fdgacbe cefdb cefbgd gcbe
//! edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |
//! fcgedb cgb dgebacf gc
//! fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |
//! cg cg fdcagb cbg
//! fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |
//! efabcd cedba gadfec cb
//! aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |
//! gecf egdcabf bgf bfgea
//! fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |
//! gebdcfa ecba ca fadegcb
//! dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |
//! cefg dcbef fcge gbcadfe
//! bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |
//! ed bcgafe cdgba cbgef
//! egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |
//! gbdfcae bgc cg cgb
//! gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |
//! fgae cfgab fg bagce
//!
//! ```
//!
//! Because the digits `1`, `4`, `7`, and `8` each use a unique number of segments, you should be
//! able to tell which combinations of signals correspond to those digits. Counting only digits in
//! the output values (the part after `|` on each line), in the above example, there are `26`
//! instances of digits that use a unique number of segments (highlighted above).
//!
//! **In the output values, how many times do digits `1`, `4`, `7`, or `8` appear**?
//!
//! # Part Two
//!
//! Through a little deduction, you should now be able to determine the remaining digits.
//! Consider again the first example above:
//!
//! ```plain
//! acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
//! cdfeb fcadb cdfeb cdbaf
//! ```
//!
//! After some careful analysis, the mapping between signal wires and segments only make sense in the following configuration:
//!
//! ```plain
//!  dddd
//! e    a
//! e    a
//!  ffff
//! g    b
//! g    b
//!  cccc
//!
//! ```
//!
//! So, the unique signal patterns would correspond to the following digits:
//!
//! -   `acedgfb`: `8`
//! -   `cdfbe`: `5`
//! -   `gcdfa`: `2`
//! -   `fbcad`: `3`
//! -   `dab`: `7`
//! -   `cefabd`: `9`
//! -   `cdfgeb`: `6`
//! -   `eafb`: `4`
//! -   `cagedb`: `0`
//! -   `ab`: `1`
//!
//! Then, the four digits of the output value can be decoded:
//!
//! -   `cdfeb`: `5`
//! -   `fcadb`: `3`
//! -   `cdfeb`: `5`
//! -   `cdbaf`: `3`
//!
//! Therefore, the output value for this entry is `5353`.
//!
//! Following this same process for each entry in the second, larger example above, the output value of each entry can be determined:
//!
//! -   `fdgacbe cefdb cefbgd gcbe`: `8394`
//! -   `fcgedb cgb dgebacf gc`: `9781`
//! -   `cg cg fdcagb cbg`: `1197`
//! -   `efabcd cedba gadfec cb`: `9361`
//! -   `gecf egdcabf bgf bfgea`: `4873`
//! -   `gebdcfa ecba ca fadegcb`: `8418`
//! -   `cefg dcbef fcge gbcadfe`: `4548`
//! -   `ed bcgafe cdgba cbgef`: `1625`
//! -   `gbdfcae bgc cg cgb`: `8717`
//! -   `fgae cfgab fg bagce`: `4315`
//!
//! Adding all of the output values in this larger example produces `61229`.
//!
//! For each entry, determine all of the wire/segment connections and decode the
//! four-digit output values. **What do you get if you add up all of the output values?**

use std::collections::HashMap;

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Vec<(Vec<String>, Vec<String>)> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(" | ").collect();
            (
                parts[0].split(' ').map(sort).collect(),
                parts[1].split(' ').map(sort).collect(),
            )
        })
        .collect()
}

/// Part 1: In the output values, how many times do digits `1`, `4`, `7`, or `8` appear?
#[aoc(day8, part1)]
fn part1(input: &[(Vec<String>, Vec<String>)]) -> usize {
    let mut cnt = 0;
    // 1: 2 segments
    // 7: 3 segments
    // 4: 4 segments
    // 8: 7 segments
    for (_, outputs) in input {
        for output in outputs {
            match output.len() {
                2 | 3 | 4 | 7 => cnt += 1,
                _ => {}
            }
        }
    }
    cnt
}

/// Part 2: What do you get if you add up all of the output values?
#[aoc(day8, part2)]
fn part2(input: &[(Vec<String>, Vec<String>)]) -> u64 {
    input
        .iter()
        .map(|(unique_signal_patterns, value)| decode(unique_signal_patterns, value))
        .sum()
}

fn sort(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    String::from_iter(chars)
}

// returns true if every char of needles if present in the haystack
fn string_contains_chars(haystack: &str, needles: &str) -> bool {
    for c in needles.chars() {
        if !haystack.contains(c) {
            return false;
        }
    }
    true
}

fn build_map(unique_signal_patterns: &[String]) -> HashMap<String, u64> {
    let mut output = HashMap::new();

    let signal1 = unique_signal_patterns
        .iter()
        .find(|s| s.len() == 2)
        .expect("failed to find 1");
    output.insert(signal1.to_string(), 1); // ab: 1

    let signal7 = unique_signal_patterns
        .iter()
        .find(|s| s.len() == 3)
        .expect("failed to find 7");
    output.insert(signal7.to_string(), 7); // dab: 7

    let signal4 = unique_signal_patterns
        .iter()
        .find(|s| s.len() == 4)
        .expect("failed to find 4");
    output.insert(signal4.to_string(), 4); // eafb: 4

    let signal8 = unique_signal_patterns
        .iter()
        .find(|s| s.len() == 7)
        .expect("failed to find 8");
    output.insert(signal8.to_string(), 8); // acedgfb: 8

    // 3 should be a 1 with an additional segment
    let signal3 = unique_signal_patterns
        .iter()
        .find(|s| s.len() == 5 && string_contains_chars(s, signal1))
        .expect("failed to find 3");
    output.insert(signal3.to_string(), 3); // fbcad: 3

    // 9 should be 3 with 1 additional segment
    let signal9 = unique_signal_patterns
        .iter()
        .find(|s| s.len() == 6 && string_contains_chars(s, signal3))
        .expect("failed to find 9");
    output.insert(signal9.to_string(), 9); // cefabd: 9

    // 0 should contain 1
    let signal0 = unique_signal_patterns
        .iter()
        .find(|s| s.len() == 6 && !s.eq(&signal9) && string_contains_chars(s, signal1))
        .expect("failed to find 0");
    output.insert(signal0.to_string(), 0); // cagedb: 0

    // 6 should have 6 segments
    let signal6 = unique_signal_patterns
        .iter()
        .find(|s| s.len() == 6 && !s.eq(&signal9) && !s.eq(&signal0))
        .expect("failed to find 6");
    output.insert(signal6.to_string(), 6); // cdfgeb: 6

    // 5 is included in 6
    let signal5 = unique_signal_patterns
        .iter()
        .find(|s| s.len() == 5 && !s.eq(&signal3) && string_contains_chars(signal6, s))
        .expect("failed to find 5");
    output.insert(signal5.to_string(), 5); // cdfbe: 5

    // 2 is not 3 or 5
    let signal6 = unique_signal_patterns
        .iter()
        .find(|s| s.len() == 5 && !s.eq(&signal3) && !s.eq(&signal5))
        .expect("failed to find 2");
    output.insert(signal6.to_string(), 2); // gcdfa: 2

    assert_eq!(10, output.len());
    output
}

fn decode(unique_signal_patterns: &[String], value: &[String]) -> u64 {
    let map = build_map(unique_signal_patterns);
    let mut output = String::new();
    for digit in value {
        output += &*map.get(digit).unwrap().to_string();
    }
    output.parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn part1_examples() {
        assert_eq!(part1(&parse_input(EXAMPLE)), 26);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(
            part2(&parse_input(
                "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
            )),
            5353
        );
        assert_eq!(part2(&parse_input(EXAMPLE)), 61229);
    }
}
