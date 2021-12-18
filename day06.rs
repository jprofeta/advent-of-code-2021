#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::str::FromStr;
use std::convert::TryInto;

mod day06_input;

const DAYS_PART1: i32 = 80;
const DAYS_PART2: i32 = 256;
const OLD_FISH_PERIOD: usize = 6;
const NEW_FISH_PERIOD: usize = OLD_FISH_PERIOD + 2;

#[derive(Debug)]
struct InputError { }
#[derive(Debug)]
struct Input { fish: Vec<i32> }

impl FromStr for Input {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = String::from(s);
        let f: Vec<i32> = s.split(',').map(|x| x.trim().parse::<i32>().unwrap()).collect();
        Ok(Input { fish: f })
    }
}

fn main() {
    println!("Advent of Code 2021");
    println!("Day 06 - Lanternfish");
    println!();

    println!("Part 1");
    println!("======");
    println!();

    let puzzle_test_out1 = do_part1(day06_input::TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", puzzle_test_out1, day06_input::TEST_RESULT_PART1);
    assert_eq!(day06_input::TEST_RESULT_PART1, puzzle_test_out1);

    println!();
    println!("Running puzzle input...");
    let puzzle_out1 = do_part1(day06_input::PUZZLE_INPUT.parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out1);
    println!();

    println!("Part 2");
    println!("======");
    println!();

    let test_out2 = do_part2(day06_input::TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", test_out2, day06_input::TEST_RESULT_PART2);
    assert_eq!(day06_input::TEST_RESULT_PART2, test_out2);

    println!();
    println!("Running puzzle input...");
    let puzzle_out2 = do_part2(day06_input::PUZZLE_INPUT.parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out2);
    println!();
}

// This is pretty ineffecient way to go about it. Part 2 is much better.
fn do_part1(input: Input) -> i32 {
    let mut fish = input.fish;
    for i in 0..DAYS_PART1 {
        let new_fish = fish.iter().filter(|x| **x == 0).count();
        fish = fish.iter().map(|x| { let x = x - 1; if x < 0 { OLD_FISH_PERIOD as i32} else { x } }).collect();
        fish.append(&mut vec![NEW_FISH_PERIOD as i32; new_fish]);
    }
    fish.len() as i32
}

fn do_part2(input: Input) -> i64 {
    let fish = input.fish;
    let mut fish_at_age: Vec<i64> = vec![0; NEW_FISH_PERIOD+1];

    // Initialize the fish age counts
    for i in 0..=OLD_FISH_PERIOD {
        fish_at_age[i] = fish.iter().filter(|x| **x == (i as i32)).count() as i64;
    }

    for d in 0..DAYS_PART2 {
        // Move the fish counts down to the next day
        // Save the zero fish before moving since it is needed to modify the array
        let day0_fish = fish_at_age[0];
        for i in 1..=NEW_FISH_PERIOD {
            fish_at_age[i-1] = fish_at_age[i];
        }
        // Birth some fish equal to the number of day0 fish
        fish_at_age[NEW_FISH_PERIOD] = day0_fish;
        // Now add the day0 fish back into the count
        fish_at_age[OLD_FISH_PERIOD] += day0_fish;
    }

    // Add up all the fish
    fish_at_age.iter().sum::<i64>()
}
