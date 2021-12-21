#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::str::FromStr;
use std::convert::TryInto;
use std::collections::HashMap;

mod day10_input;

const SCORES: [(char,i32); 4] = [
    (')', 3),
    (']', 57),
    ('}', 1197),
    ('>', 25137),
];

#[derive(Debug)]
struct InputError { }
#[derive(Debug)]
struct Input { lines: Vec<String> }

impl FromStr for Input {
    type Err = InputError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<String> = String::from(s).lines().map(|x| String::from(x)).collect();
        Ok(Input { lines: lines })
    }
}

fn main() {
    println!("Advent of Code 2021");
    println!("Day 10 - Syntax Scoring");
    println!();

    println!("Part 1");
    println!("======");
    println!();

    let puzzle_test_out1 = do_part1(day10_input::TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", puzzle_test_out1, day10_input::TEST_RESULT_PART1);
    assert_eq!(day10_input::TEST_RESULT_PART1, puzzle_test_out1);

    println!();
    println!("Running puzzle input...");
    let puzzle_out1 = do_part1(day10_input::PUZZLE_INPUT.parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out1);
    println!();

    println!("Part 2");
    println!("======");
    println!();

    let test_out2 = do_part2(day10_input::TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", test_out2, day10_input::TEST_RESULT_PART2);
    assert_eq!(day10_input::TEST_RESULT_PART2, test_out2);

    println!();
    println!("Running puzzle input...");
    let puzzle_out2 = do_part2(day10_input::PUZZLE_INPUT.parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out2);
    println!();
}

fn do_part1(input: Input) -> i32 {
    0
}

fn do_part2(input: Input) -> i32 {
    0
}

fn is_subsystem_incomplete(line: &String) -> bool {
    let cs: Vec<char> = line.chars().collect();

    for i in 0..cs.len() {
        let chunk_end = match cs[i] {
            '(' => Some(')'),
            '[' => Some(']'),
            '{' => Some('}'),
            '<' => Some('>'),
            _ => None
        }.unwrap();
        let mut depth = 1;
        for j = (i+1)..cs.len() {
            if cs[j] == cs[i] {
                depth += 1;
            } else if cs[j] == chunk_end {
                depth -= 1;
            }
        }
    }
    
    true
}
