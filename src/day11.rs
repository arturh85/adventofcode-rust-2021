//! # [Day 11: Dumbo Octopus](https://adventofcode.com/2021/day/11)
//!
//! You enter a large cavern full of rare bioluminescent
//! [dumbo octopuses](https://www.youtube.com/watch?v=eih-VSaS2g0)! They seem to not like the
//! Christmas lights on your submarine, so you turn them off for now.
//!
//! There are 100 octopuses arranged neatly in a 10 by 10 grid. Each octopus slowly gains energy
//! over time and flashes brightly for a moment when its energy is full. Although your lights are
//! off, maybe you could navigate through the cave without disturbing the octopuses if you could
//! predict when the flashes of light will happen.
//!
//! Each octopus has an energy level - your submarine can remotely measure the energy level of each
//! octopus (your puzzle input). For example:
//!
//! ```plain
//! 5483143223
//! 2745854711
//! 5264556173
//! 6141336146
//! 6357385478
//! 4167524645
//! 2176841721
//! 6882881134
//! 4846848554
//! 5283751526
//! ```
//!
//! The energy level of each octopus is a value between `0` and `9`. Here, the top-left octopus has
//! an energy level of `5`, the bottom-right one has an energy level of `6`, and so on.
//!
//! You can model the energy levels and flashes of light in steps. During a single step, the
//! following occurs:
//!
//! -   First, the energy level of each octopus increases by `1`.
//! -   Then, any octopus with an energy level greater than `9` flashes. This increases the
//!     energy level of all adjacent octopuses by `1`, including octopuses that are diagonally
//!     adjacent. If this causes an octopus to have an energy level greater than `9`, it also
//!     flashes. This process continues as long as new octopuses keep having their energy level
//!     increased beyond `9`. (An octopus can only flash at most once per step.)
//! -   Finally, any octopus that flashed during this step has its energy level set to `0`,
//!     as it used all of its energy to flash.
//!
//! Adjacent flashes can cause an octopus to flash on a step even if it begins that step with
//! very little energy. Consider the middle octopus with `1` energy in this situation:
//!
//! ```plain
//! Before any steps:
//! 11111
//! 19991
//! 19191
//! 19991
//! 11111
//!
//! After step 1:
//! 34543
//! 40004
//! 50005
//! 40004
//! 34543
//!
//! After step 2:
//! 45654
//! 51115
//! 61116
//! 51115
//! 45654
//! ```
//!
//! An octopus is highlighted when it flashed during the given step.
//!
//! Here is how the larger example above progresses:
//!
//! ```plain
//! Before any steps:
//! 5483143223
//! 2745854711
//! 5264556173
//! 6141336146
//! 6357385478
//! 4167524645
//! 2176841721
//! 6882881134
//! 4846848554
//! 5283751526
//!
//! After step 1:
//! 6594254334
//! 3856965822
//! 6375667284
//! 7252447257
//! 7468496589
//! 5278635756
//! 3287952832
//! 7993992245
//! 5957959665
//! 6394862637
//!
//! After step 2:
//! 8807476555
//! 5089087054
//! 8597889608
//! 8485769600
//! 8700908800
//! 6600088989
//! 6800005943
//! 0000007456
//! 9000000876
//! 8700006848
//!
//! After step 3:
//! 0050900866
//! 8500800575
//! 9900000039
//! 9700000041
//! 9935080063
//! 7712300000
//! 7911250009
//! 2211130000
//! 0421125000
//! 0021119000
//!
//! After step 4:
//! 2263031977
//! 0923031697
//! 0032221150
//! 0041111163
//! 0076191174
//! 0053411122
//! 0042361120
//! 5532241122
//! 1532247211
//! 1132230211
//!
//! After step 5:
//! 4484144000
//! 2044144000
//! 2253333493
//! 1152333274
//! 1187303285
//! 1164633233
//! 1153472231
//! 6643352233
//! 2643358322
//! 2243341322
//!
//! After step 6:
//! 5595255111
//! 3155255222
//! 3364444605
//! 2263444496
//! 2298414396
//! 2275744344
//! 2264583342
//! 7754463344
//! 3754469433
//! 3354452433
//!
//! After step 7:
//! 6707366222
//! 4377366333
//! 4475555827
//! 3496655709
//! 3500625609
//! 3509955566
//! 3486694453
//! 8865585555
//! 4865580644
//! 4465574644
//!
//! After step 8:
//! 7818477333
//! 5488477444
//! 5697666949
//! 4608766830
//! 4734946730
//! 4740097688
//! 6900007564
//! 0000009666
//! 8000004755
//! 6800007755
//!
//! After step 9:
//! 9060000644
//! 7800000976
//! 6900000080
//! 5840000082
//! 5858000093
//! 6962400000
//! 8021250009
//! 2221130009
//! 9111128097
//! 7911119976
//!
//! After step 10:
//! 0481112976
//! 0031112009
//! 0041112504
//! 0081111406
//! 0099111306
//! 0093511233
//! 0442361130
//! 5532252350
//! 0532250600
//! 0032240000
//! ```
//!
//! After step 10, there have been a total of `204` flashes. Fast forwarding, here is the same
//! configuration every 10 steps:
//!
//! ```plain
//! After step 20:
//! 3936556452
//! 5686556806
//! 4496555690
//! 4448655580
//! 4456865570
//! 5680086577
//! 7000009896
//! 0000000344
//! 6000000364
//! 4600009543
//!
//! After step 30:
//! 0643334118
//! 4253334611
//! 3374333458
//! 2225333337
//! 2229333338
//! 2276733333
//! 2754574565
//! 5544458511
//! 9444447111
//! 7944446119
//!
//! After step 40:
//! 6211111981
//! 0421111119
//! 0042111115
//! 0003111115
//! 0003111116
//! 0065611111
//! 0532351111
//! 3322234597
//! 2222222976
//! 2222222762
//!
//! After step 50:
//! 9655556447
//! 4865556805
//! 4486555690
//! 4458655580
//! 4574865570
//! 5700086566
//! 6000009887
//! 8000000533
//! 6800000633
//! 5680000538
//!
//! After step 60:
//! 2533334200
//! 2743334640
//! 2264333458
//! 2225333337
//! 2225333338
//! 2287833333
//! 3854573455
//! 1854458611
//! 1175447111
//! 1115446111
//!
//! After step 70:
//! 8211111164
//! 0421111166
//! 0042111114
//! 0004211115
//! 0000211116
//! 0065611111
//! 0532351111
//! 7322235117
//! 5722223475
//! 4572222754
//!
//! After step 80:
//! 1755555697
//! 5965555609
//! 4486555680
//! 4458655580
//! 4570865570
//! 5700086566
//! 7000008666
//! 0000000990
//! 0000000800
//! 0000000000
//!
//! After step 90:
//! 7433333522
//! 2643333522
//! 2264333458
//! 2226433337
//! 2222433338
//! 2287833333
//! 2854573333
//! 4854458333
//! 3387779333
//! 3333333333
//!
//! After step 100:
//! 0397666866
//! 0749766918
//! 0053976933
//! 0004297822
//! 0004229892
//! 0053222877
//! 0532222966
//! 9322228966
//! 7922286866
//! 6789998766
//!
//! After 100 steps, there have been a total of 1656 flashes.
//! Given the starting energy levels of the dumbo octopuses in your cavern, simulate 100 steps.
//!
//! **How many total flashes are there after 100 steps?**
//!
//! # Part Two
//!
//! It seems like the individual flashes aren't bright enough to navigate.
//! However, you might have a better option: the flashes seem to be synchronizing!
//!
//! In the example above, the first time all octopuses flash simultaneously is step `195`:
//!
//! ```plain
//! After step 193:
//! 5877777777
//! 8877777777
//! 7777777777
//! 7777777777
//! 7777777777
//! 7777777777
//! 7777777777
//! 7777777777
//! 7777777777
//! 7777777777
//!
//! After step 194:
//! 6988888888
//! 9988888888
//! 8888888888
//! 8888888888
//! 8888888888
//! 8888888888
//! 8888888888
//! 8888888888
//! 8888888888
//! 8888888888
//!
//! After step 195:
//! 0000000000
//! 0000000000
//! 0000000000
//! 0000000000
//! 0000000000
//! 0000000000
//! 0000000000
//! 0000000000
//! 0000000000
//! 0000000000
//! ```
//!
//! If you can calculate the exact moments when the octopuses will all flash simultaneously,
//! you should be able to navigate through the cavern.
//!
//! **What is the first step during which all octopuses flash?**

#[aoc_generator(day11)]
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

/// Part 1: Given the starting energy levels of the dumbo octopuses in your cavern,
/// simulate 100 steps. How many total flashes are there after 100 steps?
#[aoc(day11, part1)]
fn part1(input: &Vec<Vec<u8>>) -> usize {
    let (flashes, _grid) = evolve(input, 100);
    flashes
}

/// Part 2: What is the first step during which all octopuses flash?
#[aoc(day11, part2)]
fn part2(input: &Vec<Vec<u8>>) -> usize {
    let mut step = 0;
    let mut state = input.clone();
    loop {
        let (_, new_state) = evolve(&state, 1);
        state = new_state;
        step += 1;
        if state
            .iter()
            .map(|row| row.iter().map(|v| *v as u64).sum::<u64>())
            .sum::<u64>()
            == 0
        {
            break;
        }
    }

    step
}

/// You can model the energy levels and flashes of light in steps. During a single step,
/// the following occurs:
///
/// - First, the energy level of each octopus increases by `1`.
/// - Then, any octopus with an energy level greater than `9` flashes. This increases the energy
///   level of all adjacent octopuses by `1`, including octopuses that are diagonally adjacent.
///   If this causes an octopus to have an energy level greater than `9`, it also flashes.
///   This process continues as long as new octopuses keep having their energy level
///   increased beyond `9`. (An octopus can only flash at most once per step.)
/// - Finally, any octopus that flashed during this step has its energy level set to `0`,
///   as it used all of its energy to flash.
///
/// Adjacent flashes can cause an octopus to flash on a step even if it begins that
/// step with very little energy.
fn evolve(energy_map: &Vec<Vec<u8>>, steps: usize) -> (usize, Vec<Vec<u8>>) {
    let mut state = energy_map.clone();
    let mut flashes = 0;
    for _ in 0..steps {
        let mut next_state = zero_grid(energy_map);
        let mut step_flashes: Vec<(usize, usize)> = Vec::new();
        // println!("state {:?}", state);
        for (y, row) in state.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if *col < 9 {
                    next_state[y][x] = *col + 1;
                } else {
                    next_state[y][x] = 0;
                    step_flashes.push((y, x));
                }
            }
        }
        let mut todo_flashes = step_flashes.clone();
        while todo_flashes.len() > 0 {
            let flash = todo_flashes.pop().unwrap();
            for (y, x) in get_neighbors(&state, &flash) {
                if step_flashes.contains(&(y, x)) {
                    continue;
                }

                let value = next_state[y][x];
                if value < 9 {
                    next_state[y][x] = value + 1;
                } else {
                    next_state[y][x] = 0;
                    if !step_flashes.contains(&(y, x)) {
                        todo_flashes.push((y, x));
                        step_flashes.push((y, x));
                    }
                }
            }
        }

        // println!("next state {:?}", next_state);
        state = next_state;
        flashes += step_flashes.len();
    }
    (flashes, state)
}

fn zero_grid(energy_map: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    for row in energy_map {
        let mut outrow = Vec::new();
        for _ in row {
            outrow.push(0);
        }
        out.push(outrow);
    }
    out
}

fn get_neighbors(energy_map: &Vec<Vec<u8>>, pos: &(usize, usize)) -> Vec<(usize, usize)> {
    let mut list = Vec::new();
    if pos.0 > 0 {
        list.push((pos.0 - 1, pos.1));
    }
    if pos.1 > 0 {
        list.push((pos.0, pos.1 - 1));
    }
    if pos.0 < energy_map.len() - 1 {
        list.push((pos.0 + 1, pos.1));
    }
    if pos.1 < energy_map[0].len() - 1 {
        list.push((pos.0, pos.1 + 1));
    }
    if pos.0 > 0 && pos.1 > 0 {
        list.push((pos.0 - 1, pos.1 - 1));
    }
    if pos.0 > 0 && pos.1 < energy_map[0].len() - 1 {
        list.push((pos.0 - 1, pos.1 + 1));
    }
    if pos.0 < energy_map.len() - 1 && pos.1 > 0 {
        list.push((pos.0 + 1, pos.1 - 1));
    }
    if pos.0 < energy_map.len() - 1 && pos.1 < energy_map[0].len() - 1 {
        list.push((pos.0 + 1, pos.1 + 1));
    }
    list
}

fn string_evolve(energy_map: &Vec<Vec<u8>>, steps: usize) -> (usize, String) {
    let (flashes, grid) = evolve(energy_map, steps);
    (flashes, stringify(&grid))
}

fn stringify(energy_map: &Vec<Vec<u8>>) -> String {
    let mut out = String::new();
    for (idx, row) in energy_map.iter().enumerate() {
        let mut line = String::new();
        for (_, col) in row.iter().enumerate() {
            line += &col.to_string();
        }
        out += &line;
        if idx < energy_map.len() - 1 {
            out += "\n";
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_EXAMPLE_0: &str = "11111
19991
19191
19991
11111";
    const SMALL_EXAMPLE_1: &str = "34543
40004
50005
40004
34543";

    const SMALL_EXAMPLE_2: &str = "45654
51115
61116
51115
45654";

    const EXAMPLE_0: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    const EXAMPLE_1: &str = "6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637";

    const EXAMPLE_2: &str = "8807476555
5089087054
8597889608
8485769600
8700908800
6600088989
6800005943
0000007456
9000000876
8700006848";

    const EXAMPLE_3: &str = "0050900866
8500800575
9900000039
9700000041
9935080063
7712300000
7911250009
2211130000
0421125000
0021119000";

    const EXAMPLE_4: &str = "2263031977
0923031697
0032221150
0041111163
0076191174
0053411122
0042361120
5532241122
1532247211
1132230211";

    const EXAMPLE_5: &str = "4484144000
2044144000
2253333493
1152333274
1187303285
1164633233
1153472231
6643352233
2643358322
2243341322";

    const EXAMPLE_6: &str = "5595255111
3155255222
3364444605
2263444496
2298414396
2275744344
2264583342
7754463344
3754469433
3354452433";

    const EXAMPLE_7: &str = "6707366222
4377366333
4475555827
3496655709
3500625609
3509955566
3486694453
8865585555
4865580644
4465574644";

    const EXAMPLE_8: &str = "7818477333
5488477444
5697666949
4608766830
4734946730
4740097688
6900007564
0000009666
8000004755
6800007755";

    const EXAMPLE_9: &str = "9060000644
7800000976
6900000080
5840000082
5858000093
6962400000
8021250009
2221130009
9111128097
7911119976";

    const EXAMPLE_10: &str = "0481112976
0031112009
0041112504
0081111406
0099111306
0093511233
0442361130
5532252350
0532250600
0032240000";

    const EXAMPLE_20: &str = "3936556452
5686556806
4496555690
4448655580
4456865570
5680086577
7000009896
0000000344
6000000364
4600009543";

    const EXAMPLE_30: &str = "0643334118
4253334611
3374333458
2225333337
2229333338
2276733333
2754574565
5544458511
9444447111
7944446119";

    const EXAMPLE_40: &str = "6211111981
0421111119
0042111115
0003111115
0003111116
0065611111
0532351111
3322234597
2222222976
2222222762";

    const EXAMPLE_50: &str = "9655556447
4865556805
4486555690
4458655580
4574865570
5700086566
6000009887
8000000533
6800000633
5680000538";

    const EXAMPLE_60: &str = "2533334200
2743334640
2264333458
2225333337
2225333338
2287833333
3854573455
1854458611
1175447111
1115446111";

    const EXAMPLE_70: &str = "8211111164
0421111166
0042111114
0004211115
0000211116
0065611111
0532351111
7322235117
5722223475
4572222754";

    const EXAMPLE_80: &str = "1755555697
5965555609
4486555680
4458655580
4570865570
5700086566
7000008666
0000000990
0000000800
0000000000";

    const EXAMPLE_90: &str = "7433333522
2643333522
2264333458
2226433337
2222433338
2287833333
2854573333
4854458333
3387779333
3333333333";

    const EXAMPLE_100: &str = "0397666866
0749766918
0053976933
0004297822
0004229892
0053222877
0532222966
9322228966
7922286866
6789998766";

    #[test]
    fn part1_small_examples() {
        assert_eq!(
            string_evolve(&parse_input(SMALL_EXAMPLE_0), 1),
            (9, SMALL_EXAMPLE_1.into()),
        );
        assert_eq!(
            string_evolve(&parse_input(SMALL_EXAMPLE_0), 2),
            (9, SMALL_EXAMPLE_2.into())
        );
        assert_eq!(
            string_evolve(&parse_input(SMALL_EXAMPLE_1), 1),
            (0, SMALL_EXAMPLE_2.into())
        );
    }

    #[test]
    fn part1_big_examples() {
        assert_eq!(
            string_evolve(&parse_input(EXAMPLE_0), 1),
            (0, EXAMPLE_1.into())
        );
        assert_eq!(
            string_evolve(&parse_input(EXAMPLE_0), 2),
            (35, EXAMPLE_2.into())
        );
        assert_eq!(
            string_evolve(&parse_input(EXAMPLE_0), 3),
            (80, EXAMPLE_3.into())
        );
        assert_eq!(
            string_evolve(&parse_input(EXAMPLE_0), 4),
            (96, EXAMPLE_4.into())
        );
        assert_eq!(
            string_evolve(&parse_input(EXAMPLE_0), 5),
            (104, EXAMPLE_5.into())
        );
        assert_eq!(
            string_evolve(&parse_input(EXAMPLE_0), 6),
            (105, EXAMPLE_6.into())
        );
        assert_eq!(
            string_evolve(&parse_input(EXAMPLE_0), 7),
            (112, EXAMPLE_7.into())
        );
        assert_eq!(
            string_evolve(&parse_input(EXAMPLE_0), 8),
            (136, EXAMPLE_8.into())
        );
        assert_eq!(
            string_evolve(&parse_input(EXAMPLE_0), 9),
            (175, EXAMPLE_9.into())
        );
        // After step 10, there have been a total of `204` flashes.
        assert_eq!(
            string_evolve(&parse_input(EXAMPLE_0), 10),
            (204, EXAMPLE_10.into())
        );
        assert_eq!(
            string_evolve(&parse_input(EXAMPLE_0), 20),
            (344, EXAMPLE_20.into())
        );
        assert_eq!(
            string_evolve(&parse_input(EXAMPLE_0), 30),
            (513, EXAMPLE_30.into())
        );
        assert_eq!(
            string_evolve(&parse_input(EXAMPLE_0), 40),
            (703, EXAMPLE_40.into())
        );
        assert_eq!(
            string_evolve(&parse_input(EXAMPLE_0), 50),
            (842, EXAMPLE_50.into())
        );
        assert_eq!(
            string_evolve(&parse_input(EXAMPLE_0), 60),
            (1016, EXAMPLE_60.into())
        );
        assert_eq!(
            string_evolve(&parse_input(EXAMPLE_0), 70),
            (1202, EXAMPLE_70.into())
        );
        assert_eq!(
            string_evolve(&parse_input(EXAMPLE_0), 80),
            (1344, EXAMPLE_80.into())
        );
        assert_eq!(
            string_evolve(&parse_input(EXAMPLE_0), 90),
            (1505, EXAMPLE_90.into())
        );
        // After 100 steps, there have been a total of 1656 flashes.
        assert_eq!(
            string_evolve(&parse_input(EXAMPLE_0), 100),
            (1656, EXAMPLE_100.into())
        );
    }

    #[test]
    fn part2_examples() {
        // In the example above, the first time all octopuses flash simultaneously is step `195`:
        assert_eq!(part2(&parse_input(EXAMPLE_0)), 195);
    }
}
