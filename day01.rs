//use std::vec;
use std::convert::TryInto;
use std::iter::FromIterator;

use crate::day01_input;

const TEST_INPUT: [i32; 10] = [
    199,
    200,
    208,
    210,
    200,
    207,
    240,
    269,
    260,
    263
];
const TEST_RESULT_PART1: i32 = 7;
const TEST_RESULT_PART2: i32 = 5;

struct Input(Vec<i32>);


pub fn main() {
    println!("Advent of Code");
    println!("Day 01 - Sonar Sweep");
    println!();

    println!("Part 1");
    println!("======");
    println!();

    let puzzle_test_out1 = do_part1(&Input(Vec::from(TEST_INPUT)));
    println!("Test output: {} (expected {})", puzzle_test_out1, TEST_RESULT_PART1);
    assert_eq!(TEST_RESULT_PART1, puzzle_test_out1);

    println!();
    println!("Running puzzle input...");
    let puzzle_out1 = do_part1(&Input(Vec::from(day01_input::PUZZLE_INPUT)));
    println!("Puzzle result: {}", puzzle_out1);
    println!();

    println!("Part 2");
    println!("======");
    println!();

    let test_out2 = do_part2(&Input(Vec::from(TEST_INPUT)));
    println!("Test output: {} (expected {})", test_out2, TEST_RESULT_PART2);
    assert_eq!(TEST_RESULT_PART2, test_out2);

    println!();
    println!("Running puzzle input...");
    let puzzle_out2 = do_part2(&Input(Vec::from(day01_input::PUZZLE_INPUT)));
    println!("Puzzle result: {}", puzzle_out2);
    println!();
}

fn do_part1(input: &Input) -> i32 {
    let depths = &input.0;
    let pairs = depths.iter()
                    .take(depths.len()-1)
                    .zip(depths.iter().skip(1));
    pairs
        .map(|x| x.1-x.0)
        .filter(|x| *x > 0)
        .count()
        .try_into().unwrap()
}

fn do_part2(input: &Input) -> i32 {
    let depths = &input.0;
    let triples = depths.iter()
                    .take(depths.len()-2)
                    .zip(depths.iter().skip(1).take(depths.len()-1))
                    .zip(depths.iter().skip(2))
                    .map(|x| (x.0.0, x.0.1, x.1));
    let sums = Vec::from_iter(triples.map(|x| x.2+x.1+x.0));
    let sum_pairs = sums.iter()
                    .take(sums.len()-1)
                    .zip(sums.iter().skip(1));
    sum_pairs
        .map(|x| x.1-x.0)
        .filter(|x| *x > 0)
        .count()
        .try_into().unwrap()
}
