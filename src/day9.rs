//! # [Day 9: Smoke Basin](https://adventofcode.com/2021/day/9)
//!
//! These caves seem to be [lava tubes](https://en.wikipedia.org/wiki/Lava_tube).
//! Parts are even still volcanically active; small hydrothermal vents release smoke into the
//! caves that slowly settles like rain.
//!
//! If you can model how the smoke flows through the caves, you might be able to avoid it and be
//! that much safer. The submarine generates a heightmap of the floor of the
//! nearby caves for you (your puzzle input).
//!
//! Smoke flows to the lowest point of the area it's in.
//! For example, consider the following heightmap:
//!
//! ```plain
//! 2199943210
//! 3987894921
//! 9856789892
//! 8767896789
//! 9899965678
//! ```
//!
//! Each number corresponds to the height of a particular location, where `9` is the highest
//! and `0` is the lowest a location can be.
//!
//! Your first goal is to find the low points - the locations that are lower than any of its
//! adjacent locations. Most locations have four adjacent locations (up, down, left, and right);
//! locations on the edge or corner of the map have three or two adjacent locations, respectively.
//! (Diagonal locations do not count as adjacent.)
//!
//! In the above example, there are four low points, all highlighted: two are in the first
//! row (a `1` and a `0`), one is in the third row (a `5`), and one is in the bottom row
//! (also a `5`). All other locations on the heightmap have some lower adjacent location,
//! and so are not low points.
//!
//! The risk level of a low point is 1 plus its height. In the above example, the risk levels of
//! the low points are `2`, `1`, `6`, and `6`. The sum of the risk levels of all low points in the
//! heightmap is therefore `15`.
//!
//! Find all of the low points on your heightmap.
//! **What is the sum of the risk levels of all low points on your heightmap?**
//!
//! # Part Two
//!
//! Next, you need to find the largest basins so you know what areas are most important to avoid.
//!
//! A basin is all locations that eventually flow downward to a single low point.
//! Therefore, every low point has a basin, although some basins are very small.
//! Locations of height `9` do not count as being in any basin, and all other locations will
//! always be part of exactly one basin.
//!
//! The size of a basin is the number of locations within the basin, including the low point.
//! The example above has four basins.
//!
//! The top-left basin, size `3`:
//!
//! ```plain
//! 2199943210
//! 3987894921
//! 9856789892
//! 8767896789
//! 9899965678
//! ```
//!
//! The top-right basin, size `9`:
//!
//! ```plain
//! 2199943210
//! 3987894921
//! 9856789892
//! 8767896789
//! 9899965678
//! ```
//!
//! The middle basin, size `14`:
//!
//! ```plain
//! 2199943210
//! 3987894921
//! 9856789892
//! 8767896789
//! 9899965678
//! ```
//!
//! The bottom-right basin, size `9`:
//!
//! ```plain
//! 2199943210
//! 3987894921
//! 9856789892
//! 8767896789
//! 9899965678
//! ```
//!
//! Find the three largest basins and multiply their sizes together.
//! In the above example, this is `9 * 14 * 9 = 1134`.
//!
//! **What do you get if you multiply together the sizes of the three largest basins?**

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

/// Part 1: What is the sum of the risk levels of all low points on your heightmap?
#[aoc(day9, part1)]
fn part1(input: &Vec<Vec<u8>>) -> u64 {
    find_low_points(input).iter().sum()
}

/// Part 2: What do you get if you multiply together the sizes of the three largest basins?
#[aoc(day9, part2)]
fn part2(input: &Vec<Vec<u8>>) -> u64 {
    let mut basins = find_basins(input);
    basins.sort();
    basins.reverse();
    basins[0..3].iter().product()
}

fn get_neighbors(heightmap: &Vec<Vec<u8>>, pos: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut list = Vec::new();
    if pos.0 > 0 {
        list.push((pos.0 - 1, pos.1));
    }
    if pos.1 > 0 {
        list.push((pos.0, pos.1 - 1));
    }
    if pos.0 < heightmap.len() - 1 {
        list.push((pos.0 + 1, pos.1));
    }
    if pos.1 < heightmap[0].len() - 1 {
        list.push((pos.0, pos.1 + 1));
    }

    list
}

fn find_low_points(heightmap: &Vec<Vec<u8>>) -> Vec<u64> {
    let mut lows = Vec::new();
    for (y, row) in heightmap.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            let neighbors = get_neighbors(heightmap, &(y, x));
            let mut higher_found = false;
            for (ny, nx) in &neighbors {
                if *col >= heightmap[*ny][*nx] {
                    higher_found = true;
                    break;
                }
            }
            if higher_found {
                continue;
            }
            lows.push((*col + 1) as u64)
        }
    }
    lows
}

fn explore_basin(heightmap: &Vec<Vec<u8>>, start: (usize, usize)) -> u64 {
    let mut already_visited = Vec::new();
    already_visited.push(start);

    let mut to_explore = get_neighbors(heightmap, &start);
    let mut to_add: Vec<(usize, usize)> = Vec::new();

    while to_explore.len() > 0 {
        for (y, x) in &to_explore {
            let pos = (*y, *x);
            if heightmap[*y][*x] != 9 && !already_visited.contains(&pos) && !to_add.contains(&pos) {
                to_add.append(&mut get_neighbors(heightmap, &pos));
                already_visited.push(pos);
            }
        }
        to_explore = to_add;
        to_add = Vec::new();
    }

    already_visited.len() as u64
}

fn find_basins(heightmap: &Vec<Vec<u8>>) -> Vec<u64> {
    let mut basins = Vec::new();
    for (y, row) in heightmap.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            let neighbors = get_neighbors(heightmap, &(y, x));
            let mut higher_found = false;
            for (ny, nx) in &neighbors {
                if *col >= heightmap[*ny][*nx] {
                    higher_found = true;
                    break;
                }
            }
            if higher_found {
                continue;
            }
            basins.push(explore_basin(heightmap, (y, x)));
        }
    }
    basins
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn part1_examples() {
        assert_eq!(15, part1(&parse_input(EXAMPLE)));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(1134, part2(&parse_input(EXAMPLE)));
    }
}
