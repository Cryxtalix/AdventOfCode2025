use libaocparser_rs::*;
use std::{
    time::Instant,
    str::FromStr,
};
use crate::PuzzleError;

#[derive(Debug)]
struct FreshRangeInc {
    start: u64,
    end: u64,
}

impl FromStr for FreshRangeInc {
    type Err = PuzzleError;

    fn from_str(s: &str) -> Result<Self, PuzzleError> {
        if let Some((start, end)) = s.split_once("-") {
            let start = u64::from_str(start).map_err(|_| PuzzleError)?;
            let end = u64::from_str(end).map_err(|_| PuzzleError)?;
            Ok(FreshRangeInc {
                start,
                end,
            })
        } else {
            Err(PuzzleError)
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct IngredientID(u64);

impl FromStr for IngredientID {
    type Err = PuzzleError;

    fn from_str(s: &str) -> Result<Self, PuzzleError> {
        let id = u64::from_str(s).map_err(|_| PuzzleError)?;
        Ok(IngredientID(id))
    }
}

fn puzzle1(aoc: AocParser) {
    fn is_fresh(fresh_ranges: &Vec<FreshRangeInc>, id: &u64) -> bool {
        let mut is_within = false;
        for range in fresh_ranges {
            if *id >= range.start && *id <= range.end {
                is_within |= true;
                break;
            }
        }
        is_within
    }

    let fresh_ranges: Vec<FreshRangeInc> = aoc.slice_as_type(None, Some(167)).unwrap();
    let ingredients: Vec<u64> = aoc.slice_as_type(Some(169), None).unwrap();

    let mut total = 0;
    for id in &ingredients {
        if is_fresh(&fresh_ranges, id) {
            total += 1;
            continue;
        }
    }

    print!("Puzzle 1: {}", total);
}

fn puzzle2(aoc: AocParser) {
    /// Groups are ranges that can be coalesced into a single continuous range
    /// Returns index of final member and largest final id in group
    fn find_group(fresh_ranges: &[FreshRangeInc], start_idx:  usize) -> (usize, u64) {
        let mut largest = fresh_ranges[start_idx].end;
        let mut last = 0;
        // Find last idx where start is less than largest
        for (idx, range) in fresh_ranges.iter().enumerate().skip(start_idx) {
            // Continually update largest
            if range.start <= largest && range.end > largest {
                largest = range.end;
            }
            if range.start > largest {
                break;
            }
            last = idx;
        }
        (last, largest)
    }

    let mut fresh_ranges: Vec<FreshRangeInc> = aoc.slice_as_type(None, Some(167)).unwrap();
    fresh_ranges.sort_unstable_by_key(|a| a.start);
    let length = fresh_ranges.len();

    let mut total: u64 = 0;
    let mut start_idx = 0;
    // Repeatedly find groups until there are none left
    loop {
        let (last_idx, largest) = find_group(&fresh_ranges, start_idx);
        let size_of_group = largest - fresh_ranges[start_idx].start + 1;
        total += size_of_group;
        if last_idx == length - 1 {
            break;
        } else {
            start_idx = last_idx + 1;
        }
    }

    print!("Puzzle 2: {}", total);
}

pub fn run() {
    println!("============= Day 5 =============");

    let aoc = AocParser::new("inputs/day5/input.txt", Separator::Newline).unwrap();

    let now = Instant::now();
    puzzle1(aoc);
    let elapsed = now.elapsed();
    print!(" [{:.2?}]", elapsed);
    println!();

    let aoc = AocParser::new("inputs/day5/input.txt", Separator::Newline).unwrap();

    let now = Instant::now();
    puzzle2(aoc);
    let elapsed = now.elapsed();
    print!(" [{:.2?}]", elapsed);
    println!();
}
