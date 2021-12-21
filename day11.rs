#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::str::FromStr;
use std::convert::TryInto;

mod day11_input;

const STEPS_PART1: usize = 100;

#[derive(Debug)]
struct Matrix<T>(Vec<Vec<T>>);
impl<T> Matrix<T> {
    fn size(&self) -> (usize, usize) {
        (self.0.len(), self.0.len())
    }

    fn map<F>(&self, f: F) -> Matrix<T>
    where F: Fn(&T) -> T {
        let mut new_data: Vec<Vec<T>> = Vec::with_capacity(self.0.len());

        for j in 0..self.0.len() {
            let mut row: Vec<T> = Vec::with_capacity(self.0[j].len());
            for i in 0..self.0[j].len() {
                row.push(f(&self.0[j][i]));
            }
            new_data.push(row);
        }

        Matrix (new_data)
    }

    fn mapi<F>(&self, f: F) -> Matrix<T>
    where F: Fn(usize,usize, &T) -> T {
        let mut new_data: Vec<Vec<T>> = Vec::new();

        for j in 0..self.0.len() {
            let mut row: Vec<T> = Vec::new();
            for i in 0..self.0[j].len() {
                row.push(f(i, j, &self.0[j][i]));
            }
            new_data.push(row);
        }

        Matrix (new_data)
    }

    fn enumerate<F>(&mut self, mut f: F)
    where F: FnMut(usize, usize, &T) {
        for j in 0..self.0.len() {
            for i in 0..self.0[j].len() {
                f(i, j, &self.0[j][i]);
            }
        }
    }

    fn count_by<F>(&self, f: F) -> usize
    where F: Fn(&T) -> bool {
        let mut cnt: usize = 0;

        for j in 0..self.0.len() {
            for i in 0..self.0[j].len() {
                if f(&self.0[j][i]) { cnt += 1; }
            }
        }

        cnt
    }

    fn all<F>(&self, f: F) -> bool
    where F: Fn(&T) -> bool {
        for j in 0..self.0.len() {
            for i in 0..self.0[j].len() {
                if !f(&self.0[j][i]) { return false; }
            }
        }
        true
    }
}


#[derive(Debug)]
struct InputError { }
#[derive(Debug)]
struct Input { octopus: Matrix<i32> }

impl FromStr for Input {
    type Err = InputError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = String::from(s);
        let octopus =
            s.lines()
            .map(|l| 
                l.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
            ).collect();

        Ok(Input { octopus: Matrix(octopus) })
    }
}

fn main() {
    println!("Advent of Code 2021");
    println!("Day 11 - Dumbo Octopus");
    println!();

    println!("Part 1");
    println!("======");
    println!();

    let puzzle_test_out1 = do_part1(day11_input::TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", puzzle_test_out1, day11_input::TEST_RESULT_PART1);
    assert_eq!(day11_input::TEST_RESULT_PART1, puzzle_test_out1);

    println!();
    println!("Running puzzle input...");
    let puzzle_out1 = do_part1(day11_input::PUZZLE_INPUT.parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out1);
    println!();

    println!("Part 2");
    println!("======");
    println!();

    let test_out2 = do_part2(day11_input::TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", test_out2, day11_input::TEST_RESULT_PART2);
    assert_eq!(day11_input::TEST_RESULT_PART2, test_out2);

    println!();
    println!("Running puzzle input...");
    let puzzle_out2 = do_part2(day11_input::PUZZLE_INPUT.parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out2);
    println!();
}

fn do_part1(input: Input) -> i32 {
    let mut octopus = input.octopus;
    let (rows, cols) = octopus.size();
    let mut flashes: i32 = 0;

    for _ in 0..STEPS_PART1 {
        // 1. Increment the octopus energy level
        octopus = octopus.map(|o| o + 1);
        // 2. Flash and keep flashing
        while octopus.count_by(|o| *o > 9) > 0 {
            for j in 0..rows {
                for i in 0..cols {
                    let o = octopus.0[j][i];
                    if o > 9 {
                        flashes += 1;

                        // Transfer energy to adjacent octopus
                        if i > 0 {
                            let energy = octopus.0[j][i-1];
                            if energy >= 0 { octopus.0[j][i-1] += 1 }
                            if j > 0 {
                                let energy = octopus.0[j-1][i-1];
                                if energy >= 0 { octopus.0[j-1][i-1] += 1 }
                            }
                            if j < rows-1 {
                                let energy = octopus.0[j+1][i-1];
                                if energy >= 0 { octopus.0[j+1][i-1] += 1 }
                            }
                        }
                        if i < rows-1 {
                            let energy = octopus.0[j][i+1];
                            if energy >= 0 { octopus.0[j][i+1] += 1 }
                            if j > 0 {
                                let energy = octopus.0[j-1][i+1];
                                if energy >= 0 { octopus.0[j-1][i+1] += 1 }
                            }
                            if j < rows-1 {
                                let energy = octopus.0[j+1][i+1];
                                if energy >= 0 { octopus.0[j+1][i+1] += 1 }
                            }
                        }
                        {
                            if j > 0 {
                                let energy = octopus.0[j-1][i];
                                if energy >= 0 { octopus.0[j-1][i] += 1 }
                            }
                            if j < rows-1 {
                                let energy = octopus.0[j+1][i];
                                if energy >= 0 { octopus.0[j+1][i] += 1 }
                            }
                        }

                        // Clear this energy
                        octopus.0[j][i] = -1;
                    }
                }
            }
        }
        // Reset flashed octopus to 0
        octopus = octopus.map(|o| if *o < 0 { 0 } else { *o });
    }
    flashes
}

fn do_part2(input: Input) -> i32 {
    let mut octopus = input.octopus;
    let (rows, cols) = octopus.size();
    let mut loop_cnt = 0;

    loop {
        loop_cnt += 1;

        // 1. Increment the octopus energy level
        octopus = octopus.map(|o| o + 1);
        // 2. Flash and keep flashing
        while octopus.count_by(|o| *o > 9) > 0 {
            for j in 0..rows {
                for i in 0..cols {
                    let o = octopus.0[j][i];
                    if o > 9 {
                        // Transfer energy to adjacent octopus
                        if i > 0 {
                            let energy = octopus.0[j][i-1];
                            if energy >= 0 { octopus.0[j][i-1] += 1 }
                            if j > 0 {
                                let energy = octopus.0[j-1][i-1];
                                if energy >= 0 { octopus.0[j-1][i-1] += 1 }
                            }
                            if j < rows-1 {
                                let energy = octopus.0[j+1][i-1];
                                if energy >= 0 { octopus.0[j+1][i-1] += 1 }
                            }
                        }
                        if i < rows-1 {
                            let energy = octopus.0[j][i+1];
                            if energy >= 0 { octopus.0[j][i+1] += 1 }
                            if j > 0 {
                                let energy = octopus.0[j-1][i+1];
                                if energy >= 0 { octopus.0[j-1][i+1] += 1 }
                            }
                            if j < rows-1 {
                                let energy = octopus.0[j+1][i+1];
                                if energy >= 0 { octopus.0[j+1][i+1] += 1 }
                            }
                        }
                        {
                            if j > 0 {
                                let energy = octopus.0[j-1][i];
                                if energy >= 0 { octopus.0[j-1][i] += 1 }
                            }
                            if j < rows-1 {
                                let energy = octopus.0[j+1][i];
                                if energy >= 0 { octopus.0[j+1][i] += 1 }
                            }
                        }

                        // Clear this energy
                        octopus.0[j][i] = -1;
                    }
                }
            }
        }

        // Break out of the loop if they all flashed.
        if octopus.all(|o| *o < 0) { break; }

        // Reset flashed octopus to 0
        octopus = octopus.map(|o| if *o < 0 { 0 } else { *o });
    }
    loop_cnt
}
