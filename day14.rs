use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::str::FromStr;
use std::convert::TryInto;

use crate::day14_input;

#[derive(Debug)]
struct InputError { }
#[derive(Debug)]
struct Input { seed: String, insertions: Vec<(String, char)> }

const ITERATIONS_PART1: usize = 10;
const ITERATIONS_PART2: usize = 40;

impl FromStr for Input {
    type Err = InputError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = String::from(s);
        let mut lines = s.lines();
        let seed = String::from(lines.nth(0).unwrap());
        let insertions: Vec<(String, char)> = lines.skip(1).map(|l| {
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

fn build_polymer(input: Input, iterations: usize) -> String {
    let mut polymer = input.seed.clone();
    for _ in 0..iterations {
        let mut insertions: Vec<(usize,char)> = Vec::new();
        for (pair, insertion) in &input.insertions {
            for i in 0..(polymer.len()-1) {
                if polymer[i..i+2] == *pair.as_str() {
                    insertions.push((i+1, *insertion))
                }
            }
        }
        insertions.sort_by(|(i,_), (j,_)| j.cmp(i));
        for (i,c) in insertions {
            polymer.insert(i, c)
        }
    }
    polymer
}

fn get_polymer_strength(polymer: &String) -> i64 {
    let mut poly_chars: Vec<char> = polymer.chars().collect();
    poly_chars.sort_unstable();
    let mut unique_chars = poly_chars.clone();
    unique_chars.dedup();

    let mut min: usize = poly_chars.len();
    let mut max: usize = 0;

    for c in unique_chars {
        let n = poly_chars.iter().filter(|x| x == &&c).count();
        if n < min {
            min = n
        }
        if n > max {
            max = n
        }
    }

    (max-min).try_into().unwrap()
}

fn do_part1(input: Input) -> i64 {
    let polymer = build_polymer(input, ITERATIONS_PART1);
    get_polymer_strength(&polymer)
}

fn do_part2(input: Input) -> i64 {
    let polymer = build_polymer(input, ITERATIONS_PART2);
    get_polymer_strength(&polymer)
}
