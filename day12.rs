#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::str::FromStr;
use std::convert::TryInto;
use std::collections::HashMap;
use std::cell::RefCell;

use crate::dbgprint;
use crate::day12_input;

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

fn is_uppercase(s: &String) -> bool {
    s.chars().all(|c| c.is_uppercase())
}
fn is_lowercase(s: &String) -> bool {
    s.chars().all(|c| c.is_lowercase())
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

pub fn main() {
    //dbgprint::enable();

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

fn input_to_caves_nodes(input: &Input) -> (Vec<String>, HashMap<String, Vec<usize>>) {
    let mut caves: Vec<String> = Vec::new();
    let mut nodes: HashMap<String, Vec<usize>> = HashMap::new();

    // Read the paths and store all connected caves
    for path in input.paths.iter() {
        if !caves.contains(&path.a.name) {
            caves.push(path.a.name.clone());
            nodes.entry(path.a.name.clone()).or_insert(Vec::new());
        }
        if !caves.contains(&path.b.name) {
            caves.push(path.b.name.clone());
            nodes.entry(path.b.name.clone()).or_insert(Vec::new());
        }

        let a_idx = caves.iter().position(|c| *c == path.a.name).unwrap();
        let b_idx = caves.iter().position(|c| *c == path.b.name).unwrap();

        nodes.get_mut(&caves[a_idx]).unwrap().push(b_idx);
        nodes.get_mut(&caves[b_idx]).unwrap().push(a_idx);
    }

    // Sort then remove duplicates for connected caves
    for c in caves.iter() {
        let nodes = nodes.get_mut(c).unwrap();
        nodes.sort();
        nodes.dedup();
    }

    (caves.clone(), nodes.clone())
}

fn do_part1(input: Input) -> i32 {
    let (caves, nodes) = input_to_caves_nodes(&input);

    let end_idx = caves.iter().position(|c| *c == "end").unwrap();
    let start_idx = caves.iter().position(|c| *c == "start").unwrap();
    let mut paths: Vec<Vec<usize>> = Vec::new();
    for i in nodes.get("start").unwrap().iter() {
        paths.push(vec![start_idx, *i]);
    }

    // Keep running through the paths until they are all ended or reached a dead-end
    loop {
        let mut dead_ends: Vec<usize> = Vec::new();
        let remaining_paths: Vec<usize> = paths.iter().enumerate().filter(|(i,p)| p.last().unwrap() != &end_idx).map(|(i,p)| i).collect();

        for i in remaining_paths {
            let n = *paths[i].last().unwrap();
            let next_caves: Vec<&usize> = nodes.get(&caves[n]).unwrap().iter().filter(|p| is_uppercase(&caves[**p]) || !paths[i].contains(p)).collect();
            if next_caves.len() == 0 {
                dead_ends.push(i);
            } else {
                let path = paths[i].clone();
                paths[i].push(*next_caves[0]);
                
                if next_caves.len() > 1 {
                    for next in next_caves.iter().skip(1) {
                        let mut path = path.clone();
                        path.push(**next);
                        paths.push(path);
                    }
                }
            }
        }

        // Remove the dead-end paths
        dead_ends.sort();
        for i in dead_ends.iter().rev() { paths.remove(*i); }

        // End condition
        if paths.iter().all(|p| p.last().unwrap() == &end_idx) { break; }
    }

    paths.len() as i32
}

fn find_paths_part2(caves: &Vec<String>, nodes: &HashMap<String, Vec<usize>>, allow_small_cave_idx: usize) -> Vec<Vec<usize>> {
    let end_idx = caves.iter().position(|c| *c == "end").unwrap();
    let start_idx = caves.iter().position(|c| *c == "start").unwrap();
    let mut paths: Vec<Vec<usize>> = Vec::new();
    for i in nodes.get("start").unwrap().iter() {
        paths.push(vec![start_idx, *i]);
    }

    // Keep running through the paths until they are all ended or reached a dead-end
    loop {
        let mut dead_ends: Vec<usize> = Vec::new();
        let remaining_paths: Vec<usize> = paths.iter().enumerate().filter(|(i,p)| p.last().unwrap() != &end_idx).map(|(i,p)| i).collect();

        for i in remaining_paths {
            let n = *paths[i].last().unwrap();
            let next_caves: Vec<&usize> = 
                nodes.get(&caves[n]).unwrap().iter()
                    .filter(|p| 
                        is_uppercase(&caves[**p])
                        || !paths[i].contains(p)
                        || (**p == allow_small_cave_idx && paths[i].iter().filter(|q| q == p).count() < 2)
                    ).collect();
            if next_caves.len() == 0 {
                dead_ends.push(i);
            } else {
                let path = paths[i].clone();
                paths[i].push(*next_caves[0]);
                
                if next_caves.len() > 1 {
                    for next in next_caves.iter().skip(1) {
                        let mut path = path.clone();
                        path.push(**next);
                        paths.push(path);
                    }
                }
            }
        }

        // Remove the dead-end paths
        dead_ends.sort();
        for i in dead_ends.iter().rev() { paths.remove(*i); }

        // End condition
        if paths.iter().all(|p| p.last().unwrap() == &end_idx) { break; }
    }

    paths.clone()
}

fn do_part2(input: Input) -> i32 {
    let (caves, nodes) = input_to_caves_nodes(&input);
    let mut paths: Vec<Vec<usize>> = Vec::new();
    
    for (i, small_cave) in caves.iter().enumerate().filter(|(i,c)| is_lowercase(c) && c != &"start" && c != &"end") {
        paths.append(&mut find_paths_part2(&caves, &nodes, i));
    }
    paths.sort();
    paths.dedup();

    paths.iter().map(|p| p.iter().map(|c| caves[*c].clone()).collect::<Vec<String>>().join(",")).for_each(|p| dbgprintln!("{:?}", p));

    paths.len() as i32
}
