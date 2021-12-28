#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::str::FromStr;
use std::convert::TryInto;

use ndarray::arr2;

mod day13_input;
mod dbgprint;

#[derive(Debug)]
enum FoldDirection {
    Horizontal,
    Vertical
}

#[derive(Debug)]
struct InputError { }
#[derive(Debug)]
struct Input { dots: Vec<(usize, usize)>, folds: Vec<(FoldDirection, usize)> }

impl FromStr for Input {
    type Err = InputError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        enum InputState { Dots, Folds }

        let s = String::from(s);
        let mut state = InputState::Dots;

        let mut dots: Vec<(usize, usize)> = Vec::new();
        let mut folds: Vec<(FoldDirection, usize)> = Vec::new();

        for l in s.lines() {
            match state {
                InputState::Dots => {
                    let l = l.trim();
                    if l.len() > 0 {
                        let coords = l.trim().split(',').collect::<Vec<&str>>();
                        dots.push((coords[0].parse::<usize>().unwrap(), coords[1].parse::<usize>().unwrap()));
                    } else {
                        state = InputState::Folds;
                    }
                },
                InputState::Folds => {
                    let l = l.trim().strip_prefix("fold along ").unwrap();
                    let line = l.split('=').collect::<Vec<&str>>();
                    if line[0] == "x" {
                        folds.push((FoldDirection::Vertical, line[1].parse::<usize>().unwrap()));
                    } else if line[0] == "y" {
                        folds.push((FoldDirection::Horizontal, line[1].parse::<usize>().unwrap()));
                    }
                }
            }
        }

        Ok(Input { dots, folds })
    }
}

fn main() {
    dbgprint::enable();

    println!("Advent of Code 2021");
    println!("Day 13 - Transparent Origami");
    println!();

    println!("Part 1");
    println!("======");
    println!();

    let puzzle_test_out1 = do_part1(day13_input::TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", puzzle_test_out1, day13_input::TEST_RESULT_PART1);
    assert_eq!(day13_input::TEST_RESULT_PART1, puzzle_test_out1);

    println!();
    println!("Running puzzle input...");
    let puzzle_out1 = do_part1(day13_input::PUZZLE_INPUT.parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out1);
    println!();

    println!("Part 2");
    println!("======");
    println!();

    let test_out2 = do_part2(day13_input::TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", test_out2, day13_input::TEST_RESULT_PART2);
    assert_eq!(day13_input::TEST_RESULT_PART2, test_out2);

    println!();
    println!("Running puzzle input...");
    let puzzle_out2 = do_part2(day13_input::PUZZLE_INPUT.parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out2);
    println!();
}

fn do_part1(input: Input) -> i32 {
    0
}

fn do_part2(input: Input) -> i32 {
    0
}
