#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::str::FromStr;
use std::convert::TryInto;

use ndarray::Array;
use ndarray::Array2;
use ndarray::arr2;

mod day13_input;
mod dbgprint;

#[derive(Debug, Clone, Eq, PartialEq)]
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
    //dbgprint::enable();

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

fn make_paper(input: &Input) -> Array2<i32> {
    let width = input.dots.iter().map(|(x,y)| *x).max().unwrap();
    let height = input.dots.iter().map(|(x,y)| *y).max().unwrap();
    let mut paper: Array2<i32> = Array::zeros((width + 1, height + 1));

    for d in input.dots.iter() {
        *(paper.get_mut((d.0, d.1)).unwrap()) = 1;
    }

    paper.clone()
}

fn fold_paper(arr: &Array2<i32>, fold: (FoldDirection, usize)) -> Array2<i32> {
    let shape = arr.shape();
    let (w, h) = match fold.0 {
        FoldDirection::Horizontal => (shape[0], fold.1),
        FoldDirection::Vertical => (fold.1, shape[1])
    };

    let mut folded: Array2<i32> = Array::zeros((w, h));
    for j in 0..h {
        for i in 0..w {
            let idx = (i,j);
            let idx_fold = match fold.0 {
                FoldDirection::Horizontal => (i, 2 * fold.1 - j),
                FoldDirection::Vertical => (2 * fold.1 - i, j)
            };
            *folded.get_mut(idx).unwrap() = *arr.get(idx).unwrap() | *arr.get(idx_fold).unwrap_or(&0);
        }
    }

    folded
}

fn print_paper_fold(arr: &Array2<i32>, fold: &(FoldDirection, usize)) {
    let shape = arr.shape();
    for j in 0..shape[1] {
        for i in 0..shape[0] {
            let is_dot = arr.get((i,j)).unwrap() > &0;

            if i == fold.1 && fold.0 == FoldDirection::Vertical {
                if is_dot { dbgprint!("X"); } else { dbgprint!("|"); }
            } else if j == fold.1 && fold.0 == FoldDirection::Horizontal {
                if is_dot { dbgprint!("X"); } else { dbgprint!("_"); }
            } else {
                if is_dot { dbgprint!("#"); }
                else { dbgprint!("."); }
            }
        }
        dbgprintln!();
    }
}

fn print_paper(arr: &Array2<i32>) {
    let shape = arr.shape();
    for j in 0..shape[1] {
        for i in 0..shape[0] {
            if arr.get((i,j)).unwrap() > &0 { print!("#"); }
            else { print!("."); }
        }
        println!();
    }
}

fn do_part1(input: Input) -> i32 {
    let paper = make_paper(&input);
    let folded = fold_paper(&paper, input.folds[0].clone());
    folded.iter().filter(|x| **x > 0).count() as i32
}

fn do_part2(input: Input) -> i32 {
    let mut paper = make_paper(&input);
    for fold in input.folds.iter() {
        dbgprintln!("{:?}", fold);
        if paper.shape().iter().max().unwrap() > &0 {
            print_paper_fold(&paper, &fold);
            dbgprintln!();
        }

        paper = fold_paper(&paper, fold.clone());
    }
    print_paper(&paper);
    0
}
