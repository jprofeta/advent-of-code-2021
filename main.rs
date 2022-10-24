#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

#[macro_use]
mod dbgprint;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;

mod day01_input;
mod day02_input;
mod day03_input;
mod day04_input;
mod day05_input;
mod day06_input;
mod day07_input;
mod day08_input;
mod day09_input;
mod day10_input;
mod day11_input;
mod day12_input;
mod day13_input;
mod day14_input;

use std::env;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<i32>().unwrap();
    match day {
        1 => day01::main(),
        2 => day02::main(),
        3 => day03::main(),
        4 => day04::main(),
        5 => day05::main(),
        6 => day06::main(),
        7 => day07::main(),
        8 => day08::main(),
        9 => day09::main(),
        10 => day10::main(),
        11 => day11::main(),
        12 => day12::main(),
        13 => day13::main(),
        14 => day14::main(),
        _ => println!("Undefined day")
    }
}
