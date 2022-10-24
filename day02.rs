use std::iter::FromIterator;
use std::str::FromStr;

use crate::day02_input;

enum SubOperation {
    Forward,
    Down,
    Up
}

struct Action { op: SubOperation, value: i32 }
impl FromStr for Action {
    type Err = std::num::ParseIntError;

    fn from_str(action_str: &str) -> Result<Self, Self::Err> {
        let i = action_str.find(' ').unwrap();
        let (op_s,val_s) = action_str.split_at(i);
        let val = val_s.trim().parse::<i32>();
        let op:Option<SubOperation> = match op_s {
            "forward" => Some(SubOperation::Forward),
            "down" => Some(SubOperation::Down),
            "up" => Some(SubOperation::Up),
            _ => None,
        };
        Ok(Action { op:op.unwrap(), value: val.unwrap() })
    }
}

pub fn main() {
    println!("Advent of Code");
    println!("Day 02 - Dive");
    println!();

    println!("Part 1");
    println!("======");
    println!();

    let puzzle_test_out1 = do_part1(&Vec::from_iter(day02_input::TEST_INPUT.iter().map(|x| Action::from_str(x).unwrap())));
    println!("Test output: {} (expected {})", puzzle_test_out1, day02_input::TEST_RESULT_PART1);
    assert_eq!(day02_input::TEST_RESULT_PART1, puzzle_test_out1);

    println!();
    println!("Running puzzle input...");
    let puzzle_out1 = do_part1(&Vec::from_iter(day02_input::PUZZLE_INPUT.iter().map(|x| Action::from_str(x).unwrap())));
    println!("Puzzle result: {}", puzzle_out1);
    println!();

    println!("Part 2");
    println!("======");
    println!();

    let test_out2 = do_part2(&Vec::from_iter(day02_input::TEST_INPUT.iter().map(|x| Action::from_str(x).unwrap())));
    println!("Test output: {} (expected {})", test_out2, day02_input::TEST_RESULT_PART2);
    assert_eq!(day02_input::TEST_RESULT_PART2, test_out2);

    println!();
    println!("Running puzzle input...");
    let puzzle_out2 = do_part2(&Vec::from_iter(day02_input::PUZZLE_INPUT.iter().map(|x| Action::from_str(x).unwrap())));
    println!("Puzzle result: {}", puzzle_out2);
    println!();
}

fn do_part1(actions: &Vec<Action>) -> i32 {
    let position = 
        actions.iter().fold(
            (0,0),
            |acc, a| {
                let (hor, ver) = acc;
                match a.op {
                    SubOperation::Forward => (hor + a.value, ver),
                    SubOperation::Down => (hor, ver + a.value),
                    SubOperation::Up => (hor, ver - a.value)
                }
            }
        );
    position.0 * position.1
}

fn do_part2(actions: &Vec<Action>) -> i32 {
    let position = 
        actions.iter().fold(
            (0,0,0),
            |acc, a| {
                let (hor, ver, aim) = acc;
                match a.op {
                    SubOperation::Forward => (hor + a.value, ver + aim*a.value, aim),
                    SubOperation::Down => (hor, ver, aim + a.value),
                    SubOperation::Up => (hor, ver, aim - a.value)
                }
            }
        );
    position.0 * position.1
}
