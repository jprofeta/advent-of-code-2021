#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::str::FromStr;
use std::convert::TryInto;
use std::collections::HashMap;

mod day10_input;

const DEBUG_PRINT_ENABLE: bool = false;

macro_rules! dbgprintln {
    () => (if DEBUG_PRINT_ENABLE { println!() });
    ($($arg:tt)*) => (if DEBUG_PRINT_ENABLE { println!($($arg)*) });
}

const SCORES_PART1: [(char,i32); 4] = [
    (')', 3),
    (']', 57),
    ('}', 1197),
    ('>', 25137),
];

const SCORES_PART2: [(char,i32); 4] = [
    (')', 1),
    (']', 2),
    ('}', 3),
    ('>', 4),
];

const OPENS: [char; 4] = [ '(', '[', '{', '<' ];
const CLOSURES: [char; 4] = [ ')', ']', '}', '>' ];

#[derive(Debug)]
struct InputError { }
#[derive(Debug)]
struct Input { subsystems: Vec<String> }

impl FromStr for Input {
    type Err = InputError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<String> = String::from(s).lines().map(|x| String::from(x)).collect();
        Ok(Input { subsystems: lines })
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
    assert_eq!(day10_input::TEST_RESULT_PART2 as i64, test_out2);

    println!();
    println!("Running puzzle input...");
    let puzzle_out2 = do_part2(day10_input::PUZZLE_INPUT.parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out2);
    println!();
}

fn do_part1(input: Input) -> i32 {
    let mut syntax_violations: Vec<char> = Vec::new();
    struct WorkingChunk { expects: char }

    for subsystem in input.subsystems {
        let mut chunks: Vec<WorkingChunk> = Vec::new();
        let cs: Vec<char> = subsystem.chars().collect();

        for i in 0..cs.len() {
            if OPENS.contains(&cs[i]) {
                let expected = match cs[i] {
                    '{' => Some('}'),
                    '[' => Some(']'),
                    '(' => Some(')'),
                    '<' => Some('>'),
                    _ => None
                }.unwrap();
                chunks.push(WorkingChunk{ expects: expected });
            } else if CLOSURES.contains(&cs[i]) {
                let chunk = chunks.pop().unwrap();
                if cs[i] != chunk.expects {
                    syntax_violations.push(cs[i]);
                    break;
                }
            }
        }
    }

    dbgprintln!("{}", syntax_violations.len());
    let scores = HashMap::from(SCORES_PART1);
    syntax_violations.iter().map(|v| scores.get(v).unwrap()).sum::<i32>()
}

fn do_part2(input: Input) -> i64 {
    struct WorkingChunk { expects: char }
    let scores = HashMap::from(SCORES_PART2);

    let mut subsystem_totals: Vec<i64> = Vec::new();

    for subsystem in input.subsystems {
        let mut chunks: Vec<WorkingChunk> = Vec::new();
        let cs: Vec<char> = subsystem.chars().collect();

        for i in 0..cs.len() {
            if OPENS.contains(&cs[i]) {
                let expected = match cs[i] {
                    '{' => Some('}'),
                    '[' => Some(']'),
                    '(' => Some(')'),
                    '<' => Some('>'),
                    _ => None
                }.unwrap();
                chunks.push(WorkingChunk{ expects: expected });
            } else if CLOSURES.contains(&cs[i]) {
                let chunk = chunks.pop().unwrap();
                if cs[i] != chunk.expects {
                    // Corrupted subsystem
                    dbgprintln!("Expected {} found {}. {}", &chunk.expects, &cs[i], &subsystem);
                    chunks.clear();
                    break;
                }
            }
        }

        if chunks.len() == 0 { continue; }

        dbgprintln!("Incomplete by {:?}", &chunks.iter().map(|c| c.expects).collect::<Vec<char>>());
        let total = chunks.iter().rev().map(|c| scores.get(&c.expects).unwrap()).fold(0, |acc,x| acc * 5 + (*x as i64));
        subsystem_totals.push(total);
    }

    let n = subsystem_totals.len();
    dbgprintln!("{} {}", &n, &(n/2));
    subsystem_totals.sort();
    subsystem_totals[n/2]
}

fn is_subsystem_incomplete(line: &String) -> bool {
    let cs: Vec<char> = line.chars().collect();

    let chunk_opens = cs.iter().filter(|c| OPENS.contains(&c)).count();
    let chunk_closures = cs.iter().filter(|c| CLOSURES.contains(&c)).count();
    println!("{}\t{}/{}", line, &chunk_opens, &chunk_closures);
    chunk_opens != chunk_closures
}
