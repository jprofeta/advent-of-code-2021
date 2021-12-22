#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::str::FromStr;
use std::convert::TryInto;
use std::collections::HashMap;

mod dbgprint;
mod day12_input;

pub trait IteratorSingle<T> {
    fn single(&mut self) -> Option<T>;
}
impl<T, I> IteratorSingle<T> for I where I: Iterator<Item=T> {
    fn single(&mut self) -> Option<T> {
        let item = self.next()?;
        if self.next().is_none() { Some(item) }
        else { None }
    }
}

#[derive(Debug, Clone)]
struct Cave { name: String, large: bool }
impl FromStr for Cave {
    type Err = InputError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = String::from(s);
        let large = s.chars().all(|c| c.is_uppercase());

        Ok(Cave{name: s, large: large})
    }
}

#[derive(Debug, Clone)]
struct Path { a: Cave, b: Cave }
impl FromStr for Path {
    type Err = InputError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = String::from(s);
        let caves: Vec<Cave> = s.split('-').map(|x| x.parse::<Cave>().unwrap()).collect();

        Ok(Path{a: caves[0].clone(), b: caves[1].clone()})
    }
}

#[derive(Debug)]
struct InputError { }

#[derive(Debug)]
struct Input { paths: Vec<Path> }
impl FromStr for Input {
    type Err = InputError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = String::from(s);
        let paths = s.lines().map(|l| l.parse::<Path>().unwrap()).collect();

        Ok(Input { paths: paths })
    }
}

fn main() {
    dbgprint::enable();

    println!("Advent of Code 2021");
    println!("Day 12 - Passage Pathing");
    println!();

    println!("Part 1");
    println!("======");
    println!();

    let puzzle_test_out1 = do_part1(day12_input::TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", puzzle_test_out1, day12_input::TEST_RESULT_PART1);
    assert_eq!(day12_input::TEST_RESULT_PART1, puzzle_test_out1);

    println!();
    println!("Running puzzle input...");
    let puzzle_out1 = do_part1(day12_input::PUZZLE_INPUT.parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out1);
    println!();

    println!("Part 2");
    println!("======");
    println!();

    let test_out2 = do_part2(day12_input::TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", test_out2, day12_input::TEST_RESULT_PART2);
    assert_eq!(day12_input::TEST_RESULT_PART2, test_out2);

    println!();
    println!("Running puzzle input...");
    let puzzle_out2 = do_part2(day12_input::PUZZLE_INPUT.parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out2);
    println!();

}

fn do_part1(input: Input) -> i32 {
    struct CaveNode<'a> { paths: Vec<&'a CaveNode<'a>>, cave: &'a Cave }
    let mut nodes: HashMap<String, CaveNode> = HashMap::new();

    for path in input.paths {
        nodes.entry(path.a.name.clone()).or_insert(CaveNode{ paths: Vec::new(), cave: *path.a });
        nodes.entry(path.b.name.clone()).or_insert(CaveNode{ paths: Vec::new(), cave: *path.b });
    }


    0
}

fn do_part2(input: Input) -> i32 {
    0
}
