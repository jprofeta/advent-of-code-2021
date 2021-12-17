use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::str::FromStr;
use std::convert::TryInto;

mod day03_input;

struct Input { width: usize, diag: Vec<String> }

fn make_input<'a, I>(val: I) -> Input
where I: IntoIterator<Item=&'a str> {
    let strs = Vec::from_iter(val.into_iter().map(|a| String::from(a)));
    let width = strs[0].len();
    Input { width: width, diag: strs }
}

fn main() {
    println!("Advent of Code");
    println!("Day 03 - Binary Diagnostic");
    println!();

    println!("Part 1");
    println!("======");
    println!();

    let puzzle_test_out1 = do_part1(make_input(day03_input::TEST_INPUT));
    println!("Test output: {} (expected {})", puzzle_test_out1, day03_input::TEST_RESULT_PART1);
    assert_eq!(day03_input::TEST_RESULT_PART1, puzzle_test_out1);

    println!();
    println!("Running puzzle input...");
    let puzzle_out1 = do_part1(make_input(day03_input::PUZZLE_INPUT));
    println!("Puzzle result: {}", puzzle_out1);
    println!();

    println!("Part 2");
    println!("======");
    println!();

    let test_out2 = do_part2(make_input(day03_input::TEST_INPUT));
    println!("Test output: {} (expected {})", test_out2, day03_input::TEST_RESULT_PART2);
    assert_eq!(day03_input::TEST_RESULT_PART2, test_out2);

    println!();
    println!("Running puzzle input...");
    let puzzle_out2 = do_part2(make_input(day03_input::PUZZLE_INPUT));
    println!("Puzzle result: {}", puzzle_out2);
    println!();
}

fn do_part1(input: Input) -> i32 {
    let cnt: i32 = input.diag.len().try_into().unwrap();
    let width: i32 = input.width.try_into().unwrap();
    let mut sums: Vec<i32> = Vec::with_capacity(input.width);
    sums.resize(input.width, 0);

    for msg in input.diag.into_iter() {
        msg.chars().enumerate().for_each(|(i,c)| {
            if c == '1' { sums[i] = sums[i] + 1; }
        });
    }
    let gamma =
        sums
            .iter()
            .map(|s| {
                if s >= &(cnt / 2) { 1 }
                else { 0 } 
            }).fold(0, |acc,b| {
                (acc << 1) | b
            });
    let epsilon = (!gamma) & ((1 << width) - 1);
    gamma * epsilon
}

fn do_part2(actions: Input) -> i32 {
    0
}

fn reduce_list<'a, I>(val: I, bit_pos: usize, take_larger: bool) -> Vec<String>
where I: IntoIterator<Item=&'a String> {
    let limit = val.len().try_into::<i32>.unwrap() / 2;
    let bitSum = 
        val.into_iter()
            .map(|x| x.get(bit_pos).unwrap())
            .fold(0, |acc,x| {
                if x == '1' { acc + 1 }
                else { acc }
            });

    if (bitSum >= limit && take_targer) || (bitSum < limit && !take_larger) {
        Vec::from_iter(
            val
            .into_iter()
            .filter(|x| x.get(bit_pos).unwrap() == '1'))
    } else {
        Vec::from_iter(
            val
            .into_iter()
            .filter(|x| x.get(bit_pos).unwrap() == '0'))
    }
}
