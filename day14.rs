#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::str::FromStr;
use std::convert::TryInto;

use crate::day14_input;

#[derive(Debug)]
struct InputError { }
#[derive(Debug)]
struct Input { seed: String, insertions: Vec<(String, char)> }

const ITERATIONS: usize = 10;

impl FromStr for Input {
    type Err = InputError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = String::from(s);
        let mut lines = s.lines();
        let seed = String::from(lines.nth(0).unwrap());
        let insertions: Vec<(String, char)> = lines.skip(2).map(|l| {
            let x: Vec<&str> = l.split("->").collect();
            (String::from(x[0].trim()), x[1].trim().chars().nth(0).unwrap())
        }).collect();

        Ok(Input { seed, insertions })
    }
}

pub fn main() {
    println!("Advent of Code 2021");
    println!("Day 14 - Extended Polymerization");
    println!();

    println!("Part 1");
    println!("======");
    println!();

    let puzzle_test_out1 = do_part1(day14_input::TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", puzzle_test_out1, day14_input::TEST_RESULT_PART1);
    assert_eq!(day14_input::TEST_RESULT_PART1, puzzle_test_out1);

    println!();
    println!("Running puzzle input...");
    let puzzle_out1 = do_part1(day14_input::PUZZLE_INPUT.parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out1);
    println!();

    println!("Part 2");
    println!("======");
    println!();

    let test_out2 = do_part2(day14_input::TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", test_out2, day14_input::TEST_RESULT_PART2);
    assert_eq!(day14_input::TEST_RESULT_PART2, test_out2);

    println!();
    println!("Running puzzle input...");
    let puzzle_out2 = do_part2(day14_input::PUZZLE_INPUT.parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out2);
    println!();
}

fn do_part1(input: Input) -> i32 {
    let mut polymer = input.seed.clone();
    for _ in 0..ITERATIONS {
        let mut new_poly = polymer.clone();
        for (pair, insertion) in input.insertions.iter() {
            for m in polymer.matches(pair) {
                
            }
        }
    }
    0
}

fn do_part2(input: Input) -> i32 {
    0
}
