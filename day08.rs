#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::iter::Iterator;
use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::str::FromStr;
use std::convert::TryInto;
use std::collections::HashMap;

mod day08_input;

const DEBUG_PRINT_ENABLE: bool = false;

macro_rules! dbgprintln {
    () => (if DEBUG_PRINT_ENABLE { println!() });
    ($($arg:tt)*) => (if DEBUG_PRINT_ENABLE { println!($($arg)*) });
}

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

#[derive(Debug)]
struct Display { signals: Vec<String>, values: Vec<String> }

#[derive(Debug)]
struct InputError { }
#[derive(Debug)]
struct Input { displays: Vec<Display> }

impl FromStr for Input {
    type Err = InputError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = String::from(s);
        let lines = s.lines();

        let mut displays: Vec<Display> = Vec::new();
        for l in lines {
            let parts: Vec<&str> = l.split('|').collect();
            let signals: Vec<&str> = parts[0].trim().split(' ').collect();
            let values: Vec<&str> = parts[1].trim().split(' ').collect();

            displays.push(
                Display { 
                    signals: signals.iter().map(|x| String::from(*x)).collect(), 
                    values: values.iter().map(|x| String::from(*x)).collect() 
                }
            );
        }
        Ok(Input{displays: displays})
    }
}

fn main() {
    println!("Advent of Code 2021");
    println!("Day 08 - Seven Segment Search");
    println!();

    println!("Part 1");
    println!("======");
    println!();

    let puzzle_test_out1 = do_part1(day08_input::TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", puzzle_test_out1, day08_input::TEST_RESULT_PART1);
    assert_eq!(day08_input::TEST_RESULT_PART1, puzzle_test_out1);

    println!();
    println!("Running puzzle input...");
    let puzzle_out1 = do_part1(day08_input::PUZZLE_INPUT.parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out1);
    println!();

    println!("Part 2");
    println!("======");
    println!();

    let test_out2 = do_part2(day08_input::TEST_INPUT.parse::<Input>().unwrap());
    println!("Test output: {} (expected {})", test_out2, day08_input::TEST_RESULT_PART2);
    assert_eq!(day08_input::TEST_RESULT_PART2, test_out2);

    println!();
    println!("Running puzzle input...");
    let puzzle_out2 = do_part2(day08_input::PUZZLE_INPUT.parse::<Input>().unwrap());
    println!("Puzzle result: {}", puzzle_out2);
    println!();
}

fn do_part1(input: Input) -> i32 {
    input.displays.iter()
        .map(|x|
            x.values.iter()
            .filter(|y| y.len() < 5 || y.len() > 6)
            .count() as i32
        ).sum::<i32>()
}

fn do_part2(input: Input) -> i32 {
    let mut sum = 0;
    for d in input.displays {
        let decoded = decode_segments(&d.signals);
        dbgprintln!("{:?}", &decoded);
        let value =
            d.values.iter()
            .map(|v| sort_string(v))
            .map(|v| { dbgprintln!("{}", &v); decoded.get(&v).unwrap() })
            .fold(0, |acc,x| acc*10 + x);
        dbgprintln!("{:?}", value);
        sum += value;
    }
    sum
}

fn decode_segments(unique_signals: &Vec<String>) -> HashMap<String,i32> {
    // Use the unique signals to determine which segments overlap.
    // 1. segA = segs7 - segs1
    // 2. segD = segs3 int. segs4 - segs1; segs3 == segs(#5) union segs7
    // 3. segB = segs4 - segs1 - segD
    // 4. segG = segs3 - segs7 - segD
    // 5. segE = segs2 - segs3; segs2 = segs(#5) not segB
    // 6. segF = segs5 - segs2 - segB; segs5 = segs(#5) and segB
    // 7. segC = segs1 - segF

    let segs1 = unique_signals.iter().filter(|x| x.len() == 2).single().unwrap();   dbgprintln!("1: {}", &segs1);
    let segs4 = unique_signals.iter().filter(|x| x.len() == 4).single().unwrap();   dbgprintln!("4: {}", &segs4);
    let segs7 = unique_signals.iter().filter(|x| x.len() == 3).single().unwrap();   dbgprintln!("7: {}", &segs7);
    let segs8 = unique_signals.iter().filter(|x| x.len() == 7).single().unwrap();   dbgprintln!("8: {}", &segs8);

    let segs_with5: Vec<&String> = unique_signals.iter().filter(|x| x.len() == 5).collect();

    let segs3 = segs_with5.iter().filter(|x| segs7.chars().all(|c| x.contains(c))).single().unwrap();   dbgprintln!("3: {}", &segs3);

    let _a = segs7.chars().filter(|c| !segs1.contains(*c)).single().unwrap();    // segs7 - segs1
    let _d = segs3.chars().filter(|c| segs4.contains(*c)).filter(|c| !segs1.contains(*c)).single().unwrap();
    let _b = segs4.chars().filter(|c| !segs1.contains(*c)).filter(|c| *c != _d).single().unwrap();
    let _g = segs3.chars().filter(|c| !segs7.contains(*c)).filter(|c| *c != _d).single().unwrap();

    let segs2 = segs_with5.iter().filter(|x| *x != segs3 && !x.contains(_b)).single().unwrap();   dbgprintln!("2: {}", &segs2);
    let segs5 = segs_with5.iter().filter(|x| *x != segs3 && x.contains(_b)).single().unwrap();    dbgprintln!("5: {}", &segs5);

    let _e = segs2.chars().filter(|c| !segs3.contains(*c)).single().unwrap();
    let _f = segs5.chars().filter(|c| !segs2.contains(*c)).filter(|c| *c != _b).single().unwrap();
    let _c = segs1.chars().filter(|c| *c != _f).single().unwrap();

    dbgprintln!(" {} ", &_a);
    dbgprintln!("{} {}", &_b, &_c);
    dbgprintln!(" {} ", &_d);
    dbgprintln!("{} {}", &_e, &_f);
    dbgprintln!(" {} ", &_g);

    HashMap::from([
        (segs_to_string(vec![_a, _b, _c, _e, _f, _g]), 0),
        (segs_to_string(vec![_c, _f]), 1),
        (segs_to_string(vec![_a, _c, _d, _e, _g]), 2),
        (segs_to_string(vec![_a, _c, _d, _f, _g]), 3),
        (segs_to_string(vec![_b, _c, _d, _f]), 4),
        (segs_to_string(vec![_a, _b, _d, _f, _g]), 5),
        (segs_to_string(vec![_a, _b, _d, _e, _f, _g]), 6),
        (segs_to_string(vec![_a, _c, _f]), 7),
        (segs_to_string(vec![_a, _b, _c, _d, _e, _f, _g]), 8),  // This is a gimmie anyway
        (segs_to_string(vec![_a, _b, _c, _d, _f, _g]), 9)
    ])
}

fn segs_to_string(segs: Vec<char>) -> String {
    sort_string(&String::from_iter(segs))
}

fn sort_string(s: &String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_by(|a,b| a.cmp(b));
    String::from_iter(chars)
}

