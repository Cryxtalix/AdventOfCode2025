use libaocparser_rs::*;
use std::{
    time::Instant,
};

fn puzzle1(aoc: AocParser) {
    fn get_highest_first(s: &str) -> (u32, usize) {
        // Exclude last position, find highest value digit
        let (exclude, _) = s.split_at(99);
        let exclude = exclude.as_bytes();

        // Select earlier one(unlike iter.max())
        let mut max = 0;
        let mut max_pos = 0;
        for (i, &value) in exclude.iter().enumerate() {

            if value > max {
                max = value;
                max_pos = i;
            }
        }
        let max = max as char;
        let max = char::to_digit(max, 10).unwrap();
        (max, max_pos)
    }

    fn get_highest_second(s: &str) -> u32 {
        // Find highest digit from substring
        // No need to exclude
        let max = s.as_bytes().iter().max().unwrap();
        let max = *max as char;
        char::to_digit(max, 10).unwrap()
    }

    let aoc = aoc.get();

    let mut total = 0;
    for bank in aoc {
        let (first_digit, pos) = get_highest_first(&bank);
        let (_, sub) = bank.split_at(pos + 1);
        let second_digit = get_highest_second(sub);
        let final_val = (first_digit * 10) + second_digit;
        total += final_val;
    }
    print!("Puzzle 1: {}", total);
}

fn puzzle2(aoc: AocParser) {
    // Select earliest highest value digit
    fn get_highest(s: &str, start_pos: usize, end_pos: usize) -> (usize, u64) {
        let slice = &s[start_pos..end_pos];
        let slice = slice.as_bytes();
        let (pos, val) = slice
            .iter()
            .rev()
            .enumerate()
            .max_by_key(|(_, val)| **val).unwrap();
        // Invert rev value with -(pos - (len - 1))
        // Then add start pos to get real position
        let pos = (-((pos as i32) - ((slice.len() as i32) - 1))) + (start_pos as i32);
        let pos = pos as usize;
        let val = *val as char;
        let val: u64 = val.to_string().parse().unwrap();
        (pos, val)
    }

    let aoc = aoc.get();
 
    let mut total = 0;
    for bank in aoc {
        let mut bank_max = 0;
        let mut front_search_limit: usize = 0;

        for i in (1..13).rev() {
            let end_search_limit = 100 - i + 1;
            let (pos, val) = get_highest(&bank, front_search_limit, end_search_limit);
            front_search_limit = pos + 1;
            let ten: u64 = 10;
            let val = val * ten.pow((i - 1) as u32);
            bank_max += val;
        }
        total += bank_max;
    }
    print!("Puzzle 2: {}", total);
}

pub fn run() {
    println!("============= Day 3 =============");

    let aoc = AocParser::new("inputs/day3/input.txt", Separator::Newline).unwrap();

    let now = Instant::now();
    puzzle1(aoc);
    let elapsed = now.elapsed();
    print!(" [{:.2?}]", elapsed);
    println!();

    let aoc = AocParser::new("inputs/day3/input.txt", Separator::Newline).unwrap();

    let now = Instant::now();
    puzzle2(aoc);
    let elapsed = now.elapsed();
    print!(" [{:.2?}]", elapsed);
    println!();
}
