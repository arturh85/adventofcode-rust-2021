//! # [Day 2: Dive!](https://adventofcode.com/2021/day/2)
//!
//! Now, you need to figure out how to pilot this thing.
//!
//! It seems like the submarine can take a series of commands like `forward 1`, `down 2`, or `up 3`:
//!
//! -   `forward X` increases the horizontal position by `X` units.
//! -   `down X` increases the depth by `X` units.
//! -   `up X` decreases the depth by `X` units.
//!
//! Note that since you're on a submarine, `down` and `up` affect your depth,
//! and so they have the opposite result of what you might expect.
//!
//! The submarine seems to already have a planned course (your puzzle input).
//! You should probably figure out where it's going. For example:
//!
//! ```plain
//! forward 5
//! down 5
//! forward 8
//! up 3
//! down 8
//! forward 2
//!
//! ```
//!
//! Your horizontal position and depth both start at `0`. The steps above would then modify them as follows:
//!
//! -   `forward 5` adds `5` to your horizontal position, a total of `5`.
//! -   `down 5` adds `5` to your depth, resulting in a value of `5`.
//! -   `forward 8` adds `8` to your horizontal position, a total of `13`.
//! -   `up 3` decreases your depth by `3`, resulting in a value of `2`.
//! -   `down 8` adds `8` to your depth, resulting in a value of `10`.
//! -   `forward 2` adds `2` to your horizontal position, a total of `15`.
//!
//! After following these instructions, you would have a horizontal position
//! of `15` and a depth of `10`. (Multiplying these together produces `150`.)
//!
//! Calculate the horizontal position and depth you would have after following the planned course.
//! **What do you get if you multiply your final horizontal position by your final depth?**
//!
//! # Part Two
//!
//! Based on your calculations, the planned course doesn't seem to make any sense. 
//! You find the submarine manual and discover that the process is actually slightly more complicated.
//!
//! In addition to horizontal position and depth, you'll also need to track a third value, aim,
//! which also starts at `0`. The commands also mean something entirely different
//! than you first thought:
//!
//! -   `down X` increases your aim by `X` units.
//! -   `up X` decreases your aim by `X` units.
//! -   `forward X` does two things:
//!     -   It increases your horizontal position by `X` units.
//!     -   It increases your depth by your aim multiplied by `X`.
//!
//! Again note that since you're on a submarine, `down` and `up` do the opposite of what you
//! might expect: "down" means aiming in the positive direction.
//!
//! Now, the above example does something different:
//!
//! -   `forward 5` adds `5` to your horizontal position, a total of `5`.
//!     Because your aim is `0`, your depth does not change.
//! -   `down 5` adds `5` to your aim, resulting in a value of `5`.
//! -   `forward 8` adds `8` to your horizontal position, a total of `13`.
//!     Because your aim is `5`, your depth increases by `8*5=40`.
//! -   `up 3` decreases your aim by `3`, resulting in a value of `2`.
//! -   `down 8` adds `8` to your aim, resulting in a value of `10`.
//! -   `forward 2` adds `2` to your horizontal position, a total of `15`.
//!     Because your aim is `10`, your depth increases by `2*10=20` to a total of `60`.
//!
//! After following these new instructions, you would have a horizontal position of `15` and
//! a depth of `60`. (Multiplying these produces `900`.)
//!
//! Using this new interpretation of the commands, calculate the horizontal position and depth
//! you would have after following the planned course.
//!
//! **What do you get if you multiply your final horizontal position by your final depth?**

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<Instr> {
    input.lines().map(|line| {
        let parts: Vec<&str> = line.split(" ").collect();
        match parts[0] {
            "forward" => Instr::Forward(parts[1].parse().unwrap()),
            "up" => Instr::Up(parts[1].parse().unwrap()),
            "down" => Instr::Down(parts[1].parse().unwrap()),
            _ => {panic!("wtf")}
        }
    }).collect()
}

fn execute1(instructions: &Vec<Instr>) -> (i64, i64) {
    let mut pos = (0,0);
    for instr in instructions {
        match instr {
            Instr::Forward(amount) => pos.0 += amount,
            Instr::Up(amount) => pos.1 -= amount,
            Instr::Down(amount) => pos.1 += amount,
        }
    }
    pos
}

fn execute2(instructions: &Vec<Instr>) -> (i64, i64) {
    let mut pos = (0,0);
    let mut aim = 0;
    for instr in instructions {
        match instr {
            Instr::Forward(amount) => {
                pos.0 += amount;
                pos.1 += aim * amount;
            },
            Instr::Up(amount) => aim -= amount,
            Instr::Down(amount) => aim += amount
        }
    }
    pos
}

/// What do you get if you multiply your final horizontal position by your final depth?
#[aoc(day2, part1)]
fn part1(input: &Vec<Instr>) -> i64 {
    let pos = execute1(input);
    pos.0 * pos.1
}

enum Instr {
    Forward(i64),
    Down(i64),
    Up(i64),
}

/// What do you get if you multiply your final horizontal position by your final depth?
#[aoc(day2, part2)]
fn part2(input: &Vec<Instr>) -> i64 {
    let pos = execute2(input);
    pos.0 * pos.1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn part1_examples() {
        let pos = execute1(&parse_input(EXAMPLE));
        assert_eq!((15, 10), pos);
    }

    #[test]
    fn part2_examples() {
        let pos = execute2(&parse_input(EXAMPLE));
        assert_eq!((15, 60), pos);
    }
}