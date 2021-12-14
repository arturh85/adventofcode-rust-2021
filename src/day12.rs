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
//!     /    \
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

use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::Graph;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Graph<CaveNode, ()> {
    let mut graph = Graph::new();
    let mut node_by_name: HashMap<String, NodeIndex> = HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split('-').map(|p| p).collect();
        let from_name = parts[0];
        let to_name = parts[1];
        if !node_by_name.contains_key(from_name) {
            let idx = graph.add_node(CaveNode::parse(from_name));
            node_by_name.insert(from_name.to_string(), idx);
        }
        if !node_by_name.contains_key(to_name) {
            let idx = graph.add_node(CaveNode::parse(to_name));
            node_by_name.insert(to_name.to_string(), idx);
        }
        let from_idx = node_by_name[from_name];
        let to_idx = node_by_name[to_name];
        graph.add_edge(from_idx, to_idx, ());
        graph.add_edge(to_idx, from_idx, ());
    }
    graph
}

/// Part 1: How many paths through this cave system are there that visit small caves at most once?
#[aoc(day12, part1)]
fn part1(input: &Graph<CaveNode, ()>) -> usize {
    build_paths(input, 1).len()
}

/// Part 2: Given these new rules, how many paths through this cave system are there?
#[aoc(day12, part2)]
fn part2(input: &Graph<CaveNode, ()>) -> usize {
    build_paths(input, 2).len()
}

#[derive(Clone, Debug)]
enum CaveNode {
    Start,
    End,
    SmallCave(String),
    BigCave(String),
}

impl CaveNode {
    fn parse(name: &str) -> CaveNode {
        match name {
            "start" => CaveNode::Start,
            "end" => CaveNode::End,
            name => match name.chars().nth(0).unwrap().is_lowercase() {
                true => CaveNode::SmallCave(name.to_string()),
                false => CaveNode::BigCave(name.to_string()),
            },
        }
    }
}

fn decend(
    graph: &Graph<CaveNode, ()>,
    visited: Arc<RefCell<Vec<Vec<NodeIndex>>>>,
    mut small_caves: HashMap<NodeIndex, u8>,
    max_small_caves: u8,
    path: Vec<NodeIndex>,
) -> Vec<Vec<NodeIndex>> {
    let mut paths = Vec::new();
    let last_idx = path[path.len() - 1];
    let last_node = graph.node_weight(last_idx).unwrap();
    if let CaveNode::SmallCave(_) = last_node {
        if !small_caves.contains_key(&last_idx) {
            small_caves.insert(last_idx, 0);
        }
        *small_caves.get_mut(&last_idx).unwrap() += 1;
    }
    for edge in graph.edges(last_idx) {
        let target_idx = edge.target();
        let target_node = graph.node_weight(target_idx).unwrap();

        match target_node {
            CaveNode::Start => {}
            CaveNode::End => {
                let mut new_path = path.clone();
                new_path.push(target_idx);
                paths.push(new_path);
            }
            CaveNode::SmallCave(_) => {
                if small_caves.contains_key(&target_idx) {
                    let cnt = small_caves[&target_idx];
                    if cnt >= max_small_caves {
                        continue;
                    } else if max_small_caves > 1 {
                        let mut found_bad = false;
                        for key in small_caves.keys() {
                            if *key != target_idx && small_caves[&key] >= max_small_caves {
                                found_bad = true;
                            }
                        }
                        if found_bad {
                            continue;
                        }
                    }
                }
            }
            _ => {}
        };
        match target_node {
            CaveNode::SmallCave(_) | CaveNode::BigCave(_) => {
                let mut new_path = path.clone();
                new_path.push(target_idx);
                if !visited.borrow().contains(&new_path) {
                    visited.borrow_mut().push(new_path.clone());
                    paths.append(&mut decend(
                        graph,
                        visited.clone(),
                        small_caves.clone(),
                        max_small_caves,
                        new_path,
                    ));
                }
            }
            _ => {}
        };
    }
    paths
}

fn build_paths(graph: &Graph<CaveNode, ()>, max_small_caves: u8) -> Vec<Vec<NodeIndex>> {
    let mut paths = Vec::new();
    let history = Arc::new(RefCell::new(Vec::new()));

    for node_idx in graph.node_indices() {
        match graph.node_weight(node_idx).unwrap() {
            CaveNode::Start => {
                for edge in graph.edges(node_idx) {
                    let target_idx = edge.target();
                    let small_caves = HashMap::new();
                    paths.append(&mut decend(
                        &graph,
                        history.clone(),
                        small_caves,
                        max_small_caves,
                        vec![node_idx, target_idx],
                    ));
                }

                // let mut dfs = Dfs::new(&graph, node_idx);
                // while let Some(nx) = dfs.next(&graph) {}

                break;
            }
            _ => {}
        }
    }
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cave_name(cave_node: &CaveNode) -> &str {
        match cave_node {
            CaveNode::Start => "start",
            CaveNode::End => "end",
            CaveNode::SmallCave(name) => name,
            CaveNode::BigCave(name) => name,
        }
    }

    const EXAMPLE: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

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
        let grid = parse_input(EXAMPLE);

        let actual_paths: Vec<String> = build_paths(&grid, 1)
            .iter()
            .map(|c| {
                let foo: Vec<String> = c
                    .iter()
                    .map(|f| cave_name(grid.node_weight(*f).unwrap()).to_string())
                    .collect();
                foo.join(",")
            })
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
        let grid = parse_input(EXAMPLE);

        let actual_paths: Vec<String> = build_paths(&grid, 2)
            .iter()
            .map(|nodes| {
                let path: Vec<String> = nodes
                    .iter()
                    .map(|f| cave_name(grid.node_weight(*f).unwrap()).to_string())
                    .collect();
                path.join(",")
            })
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
