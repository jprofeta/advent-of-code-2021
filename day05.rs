#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::str::FromStr;
use std::convert::TryInto;
use std::collections::HashMap;
use std::cmp;

use crate::day05_input;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq, Eq, Hash)]
struct Point { x: i32, y: i32 }
impl FromStr for Point {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<i32> = s.split(',').map(|x| x.parse::<i32>().unwrap()).collect();
        Ok(Point { x: coords[0], y: coords[1] })
    }
}
#[derive(Debug)]
struct Line { a: Point, b: Point }
impl Line {
    fn is_horizontal(&self) -> bool { self.a.y == self.b.y }
    fn is_vertical(&self) -> bool { self.a.x == self.b.x }
}
impl FromStr for Line {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<Point> = s.split("->").map(|x| x.trim().parse::<Point>().unwrap()).collect();
        Ok(Line { a: points[0].clone(), b: points[1].clone() })
    }
}

#[derive(Debug)]
struct InputError { }
#[derive(Debug)]
struct Input { lines: Vec<Line> }

impl FromStr for Input {
    type Err = InputError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = String::from(s);
        let lines: Vec<Line> = s.lines().map(|x| x.trim().parse::<Line>().unwrap()).collect();

        Ok(Input { lines: lines })
    }
}

pub fn main() {
    println!("Advent of Code 2021");
    println!("Day 05 - Hydrothermal Venture");
    println!();

    println!("Part 1");
    println!("======");
    println!();

    let puzzle_test_out1 = do_part1(day05_input::TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", puzzle_test_out1, day05_input::TEST_RESULT_PART1);
    assert_eq!(day05_input::TEST_RESULT_PART1, puzzle_test_out1);

    println!();
    println!("Running puzzle input...");
    let puzzle_out1 = do_part1(day05_input::PUZZLE_INPUT.parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out1);
    println!();

    println!("Part 2");
    println!("======");
    println!();

    let test_out2 = do_part2(day05_input::TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", test_out2, day05_input::TEST_RESULT_PART2);
    assert_eq!(day05_input::TEST_RESULT_PART2, test_out2);

    println!();
    println!("Running puzzle input...");
    let puzzle_out2 = do_part2(day05_input::PUZZLE_INPUT.parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out2);
    println!();
}

fn do_part1(input: Input) -> i32 {
    let mut vents: HashMap<Point,i32> = HashMap::new();
    for l in input.lines {
        if l.is_horizontal() {
            let min = cmp::min(l.a.x, l.b.x);
            let max = cmp::max(l.a.x, l.b.x);
            for x in min..=max {
                let p = Point{x: x, y: l.a.y};
                let v = vents.entry(p).or_insert(0);
                *v += 1;
            }
        } else if l.is_vertical() {
            let min = cmp::min(l.a.y, l.b.y);
            let max = cmp::max(l.a.y, l.b.y);
            for y in min..=max {
                let p = Point{x: l.a.x, y: y};
                let v = vents.entry(p).or_insert(0);
                *v += 1;
            }
        }
    }
    vents.iter().filter(|(p,c)| **c > 1).count() as i32
}

fn do_part2(input: Input) -> i32 {
    let mut vents: HashMap<Point,i32> = HashMap::new();
    for l in input.lines {
        let dy = l.b.y - l.a.y;
        let dx = l.b.x - l.a.x;
        let r = cmp::max(dy.abs(),dx.abs());
        let dy = dy/r;
        let dx = dx/r;

        let mut p = l.a.clone();
        let v = vents.entry(p.clone()).or_insert(0);
        *v += 1;

        loop {
            p = Point{ x: p.x + dx, y: p.y + dy };
            let v = vents.entry(p.clone()).or_insert(0);
            *v += 1;

            if p == l.b { break; }
        }
    }
    vents.iter().filter(|(p,c)| **c > 1).count() as i32
}
