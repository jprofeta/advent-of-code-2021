#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::str::FromStr;
use std::convert::TryInto;

use crate::day04_input;

#[derive(Debug)]
struct BingoBoard([[i32; 5]; 5]);

#[derive(Debug)]
struct InputError { }
#[derive(Debug)]
struct Input { 
    numbers: Vec<i32>,
    boards: Vec<BingoBoard>
}

impl FromStr for Input {
    type Err = InputError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = String::from(s);
        let mut s = s.lines();
        
        // Read the first line as a comma-separated list of numbers
        let l = s.next().unwrap();
        let nums = Vec::from_iter(l.split(',').map(|x| x.parse::<i32>().unwrap()));
        
        // A blank line follows
        s.next();

        // A list of boards are next until the end
        let mut boards: Vec<BingoBoard> = Vec::new();
        let mut b = [[0; 5]; 5];
        let mut row = 0;
        loop {
            let l = s.next();
            match l {
                None => { boards.push(BingoBoard(b.clone())); break; },
                Some(x) => {},
            }
            let l = l.unwrap();
            if l.len() == 0 {
                boards.push(BingoBoard(b.clone()));
                row = 0;
            } else {
                b[row] = 
                    l
                    .split(' ')
                    .filter(|x| x.len() > 0)
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
                    .try_into().unwrap();
                row += 1;
            }
        }

        Ok(Input { numbers: nums, boards: boards })
    }
}

pub fn main() {
    println!("Advent of Code 2021");
    println!("Day 04 - Giant Squid");
    println!();

    println!("Part 1");
    println!("======");
    println!();

    let puzzle_test_out1 = do_part1(day04_input::TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", puzzle_test_out1, day04_input::TEST_RESULT_PART1);
    assert_eq!(day04_input::TEST_RESULT_PART1, puzzle_test_out1);

    println!();
    println!("Running puzzle input...");
    let puzzle_out1 = do_part1(day04_input::PUZZLE_INPUT.parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out1);
    println!();

    println!("Part 2");
    println!("======");
    println!();

    let test_out2 = do_part2(day04_input::TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", test_out2, day04_input::TEST_RESULT_PART2);
    assert_eq!(day04_input::TEST_RESULT_PART2, test_out2);

    println!();
    println!("Running puzzle input...");
    let puzzle_out2 = do_part2(day04_input::PUZZLE_INPUT.parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out2);
    println!();
}

fn do_part1(input: Input) -> i32 {
    let mut marks = vec![[[false; 5]; 5]; input.boards.len()];
    for n in input.numbers {
        for (i, b) in input.boards.iter().enumerate() {
            mark_board(&b.0, &mut marks[i], n);
            if has_bingo(&marks[i]) {
                return calc_board_result(n, &b, &marks[i]);
            }
        }
    }
    -1
}

fn do_part2(input: Input) -> i32 {
    let mut marks = vec![[[false; 5]; 5]; input.boards.len()];
    let mut board_has_bingo = vec![false; input.boards.len()];
    for n in input.numbers {
        for (i, b) in input.boards.iter().enumerate() {
            // Only continue to process boards that have not won
            if !board_has_bingo[i] {
                mark_board(&b.0, &mut marks[i], n);
                if has_bingo(&marks[i]) {
                    let boards_remaining = board_has_bingo.iter().filter(|x| !**x).count();
                    if boards_remaining == 1 {
                        return calc_board_result(n, &b, &marks[i]);
                    } else {
                        board_has_bingo[i] = true;
                    }
                }
            }
        }
    }
    -1
}

fn calc_board_result(num: i32, board: &BingoBoard, mark: &[[bool; 5]; 5]) -> i32 {
    let board = &board.0;
    let mut sum = 0;
    for i in 0..5 {
        for j in 0..5 {
            if !mark[i][j] {
                sum += board[i][j];
            }
        }
    }
    sum*num
}

fn mark_board(board: &[[i32; 5]; 5], marks: &mut [[bool; 5]; 5], number: i32) {
    for i in 0..5 {
        for j in 0..5 {
            if board[i][j] == number {
                marks[i][j] = true;
            }
        }
    }
}

fn has_bingo(marks: &[[bool; 5]; 5]) -> bool {
    marks.iter().any(|x| x.iter().all(|y| *y))
        || transpose(&marks).iter().any(|x| x.iter().all(|y| *y))
}

fn transpose(a: &[[bool; 5]; 5]) -> [[bool; 5]; 5] {
    let mut b = [[false; 5]; 5];
    for i in 0..5 {
        for j in 0..5 {
            b[j][i] = a[i][j];
        }
    }
    b.clone()
}
