//! [Day 14: Extended Polymerization](https://adventofcode.com/2021/day/14)
//!
//! The incredible pressures at this depth are starting to put a strain on your submarine.
//! The submarine has [polymerization](https://en.wikipedia.org/wiki/Polymerization) equipment that
//! would produce suitable materials to reinforce the submarine, and the nearby volcanically-active
//! caves should even have the necessary input elements in sufficient quantities.
//!
//! The submarine manual contains instructions for finding the optimal polymer formula;
//! specifically, it offers a polymer template and a list of pair insertion rules
//! (your puzzle input). You just need to work out what polymer would result after repeating the
//! pair insertion process a few times.
//!
//! For example:
//!
//! ```plain
//! NNCB
//!
//! CH -> B
//! HH -> N
//! CB -> H
//! NH -> C
//! HB -> C
//! HC -> B
//! HN -> C
//! NN -> C
//! BH -> H
//! NC -> B
//! NB -> B
//! BN -> B
//! BB -> N
//! BC -> B
//! CC -> N
//! CN -> C
//! ```
//!
//! The first line is the polymer template - this is the starting point of the process.
//!
//! The following section defines the pair insertion rules. A rule like `AB -> C` means that when
//! elements `A` and `B` are immediately adjacent, element `C` should be inserted between them.
//! These insertions all happen simultaneously.
//!
//! So, starting with the polymer template `NNCB`, the first step simultaneously considers
//! all three pairs:
//!
//! -   The first pair (`NN`) matches the rule `NN -> C`, so element `C` is inserted
//!     between the first `N` and the second `N`.
//! -   The second pair (`NC`) matches the rule `NC -> B`, so element `B` is inserted
//!     between the `N` and the `C`.
//! -   The third pair (`CB`) matches the rule `CB -> H`, so element `H` is inserted
//!     between the `C` and the `B`.
//!
//! Note that these pairs overlap: the second element of one pair is the first element of the
//! next pair. Also, because all pairs are considered simultaneously, inserted elements are not
//! considered to be part of a pair until the next step.
//!
//! After the first step of this process, the polymer becomes `NCNBCHB`.
//!
//! Here are the results of a few steps using the above rules:
//!
//! ```plain
//! Template:     NNCB
//! After step 1: NCNBCHB
//! After step 2: NBCCNBBBCBHCB
//! After step 3: NBBBCNCCNBBNBNBBCHBHHBCHB
//! After step 4: NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB
//!
//! ```
//!
//! This polymer grows quickly. After step 5, it has length 97; After step 10, it has length 3073.
//! After step 10, `B` occurs 1749 times, `C` occurs 298 times, `H` occurs 161 times, and `N`
//! occurs 865 times; taking the quantity of the most common element (`B`, 1749) and subtracting the
//! quantity of the least common element (`H`, 161) produces `1749 - 161 = 1588`.
//!
//! Apply 10 steps of pair insertion to the polymer template and find the most and least common
//! elements in the result.
//! **What do you get if you take the quantity of the most common element and subtract the quantity
//! of the least common element?**
//!
//! # Part Two
//!
//! The resulting polymer isn't nearly strong enough to reinforce the submarine. You'll need to run
//! more steps of the pair insertion process; a total of 40 steps should do it.
//!
//! In the above example, the most common element is `B` (occurring `2192039569602` times) and the
//! least common element is `H` (occurring `3849876073` times); subtracting
//! these produces `2188189693529`.
//!
//! Apply 40 steps of pair insertion to the polymer template and find the most and least common
//! elements in the result. What do you get if you take the quantity of the most common
//! element and subtract the quantity of the least common element?

use std::collections::HashMap;

// NNCB -> NNCHB

#[aoc_generator(day14)]
fn parse_input(input: &str) -> Polymer {
    let mut start = String::new();
    let mut rules: Vec<(char, char, char)> = Vec::new();

    for (idx, line) in input.lines().enumerate() {
        match idx {
            0 => start = line.to_string(),
            1 => {}
            _ => {
                let parts: Vec<String> = line.split(" -> ").map(|c| c.to_string()).collect();
                rules.push((
                    parts[0].chars().nth(0).unwrap(),
                    parts[0].chars().nth(1).unwrap(),
                    parts[1].chars().nth(0).unwrap(),
                ));
            }
        }
    }
    Polymer { rules, start }
}

/// Part 1: What do you get if you take the quantity of the most common element and subtract the
/// quantity of the least common element?
#[aoc(day14, part1)]
fn part1(input: &Polymer) -> usize {
    let str = evolve1(input, 10);
    let frequencies = map_char_frequences(&str);
    let min = *frequencies.values().min().unwrap();
    let max = *frequencies.values().max().unwrap();
    max - min
}

/// Part 2
#[aoc(day14, part2)]
fn part2(input: &Polymer) -> usize {
    let frequencies = evolve2(input, 40);
    let min = *frequencies.values().min().unwrap();
    let max = *frequencies.values().max().unwrap();
    max - min
}

fn map_char_frequences(input: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for char in input.chars() {
        if !map.contains_key(&char) {
            map.insert(char, 0);
        }
        *map.get_mut(&char).unwrap() += 1;
    }
    map
}

fn evolve1(input: &Polymer, steps: usize) -> String {
    let mut state: Vec<char> = input.start.chars().map(|c| c).collect();
    for _step in 0..steps {
        let mut next_state = state.clone();
        let mut idx = 1;
        let mut inserts: Vec<(usize, char)> = Vec::new();
        for w in state.windows(2) {
            for rule in &input.rules {
                if w[0] == rule.0 && w[1] == rule.1 {
                    inserts.push((idx, rule.2));
                }
            }
            idx += 1;
        }
        inserts.reverse();
        for (idx, char) in &inserts {
            next_state.insert(*idx, *char);
        }

        state = next_state
    }
    String::from_iter(state)
}

fn evolve2(input: &Polymer, steps: usize) -> HashMap<char, usize> {
    let mut frequencies = map_char_frequences(&input.start);
    let chars: Vec<char> = input.start.chars().map(|c| c).collect();
    let windows: Vec<(char, char)> = chars.windows(2).map(|w| (w[0], w[1])).collect();
    let mut window_map: HashMap<(char, char), usize> = HashMap::new();
    for w in windows {
        if !window_map.contains_key(&w) {
            window_map.insert(w, 0);
        }
        *window_map.get_mut(&w).unwrap() += 1;
    }
    for _ in 0..steps {
        let mut next_state = window_map.clone();

        for rule in &input.rules {
            // AB -> C
            let ab = (rule.0, rule.1);
            let ac = (rule.0, rule.2);
            let cb = (rule.2, rule.1);
            if window_map.contains_key(&ab) && window_map[&ab] > 0 {
                let cnt = window_map[&ab];
                if !frequencies.contains_key(&rule.2) {
                    frequencies.insert(rule.2, 0);
                }
                *frequencies.get_mut(&rule.2).unwrap() += cnt;
                *next_state.get_mut(&ab).unwrap() -= cnt;
                if !next_state.contains_key(&ac) {
                    next_state.insert(ac, 0);
                }
                *next_state.get_mut(&ac).unwrap() += cnt;
                if !next_state.contains_key(&cb) {
                    next_state.insert(cb, 0);
                }
                *next_state.get_mut(&cb).unwrap() += cnt;
            }
        }
        window_map = next_state;
    }
    frequencies
}

struct Polymer {
    start: String,
    rules: Vec<(char, char, char)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn part1_examples() {
        // Template:     NNCB
        // After step 1: NCNBCHB
        assert_eq!("NCNBCHB", evolve1(&parse_input(EXAMPLE), 1));
        // After step 2: NBCCNBBBCBHCB
        assert_eq!("NBCCNBBBCBHCB", evolve1(&parse_input(EXAMPLE), 2));
        // After step 3: NBBBCNCCNBBNBNBBCHBHHBCHB
        assert_eq!(
            "NBBBCNCCNBBNBNBBCHBHHBCHB",
            evolve1(&parse_input(EXAMPLE), 3)
        );
        // After step 4: NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB
        assert_eq!(
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB",
            evolve1(&parse_input(EXAMPLE), 4)
        );
        assert_eq!(1588, part1(&parse_input(EXAMPLE)));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(2188189693529, part2(&parse_input(EXAMPLE)));
    }
}
