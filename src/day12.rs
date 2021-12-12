//! # [Day 12: Passage Pathing](https://adventofcode.com/2021/day/12)
//!
//! With your submarine's subterranean subsystems subsisting suboptimally, the only way you're
//! getting out of this cave anytime soon is by finding a path yourself. Not just a path - the only
//! way to know if you've found the best path is to find all of them.
//!
//! Fortunately, the sensors are still mostly working, and so you build a rough map of the
//! remaining caves (your puzzle input).
//!
//! For example:
//!
//! ```plain
//! start-A
//! start-b
//! A-c
//! A-b
//! b-d
//! A-end
//! b-end
//! ```
//!
//! This is a list of how all of the caves are connected. You start in the cave named `start`,
//! and your destination is the cave named `end`. An entry like `b-d` means that cave `b` is
//! connected to cave `d` - that is, you can move between them.
//!
//! So, the above cave system looks roughly like this:
//!
//! ```plain
//!     start
//!     /\
//! c--A-----b--d
//!     \   /
//!      end
//!
//! ```
//!
//! Your goal is to find the number of distinct paths that start at `start`, end at `end`, and
//! don't visit small caves more than once. There are two types of caves:
//! big caves (written in uppercase, like `A`) and small caves (written in lowercase, like `b`).
//! It would be a waste of time to visit any small cave more than once, but big caves are large
//! enough that it might be worth visiting them multiple times. So, all paths you find should
//! visit small caves at most once, and can visit big caves any number of times.
//!
//! Given these rules, there are `10` paths through this example cave system:
//!
//! ```plain
//! start,A,b,A,c,A,end
//! start,A,b,A,end
//! start,A,b,end
//! start,A,c,A,b,A,end
//! start,A,c,A,b,end
//! start,A,c,A,end
//! start,A,end
//! start,b,A,c,A,end
//! start,b,A,end
//! start,b,end
//!
//! ```
//!
//! (Each line in the above list corresponds to a single path; the caves visited by that path are
//! listed in the order they are visited and separated by commas.)
//!
//! Note that in this cave system, cave `d` is never visited by any path: to do so, cave `b`
//! would need to be visited twice (once on the way to cave `d` and a second time when returning
//! from cave `d`), and since cave `b` is small, this is not allowed.
//!
//! Here is a slightly larger example:
//!
//! ```plain
//! dc-end
//! HN-start
//! start-kj
//! dc-start
//! dc-HN
//! LN-dc
//! HN-end
//! kj-sa
//! kj-HN
//! kj-dc
//! ```
//!
//! The `19` paths through it are as follows:
//!
//! ```plain
//! start,HN,dc,HN,end
//! start,HN,dc,HN,kj,HN,end
//! start,HN,dc,end
//! start,HN,dc,kj,HN,end
//! start,HN,end
//! start,HN,kj,HN,dc,HN,end
//! start,HN,kj,HN,dc,end
//! start,HN,kj,HN,end
//! start,HN,kj,dc,HN,end
//! start,HN,kj,dc,end
//! start,dc,HN,end
//! start,dc,HN,kj,HN,end
//! start,dc,end
//! start,dc,kj,HN,end
//! start,kj,HN,dc,HN,end
//! start,kj,HN,dc,end
//! start,kj,HN,end
//! start,kj,dc,HN,end
//! start,kj,dc,end
//! ```
//!
//! Finally, this even larger example has `226` paths through it:
//!
//! ```plain
//! fs-end
//! he-DX
//! fs-he
//! start-DX
//! pj-DX
//! end-zg
//! zg-sl
//! zg-pj
//! pj-he
//! RW-he
//! fs-DX
//! pj-RW
//! zg-RW
//! start-pj
//! he-WI
//! zg-he
//! pj-fs
//! start-RW
//! ```
//!
//! **How many paths through this cave system are there that visit small caves at most once?**
//!
//! # Part Two
//!
//! After reviewing the available paths, you realize you might have time to visit a single small
//! cave twice. Specifically, big caves can be visited any number of times, a single small cave
//! can be visited at most twice, and the remaining small caves can be visited at most once.
//! However, the caves named `start` and `end` can only be visited exactly once each: once you
//! leave the `start` cave, you may not return to it, and once you reach the `end` cave,
//! the path must end immediately.
//!
//! Now, the `36` possible paths through the first example above are:
//!
//! ```plain
//! start,A,b,A,b,A,c,A,end
//! start,A,b,A,b,A,end
//! start,A,b,A,b,end
//! start,A,b,A,c,A,b,A,end
//! start,A,b,A,c,A,b,end
//! start,A,b,A,c,A,c,A,end
//! start,A,b,A,c,A,end
//! start,A,b,A,end
//! start,A,b,d,b,A,c,A,end
//! start,A,b,d,b,A,end
//! start,A,b,d,b,end
//! start,A,b,end
//! start,A,c,A,b,A,b,A,end
//! start,A,c,A,b,A,b,end
//! start,A,c,A,b,A,c,A,end
//! start,A,c,A,b,A,end
//! start,A,c,A,b,d,b,A,end
//! start,A,c,A,b,d,b,end
//! start,A,c,A,b,end
//! start,A,c,A,c,A,b,A,end
//! start,A,c,A,c,A,b,end
//! start,A,c,A,c,A,end
//! start,A,c,A,end
//! start,A,end
//! start,b,A,b,A,c,A,end
//! start,b,A,b,A,end
//! start,b,A,b,end
//! start,b,A,c,A,b,A,end
//! start,b,A,c,A,b,end
//! start,b,A,c,A,c,A,end
//! start,b,A,c,A,end
//! start,b,A,end
//! start,b,d,b,A,c,A,end
//! start,b,d,b,A,end
//! start,b,d,b,end
//! start,b,end
//!
//! ```
//!
//! The slightly larger example above now has `103` paths through it, and the even larger example
//! now has `3509` paths through it.
//!
//! **Given these new rules, how many paths through this cave system are there?**

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split('-').map(|p| p).collect();
            (parts[0].to_string(), parts[1].to_string())
        })
        .collect()
}

/// Part 1: How many paths through this cave system are there that visit small caves at most once?
#[aoc(day12, part1)]
fn part1(input: &Vec<(String, String)>) -> usize {
    build_paths(input, 1).len()
}

/// Part 2: Given these new rules, how many paths through this cave system are there?
#[aoc(day12, part2)]
fn part2(input: &Vec<(String, String)>) -> usize {
    build_paths(input, 2).len()
}

fn map_by_start(input: &Vec<(String, String)>) -> HashMap<String, Vec<(String, String)>> {
    let mut by_start: HashMap<String, Vec<(String, String)>> = HashMap::new();
    for (from, to) in input {
        if !by_start.contains_key(from) {
            by_start.insert(from.to_string(), Vec::new());
        }
        if !by_start.contains_key(to) {
            by_start.insert(to.to_string(), Vec::new());
        }
        by_start
            .get_mut(from)
            .unwrap()
            .push((from.to_string(), to.to_string()));
        by_start
            .get_mut(to)
            .unwrap()
            .push((to.to_string(), from.to_string()));
    }
    by_start
}

fn is_small_cave(name: &str) -> bool {
    match name {
        "start" | "end" => false,
        name => name.chars().nth(0).unwrap().is_lowercase(),
    }
}

fn decend(
    edges: &HashMap<String, Vec<(String, String)>>,
    visited: Arc<Mutex<Vec<String>>>,
    mut small_caves: HashMap<String, u8>,
    max_small_caves: u8,
    path: Vec<String>,
) -> Vec<Vec<String>> {
    let mut paths = Vec::new();
    let last = path.last().unwrap();
    if is_small_cave(last) {
        if !small_caves.contains_key(last) {
            small_caves.insert(last.to_string(), 0);
        }
        *small_caves.get_mut(last).unwrap() += 1;
    }
    if edges.contains_key(last) {
        for (_, next) in &edges[last] {
            let mut new_path = path.clone();
            new_path.push(next.to_string());
            if next == "start" {
            } else if next == "end" {
                paths.push(new_path);
            } else {
                if is_small_cave(next) && small_caves.contains_key(next) {
                    let cnt = small_caves[next];
                    if cnt >= max_small_caves {
                        continue;
                    } else if max_small_caves > 1 {
                        let mut found_bad = false;
                        for key in small_caves.keys() {
                            if key != next && small_caves[key] >= max_small_caves {
                                found_bad = true;
                            }
                        }
                        if found_bad {
                            continue;
                        }
                    }
                }
                let str_path = new_path.join(",");
                let mut breadcrumb = visited.lock().unwrap();
                if !breadcrumb.contains(&str_path) {
                    breadcrumb.push(str_path);
                    drop(breadcrumb);
                    paths.append(&mut decend(
                        edges,
                        visited.clone(),
                        small_caves.clone(),
                        max_small_caves,
                        new_path,
                    ));
                }
            }
        }
    }
    paths
}

fn build_paths(input: &Vec<(String, String)>, max_small_caves: u8) -> Vec<Vec<String>> {
    let mut paths = Vec::new();
    let by_start = map_by_start(input);
    let history = Arc::new(Mutex::new(Vec::new()));

    for (_, first) in &by_start["start"] {
        let small_caves = HashMap::new();
        paths.append(&mut decend(
            &by_start,
            history.clone(),
            small_caves,
            max_small_caves,
            vec!["start".into(), first.to_string()],
        ));
    }

    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    #[test]
    fn is_small_cave_test() {
        assert_eq!(is_small_cave("start"), false);
        assert_eq!(is_small_cave("end"), false);
        assert_eq!(is_small_cave("foo"), true);
        assert_eq!(is_small_cave("Bar"), false);
    }

    #[test]
    fn part1_examples() {
        let expected_paths = vec![
            "start,A,b,A,c,A,end",
            "start,A,b,A,end",
            "start,A,b,end",
            "start,A,c,A,b,A,end",
            "start,A,c,A,b,end",
            "start,A,c,A,end",
            "start,A,end",
            "start,b,A,c,A,end",
            "start,b,A,end",
            "start,b,end",
        ];

        let actual_paths: Vec<String> = build_paths(&parse_input(EXAMPLE), 1)
            .iter()
            .map(|c| c.join(","))
            .collect();
        for actual_path in &actual_paths {
            assert!(
                expected_paths.contains(&&**actual_path),
                "invalid path: {:?}",
                actual_path
            );
            // println!("valid: {:?}", actual_path);
        }
        for expected_path in expected_paths {
            if !actual_paths.contains(&expected_path.to_string()) {
                println!("missing: {:?}", expected_path);
            }
        }
        assert_eq!(actual_paths.len(), 10);
    }

    #[test]
    fn part2_examples() {
        let expected_paths = vec![
            "start,A,b,A,b,A,c,A,end",
            "start,A,b,A,b,A,end",
            "start,A,b,A,b,end",
            "start,A,b,A,c,A,b,A,end",
            "start,A,b,A,c,A,b,end",
            "start,A,b,A,c,A,c,A,end",
            "start,A,b,A,c,A,end",
            "start,A,b,A,end",
            "start,A,b,d,b,A,c,A,end",
            "start,A,b,d,b,A,end",
            "start,A,b,d,b,end",
            "start,A,b,end",
            "start,A,c,A,b,A,b,A,end",
            "start,A,c,A,b,A,b,end",
            "start,A,c,A,b,A,c,A,end",
            "start,A,c,A,b,A,end",
            "start,A,c,A,b,d,b,A,end",
            "start,A,c,A,b,d,b,end",
            "start,A,c,A,b,end",
            "start,A,c,A,c,A,b,A,end",
            "start,A,c,A,c,A,b,end",
            "start,A,c,A,c,A,end",
            "start,A,c,A,end",
            "start,A,end",
            "start,b,A,b,A,c,A,end",
            "start,b,A,b,A,end",
            "start,b,A,b,end",
            "start,b,A,c,A,b,A,end",
            "start,b,A,c,A,b,end",
            "start,b,A,c,A,c,A,end",
            "start,b,A,c,A,end",
            "start,b,A,end",
            "start,b,d,b,A,c,A,end",
            "start,b,d,b,A,end",
            "start,b,d,b,end",
            "start,b,end",
        ];
        let actual_paths: Vec<String> = build_paths(&parse_input(EXAMPLE), 2)
            .iter()
            .map(|c| c.join(","))
            .collect();
        for actual_path in &actual_paths {
            assert!(
                expected_paths.contains(&&**actual_path),
                "invalid path: {:?}",
                actual_path
            );
            // println!("valid: {:?}", actual_path);
        }
        for expected_path in expected_paths {
            if !actual_paths.contains(&expected_path.to_string()) {
                println!("missing: {:?}", expected_path);
            }
        }
        assert_eq!(actual_paths.len(), 36);
    }
}
