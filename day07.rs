#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::str::FromStr;
use std::convert::TryInto;

mod day07_input;

#[derive(Debug)]
struct InputError { }
#[derive(Debug)]
struct Input { crabs: Vec<i32> }

impl FromStr for Input {
    type Err = InputError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = String::from(s);
        let crabs: Vec<i32> = s.split(',').map(|x| x.trim().parse::<i32>().unwrap()).collect();

        Ok(Input { crabs: crabs })
    }
}

fn main() {
    println!("Advent of Code 2021");
    println!("Day 07 - The Treachery of Whales");
    println!();

    println!("Part 1");
    println!("======");
    println!();

    let puzzle_test_out1 = do_part1(day07_input::TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", puzzle_test_out1, day07_input::TEST_RESULT_PART1);
    assert_eq!(day07_input::TEST_RESULT_PART1, puzzle_test_out1);

    println!();
    println!("Running puzzle input...");
    let puzzle_out1 = do_part1(day07_input::PUZZLE_INPUT.parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out1);
    println!();

    println!("Part 2");
    println!("======");
    println!();

    let test_out2 = do_part2(day07_input::TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", test_out2, day07_input::TEST_RESULT_PART2);
    assert_eq!(day07_input::TEST_RESULT_PART2, test_out2);

    println!();
    println!("Running puzzle input...");
    let puzzle_out2 = do_part2(day07_input::PUZZLE_INPUT.parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out2);
    println!();
}

fn do_part1(input: Input) -> i32 {
    let mut crabs = input.crabs.clone();

    // Find the median to get the most "fuel-efficient" location.
    let n = crabs.len();
    crabs.sort();
    let h = crabs[n/2];

    input.crabs.iter().map(|x| (h-x).abs()).sum::<i32>()
}

fn do_part2(input: Input) -> i32 {
    // Just iterate over everything to find the smallest fuel value.
    // It should be very near the arithmetic mean.
    let max = *input.crabs.iter().max().unwrap() as usize;
    let mut fuel_cost = vec![0; max + 1];
    for h in 0..=max {
        fuel_cost[h] = input.crabs.iter().map(|x| ((h as i32)-x).abs()).map(|x| x*(x+1)/2).sum::<i32>()
    }
    
    let (h,c) = fuel_cost.iter().enumerate().min_by_key(|x| x.1).unwrap();
    *c
}
