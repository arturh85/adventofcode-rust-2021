//! # [Day 5: Hydrothermal Venture](https://adventofcode.com/2021/day/5)
//!
//! You come across a field of [hydrothermal vents](https://en.wikipedia.org/wiki/Hydrothermal_vent)
//! on the ocean floor! These vents constantly produce large, opaque clouds, so it would be best to
//! avoid them if possible.
//!
//! They tend to form in lines; the submarine helpfully produces a list of nearby lines of vents
//! (your puzzle input) for you to review. For example:
//!
//! ```plain
//! 0,9 -> 5,9
//! 8,0 -> 0,8
//! 9,4 -> 3,4
//! 2,2 -> 2,1
//! 7,0 -> 7,4
//! 6,4 -> 2,0
//! 0,9 -> 2,9
//! 3,4 -> 1,4
//! 0,0 -> 8,8
//! 5,5 -> 8,2
//!
//! ```
//!
//! Each line of vents is given as a line segment in the format `x1,y1 -> x2,y2` where `x1`,`y1`
//! are the coordinates of one end the line segment and `x2`,`y2` are the coordinates of the
//! other end. These line segments include the points at both ends. In other words:
//!
//! -   An entry like `1,1 -> 1,3` covers points `1,1`, `1,2`, and `1,3`.
//! -   An entry like `9,7 -> 7,7` covers points `9,7`, `8,7`, and `7,7`.
//!
//! For now, only consider horizontal and vertical lines: lines where either `x1 = x2` or `y1 = y2`.
//!
//! So, the horizontal and vertical lines from the above list would produce the following diagram:
//!
//! ```plain
//! .......1..
//! ..1....1..
//! ..1....1..
//! .......1..
//! .112111211
//! ..........
//! ..........
//! ..........
//! ..........
//! 222111....
//!
//! ```
//!
//! In this diagram, the top left corner is `0,0` and the bottom right corner is `9,9`.
//! Each position is shown as the number of lines which cover that point or `.` if no line
//! covers that point. The top-left pair of `1`s, for example, comes from `2,2 -> 2,1`; the
//! very bottom row is formed by the overlapping lines `0,9 -> 5,9` and `0,9 -> 2,9`.
//!
//! To avoid the most dangerous areas, you need to determine the number of points where at least
//! two lines overlap. In the above example, this is anywhere in the diagram with a `2` or
//! larger - a total of `5` points.
//!
//! Consider only horizontal and vertical lines. **At how many points do at least two lines overlap?**
//!
//! # Part Two
//!
//! Unfortunately, considering only horizontal and vertical lines doesn't give you the
//! full picture; you need to also consider diagonal lines.
//!
//! Because of the limits of the hydrothermal vent mapping system, the lines in your list will
//! only ever be horizontal, vertical, or a diagonal line at exactly 45 degrees. In other words:
//!
//! -   An entry like `1,1 -> 3,3` covers points `1,1`, `2,2`, and `3,3`.
//! -   An entry like `9,7 -> 7,9` covers points `9,7`, `8,8`, and `7,9`.
//!
//! Considering all lines from the above example would now produce the following diagram:
//!
//! ```plain
//! 1.1....11.
//! .111...2..
//! ..2.1.111.
//! ...1.2.2..
//! .112313211
//! ...1.2....
//! ..1...1...
//! .1.....1..
//! 1.......1.
//! 222111....
//! ```
//!
//! You still need to determine the number of points where at least two lines overlap.
//! In the above example, this is still anywhere in the diagram with a `2` or larger - now a
//! total of `12` points.
//!
//! Consider all of the lines. **At how many points do at least two lines overlap?**

use grid::Grid;

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Vec<Line> {
    input.lines().map(|line|{
        Line::parse(line)
    }).collect()
}

/// At how many points do at least two lines overlap?
#[aoc(day5, part1)]
fn part1(input: &Vec<Line>) -> usize {
    let non_diagonals: Vec<Line> = input.clone().into_iter().filter(|line| !line.is_diagonal()).collect();
    let grid = build_grid(&non_diagonals);
    grid.iter().filter(|v| **v >= 2).count()
}

/// At how many points do at least two lines overlap?
#[aoc(day5, part2)]
fn part2(input: &Vec<Line>) -> usize {
    let grid = build_grid(input);
    grid.iter().filter(|v| **v >= 2).count()
}


#[derive(Debug, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn parse(input: &str) -> Point {
        let parts: Vec<&str> = input.split(',').collect();
        Point {
            x: parts[0].parse().unwrap(),
            y: parts[1].parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
struct Line {
    begin: Point,
    end: Point,
}

impl Line {
    fn parse(input: &str) -> Line {
        let parts: Vec<&str> = input.split(" -> ").collect();
        Line {
            begin: Point::parse(parts[0]),
            end: Point::parse(parts[1]),
        }
    }

    fn is_diagonal(&self) -> bool {
        !(self.begin.x == self.end.x || self.begin.y == self.end.y)
    }

    fn build_points(&self) -> Vec<Point> {
        let mut arr = Vec::new();
        if self.begin.x == self.end.x {
            for y in range(self.begin.y, self.end.y) {
                arr.push(Point {
                    x: self.begin.x,
                    y
                })
            }
        } else if self.begin.y == self.end.y {
            for x in range(self.begin.x, self.end.x) {
                arr.push(Point {
                    x,
                    y: self.begin.y
                })
            }
        } else {
            let x_range = range(self.begin.x, self.end.x);
            let first = x_range.first().unwrap();
            let last = x_range.last().unwrap();
            let y_range = range(self.begin.y, self.end.y);
            for (idx, y) in y_range.iter().enumerate() {
                arr.push(Point {
                    x: if first < last {
                        first + idx
                    } else {
                        first - idx
                    },
                    y: *y,
                })
            }
        }
        arr
    }
}

fn range(a: usize, b: usize) -> Vec<usize> {
    if a < b {
        (a..=b).collect()
    } else {
        (b..=a).rev().collect()
    }
}

fn grid_size(lines: &[Line]) -> (usize, usize) {
    let mut cols = 0;
    let mut rows = 0;

    for line in lines {
        if line.begin.x > cols {
            cols = line.begin.x;
        }
        if line.end.x > cols {
            cols = line.end.x;
        }
        if line.begin.y > rows {
            rows = line.begin.y;
        }
        if line.end.y > rows {
            rows = line.end.y;
        }
    }

    (cols + 1, rows + 1)
}

fn build_grid(lines: &[Line]) -> Grid<u8> {
    let (cols, rows) = grid_size(lines);
    let mut grid = Grid::new(rows, cols);
    for line in lines {
        for point in line.build_points() {
            grid[point.y][point.x] += 1;
        }
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn part1_examples() {
        let lines = parse_input(EXAMPLE);
        assert_eq!(5, part1(&lines));
    }

    #[test]
    fn part2_examples() {
        let lines = parse_input(EXAMPLE);
        assert_eq!(12, part2(&lines));
    }
}
