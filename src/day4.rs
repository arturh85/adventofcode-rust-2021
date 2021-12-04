//! # [Day 4: Giant Squid](https://adventofcode.com/2021/day/4)
//!
//! You're already almost 1.5km (almost a mile) below the surface of the ocean, already so deep
//! that you can't see any sunlight. What you can see, however, is a giant squid that has attached
//! itself to the outside of your submarine.
//!
//! Maybe it wants to play [bingo](https://en.wikipedia.org/wiki/Bingo_(American_version))?
//!
//! Bingo is played on a set of boards each consisting of a 5x5 grid of numbers.
//! Numbers are chosen at random, and the chosen number is marked on all boards on which it appears.
//! (Numbers may not appear on all boards.) If all numbers in any row or any column of a board
//! are marked, that board wins. (Diagonals don't count.)
//!
//! The submarine has a bingo subsystem to help passengers (currently, you and the giant squid)
//! pass the time. It automatically generates a random order in which to draw numbers and a
//! random set of boards (your puzzle input). For example:
//!
//! ```plain
//! 7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1
//!
//! 22 13 17 11  0
//!  8  2 23  4 24
//! 21  9 14 16  7
//!  6 10  3 18  5
//!  1 12 20 15 19
//!
//!  3 15  0  2 22
//!  9 18 13 17  5
//! 19  8  7 25 23
//! 20 11 10 24  4
//! 14 21 16 12  6
//!
//! 14 21 17 24  4
//! 10 16 15  9 19
//! 18  8 23 26 20
//! 22 11 13  6  5
//!  2  0 12  3
//! ```
//!
//! After the first five numbers are drawn (`7`, `4`, `9`, `5`, and `11`),
//! there are no winners, but the boards are marked as follows
//! (shown here adjacent to each other to save space):
//!
//! ```plain
//! 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//!  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
//! 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//!  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//!  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
//! ```
//!
//! After the next six numbers are drawn (`17`, `23`, `2`, `0`, `14`, and `21`),
//! there are still no winners:
//!
//! ```plain
//! 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//!  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
//! 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//!  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//!  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
//! ```
//!
//! Finally, `24` is drawn:
//!
//! ```plain
//! 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//!  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
//! 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//!  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//!  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
//! ```
//!
//! At this point, the third board wins because it has at least one complete row or column of
//! marked numbers (in this case, the entire top row is marked: `14 21 17 24 4`).
//!
//! The score of the winning board can now be calculated. Start by finding the sum of all unmarked
//! numbers on that board; in this case, the sum is `188`. Then, multiply that sum by the number
//! that was just called when the board won, `24`, to get the final score, `188 * 24 = 4512`.
//!
//! To guarantee victory against the giant squid, figure out which board will win first.
//!
//! **What will your final score be if you choose that board?**
//!
//! # Part Two
//!
//! On the other hand, it might be wise to try a different strategy: let the giant squid win.
//!
//! You aren't sure how many bingo boards a giant squid could play at once, so rather than
//! waste time counting its arms, the safe thing to do is to figure out which board will win
//! last and choose that one. That way, no matter which boards it picks, it will win for sure.
//!
//! In the above example, the second board is the last to win, which happens after `13` is
//! eventually called and its middle column is completely marked. If you were to keep playing
//! until this point, the second board would have a sum of unmarked numbers equal to `148` for
//! a final score of `148 * 13 = 1924`.
//!
//! **Figure out which board will win last. Once it wins, what would its final score be?**

const BOARD_SIZE: usize = 5;

#[derive(Clone)]
struct BingoBoard {
    values: [[u8; BOARD_SIZE]; BOARD_SIZE],
    marked: [[bool; BOARD_SIZE]; BOARD_SIZE],
}

impl BingoBoard {
    fn new(board: &str) -> BingoBoard {
        let mut values = [[0; BOARD_SIZE]; BOARD_SIZE];
        let marked = [[false; BOARD_SIZE]; BOARD_SIZE];
        for (y, line) in board.lines().enumerate() {
            if line.is_empty() {
                break
            }
            for (x, value) in line.split(" ").enumerate() {
                values[x][y] = value.parse().unwrap();
            }
        }
        BingoBoard {
            values,
            marked,
        }
    }

    fn play(&mut self, n: u8) -> Option<u64> {
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                if self.values[x][y] == n {
                    self.marked[x][y] = true;
                    if self.check_win() {
                        return Some(self.score());
                    }
                }
            }
        }
        None
    }

    fn score(&self) -> u64 {
        let mut score = 0;
        for x in 0..BOARD_SIZE {
            for y in 0..BOARD_SIZE {
                if !self.marked[x][y] {
                    score += self.values[x][y] as u64
                }
            }
        }
        score
    }

    fn check_win(&self) -> bool {
        for a in 0..BOARD_SIZE {
            let mut count_a = 0;
            let mut count_b = 0;
            for b in 0..BOARD_SIZE {
                if self.marked[a][b] {
                    count_a += 1;
                }
                if self.marked[b][a] {
                    count_b += 1;
                }
            }
            if count_a == BOARD_SIZE || count_b == BOARD_SIZE {
                return true
            }
        }
        false
    }
}

struct BingoGame {
    numbers: Vec<u8>,
    boards: Vec<BingoBoard>,
}

impl BingoGame {
    fn play_win_first(&self) -> Option<u64> {
        let mut boards = self.boards.clone();
        for n in &self.numbers {
            for b in boards.iter_mut() {
                if let Some(score) = b.play(*n) {
                    return Some(*n as u64 * score);
                }
            }
        }
        None
    }
    fn play_win_last(&self) -> Option<u64> {
        let mut boards = self.boards.clone();
        for n in &self.numbers {
            let mut idx_to_remove: Vec<usize> = Vec::new();
            let count = boards.len();
            for (idx, b) in boards.iter_mut().enumerate() {
                if let Some(score) = b.play(*n) {
                    if count == 1 {
                        return Some(*n as u64 * score);
                    } else {
                        idx_to_remove.push(idx);
                    }
                }
            }
            let mut offset = 0;
            for idx in idx_to_remove {
                boards.remove(idx - offset);
                offset += 1;
            }
        }
        None
    }
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> BingoGame {
    let mut boards = Vec::new();
    let mut numbers = Vec::new();
    let mut buffer = String::new();
    for (nr, line) in input.lines().enumerate() {
        if nr == 0 {
            numbers = line.split(",").map(|c| c.parse().unwrap()).collect();
        } else if nr > 1 {
            buffer += &*line.replace("  ", " ").trim();
            buffer += "\n";
            if (nr - 2) % 6 == 5 {
                boards.push(BingoBoard::new(&buffer));
                buffer = String::new();
            }
        }
    }
    boards.push(BingoBoard::new(&buffer));

    BingoGame {
        boards,
        numbers
    }
}

/// Figure out which board will win first. What will your final score be if you choose that board?
#[aoc(day4, part1)]
fn part1(game: &BingoGame) -> u64 {
    game.play_win_first().expect("no winner")
}

/// Figure out which board will win last. Once it wins, what would its final score be?
#[aoc(day4, part2)]
fn part2(game: &BingoGame) -> u64 {
    game.play_win_last().expect("no winner")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3";

    #[test]
    fn part1_examples() {
        let result = parse_input(EXAMPLE).play_win_first().expect("no winner");
        assert_eq!(4512, result);
    }

    #[test]
    fn part2_examples() {
        let result = parse_input(EXAMPLE).play_win_last().expect("no winner");
        assert_eq!(1924, result);
    }
}
