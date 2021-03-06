#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::str::FromStr;
use std::convert::TryInto;
use std::cmp;

mod day09_input;

#[derive(Debug)]
struct InputError { }
#[derive(Debug)]
struct Input { heatmap: Vec<Vec<i32>> }

impl FromStr for Input {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut heatmap: Vec<Vec<i32>> = Vec::new();
        let s = String::from(s);
        for l in s.lines() {
            let slice: Vec<i32> = l.trim().chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
            heatmap.push(slice);
        }

        Ok(Input { heatmap: heatmap })
    }
}

fn main() {
    println!("Advent of Code 2021");
    println!("Day 09 - Smoke Basin");
    println!();

    println!("Part 1");
    println!("======");
    println!();

    let puzzle_test_out1 = do_part1(day09_input::TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", puzzle_test_out1, day09_input::TEST_RESULT_PART1);
    assert_eq!(day09_input::TEST_RESULT_PART1, puzzle_test_out1);

    println!();
    println!("Running puzzle input...");
    let puzzle_out1 = do_part1(day09_input::PUZZLE_INPUT.parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out1);
    println!();

    println!("Part 2");
    println!("======");
    println!();

    let test_out2 = do_part2(day09_input::TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", test_out2, day09_input::TEST_RESULT_PART2);
    assert_eq!(day09_input::TEST_RESULT_PART2, test_out2);

    println!();
    println!("Running puzzle input...");
    let puzzle_out2 = do_part2(day09_input::PUZZLE_INPUT.parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out2);
    println!();
}

fn do_part1(input: Input) -> i32 {
    let map_lows = get_local_mins(&input.heatmap);
    map_lows.iter().fold(0, |acc,x| acc + x.h + 1)
}

fn do_part2(input: Input) -> i32 {
    let map = &input.heatmap;
    let map_lows = get_local_mins(map);
    let mut basin_sizes: Vec<i32> = Vec::new();

    for low in map_lows {
        let mut points_in_basin: Vec<(usize, usize)> = vec![(low.x, low.y)];
        let mut k: usize = 0;

        while k < points_in_basin.len() {
            add_basin_adjacent(map, points_in_basin[k], &mut points_in_basin);
            k += 1;
        }

        basin_sizes.push(points_in_basin.len() as i32);
    }

    basin_sizes.sort();
    basin_sizes.iter().rev().take(3).fold(1, |acc,x| acc*x)
}

struct LocalMin { x: usize, y: usize, h: i32 }
fn get_local_mins(map: &Vec<Vec<i32>>) -> Vec<LocalMin> {
    let m = map.len();

    let mut map_lows: Vec<LocalMin> = Vec::new();

    for j in 0..m {
        let row = &map[j];
        let n = row.len();

        for i in 0..n {
            let h = map[j][i];
            if i > 0 {
                let x = map[j][i-1];
                if h >= x { continue; }
            }
            if i < n-1 {
                let x = map[j][i+1];
                if h >= x { continue; }
            }
            if j > 0 {
                let x = map[j-1][i];
                if h >= x { continue; }
            }
            if j < m-1 {
                let x = map[j+1][i];
                if h >= x { continue; }
            }
            
            // If it makes it this far then the value is "low"
            map_lows.push(LocalMin{x: i, y: j, h: h});
        }
    }

    map_lows
}

fn add_basin_adjacent(map: &Vec<Vec<i32>>, coords: (usize, usize), basin_points: &mut Vec<(usize, usize)>) {
    let m = map.len();
    let n = map[0].len();

    let (i,j) = coords;
    
    if i > 0 && map[j][i-1] < 9 && !basin_points.contains(&(i-1,j)) { basin_points.push((i-1, j)); }
    if j > 0 && map[j-1][i] < 9 && !basin_points.contains(&(i,j-1)) { basin_points.push((i, j-1)); }

    if i < n - 1 && map[j][i+1] < 9 && !basin_points.contains(&(i+1,j)) { basin_points.push((i+1, j)); }
    if j < m - 1 && map[j+1][i] < 9 && !basin_points.contains(&(i,j+1)) { basin_points.push((i, j+1)); }
}
