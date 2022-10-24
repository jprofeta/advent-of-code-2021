#![allow(unused_imports)]

use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::str::FromStr;
use std::convert::TryInto;

use crate::day03_input;

struct Input { width: usize, diag: Vec<String> }

fn make_input<'a, I>(val: I) -> Input
where I: IntoIterator<Item=&'a str> {
    let strs = Vec::from_iter(val.into_iter().map(|a| String::from(a)));
    let width = strs[0].len();
    Input { width: width, diag: strs }
}

pub fn main() {
    println!("Advent of Code");
    println!("Day 03 - Binary Diagnostic");
    println!();

    println!("Part 1");
    println!("======");
    println!();

    let puzzle_test_out1 = do_part1(make_input(day03_input::TEST_INPUT));
    println!("Test output: {} (expected {})", puzzle_test_out1, day03_input::TEST_RESULT_PART1);
    assert_eq!(day03_input::TEST_RESULT_PART1, puzzle_test_out1);

    println!();
    println!("Running puzzle input...");
    let puzzle_out1 = do_part1(make_input(day03_input::PUZZLE_INPUT));
    println!("Puzzle result: {}", puzzle_out1);
    println!();

    println!("Part 2");
    println!("======");
    println!();

    let test_out2 = do_part2(make_input(day03_input::TEST_INPUT));
    println!("Test output: {} (expected {})", test_out2, day03_input::TEST_RESULT_PART2);
    assert_eq!(day03_input::TEST_RESULT_PART2, test_out2);

    println!();
    println!("Running puzzle input...");
    let puzzle_out2 = do_part2(make_input(day03_input::PUZZLE_INPUT));
    println!("Puzzle result: {}", puzzle_out2);
    println!();
}

fn do_part1(input: Input) -> i32 {
    let cnt: i32 = input.diag.len().try_into().unwrap();
    let width: i32 = input.width.try_into().unwrap();
    let mut sums = vec![0; input.width];

    for msg in input.diag.into_iter() {
        msg.chars().enumerate().for_each(|(i,c)| {
            if c == '1' { sums[i] = sums[i] + 1; }
        });
    }
    let gamma =
        sums
            .iter()
            .map(|s| {
                if s >= &(cnt / 2) { 1 }
                else { 0 } 
            }).fold(0, |acc,b| {
                (acc << 1) | b
            });
    let epsilon = (!gamma) & ((1 << width) - 1);
    gamma * epsilon
}

fn do_part2(input: Input) -> i32 {
    let mut o2_list: Vec<String> = input.diag.clone();
    for i in 0..input.width {
        if o2_list.len() > 1 {
            o2_list = reduce_list(o2_list, i, true);
        } else { break; }
    }
    
    let mut co2_list = input.diag.clone();
    for i in 0..input.width {
        if co2_list.len() > 1 {
            co2_list = reduce_list(co2_list, i, false);
        } else { break; }
    }

    let o2_gen = i32::from_str_radix(o2_list[0].as_str(), 2).unwrap();
    let co2_scrub = i32::from_str_radix(co2_list[0].as_str(), 2).unwrap();

    o2_gen * co2_scrub
}

fn reduce_list(val: Vec<String>, bit_pos: usize, take_larger: bool) -> Vec<String> {
    let n = val.len() as i32;
    let limit = (n + 1) / 2;    // Do an integer-ceiling operation
    let bit_sum = 
        val.iter()
            .map(|x| x.chars().nth(bit_pos).unwrap())
            .fold(0, |acc,x| {
                if x == '1' { acc + 1 }
                else { acc }
            });

    // bit_sum will be greator than limit if it is mostly 1s.
    // it will be less than limit if it is mostly 0s.
    if (bit_sum >= limit && take_larger) || (bit_sum < limit && !take_larger) {
        Vec::from_iter(
            val
            .iter()
            .filter(|x| x.chars().nth(bit_pos).unwrap() == '1')
            .map(|x| x.clone()))
    } else {
        Vec::from_iter(
            val
            .into_iter()
            .filter(|x| x.chars().nth(bit_pos).unwrap() == '0')
            .map(|x| x.clone()))
    }
}
