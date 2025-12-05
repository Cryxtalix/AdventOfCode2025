use libaocparser_rs::*;
use std::{
    time::Instant,
    str::FromStr,
    cell::Cell,
};
use crate::PuzzleError;

#[derive(Debug)]
struct PuzzleOne {
    value: u32,
}

impl FromStr for PuzzleOne {
    type Err = PuzzleError;

    fn from_str(s: &str) -> Result<Self, PuzzleError> {
        let (dir, val) = s.split_at(1);
        let mut value = u32::from_str(val).map_err(|_|PuzzleError)?;
        if dir == 'L'.to_string() {
            value %= 100;
            value = 100 - value;
        }
        Ok(PuzzleOne { value })
    }
}

fn puzzle1(aoc: AocParser){
    let aoc: Vec<PuzzleOne> = aoc.slice_as_type(None, None).unwrap();

    let mut dial: u32 = 50;
    let mut pass_zero = 0;
    for i in aoc {
        dial += i.value;
        dial %= 100;
        if dial == 0 {
            pass_zero += 1;
        }
    }
    print!("Puzzle 1: {}", pass_zero);
}

#[derive(Debug)]
struct PuzzleTwo {
    // true for right, false for left
    direction: bool,
    value: i32,
}

impl FromStr for PuzzleTwo {
    type Err = PuzzleError;

    fn from_str(s: &str) -> Result<Self, PuzzleError> {
        let (dir, val) = s.split_at(1);
        let value = i32::from_str(val).map_err(|_|PuzzleError)?;
        let direction = dir != 'L'.to_string();
        Ok(PuzzleTwo { direction, value })
    }
}

fn puzzle2_brute(aoc: AocParser) {
    let aoc: Vec<PuzzleTwo> = aoc.slice_as_type(None, None).unwrap();
    
    // Attempt brute force method
    let dial: Cell<u32> = Cell::new(50);
    let pass_zero: Cell<u32> = Cell::new(0);

    fn add(dial: &Cell<u32>, pass_zero: &Cell<u32>) {
        if dial.get() == 99 {
            dial.set(0);
            pass_zero.set(pass_zero.get() + 1);
        } else {
            dial.set(dial.get() + 1);
        }
    }

    fn sub(dial: &Cell<u32>, pass_zero: &Cell<u32>) {
        if dial.get() == 1 {
            pass_zero.set(pass_zero.get() + 1);
        }
        if dial.get() == 0 {
            dial.set(99);
        } else {
            dial.set(dial.get() - 1);
        }
    }

    for i in &aoc {
        if i.direction {
            for _ in 0..i.value {
                add(&dial, &pass_zero);
            }
        } else {
            for _ in 0..i.value {
                sub(&dial, &pass_zero);
            }
        }
    }

    print!("Brute force method: {}", pass_zero.get());
}

fn puzzle2(aoc: AocParser) {
    let aoc: Vec<PuzzleTwo> = aoc.slice_as_type(None, None).unwrap();

    // Attempt modulo method
    let mut dial: i32 = 50;
    let mut pass_zero = 0;

    for rotation in aoc {
        let mut value = rotation.value;
        let direction = rotation.direction;

        // convert all values to < 100
        if value > 100 {
            pass_zero += value / 100;
            value %= 100;
        }

        if direction { // Right
            dial += value;
            if dial > 99 {
                pass_zero += 1;
                dial %= 100;
            }
        }

        else { // Left
            let dial_is_zero = dial == 0;
            dial -= value;
            // Value cannot be more than 100
            // If dial started at 0, there is no chance of crossing 0 again
            if !dial_is_zero && dial <= 0 {
                pass_zero += 1;
            }
            if dial < 0 {
                dial += 100;
            }
        }
    }
    print!("Modulo method: {}", pass_zero);
}

pub fn run() {
    println!("============= Day 1 =============");

    let aoc = AocParser::new("inputs/day1/input.txt", Separator::Newline).unwrap();

    let now = Instant::now();
    puzzle1(aoc);
    let elapsed = now.elapsed();
    print!(" [{:.2?}]", elapsed);
    println!();

    let aoc = AocParser::new("inputs/day1/input.txt", Separator::Newline).unwrap();

    let now = Instant::now();
    puzzle2_brute(aoc);
    let elapsed = now.elapsed();
    print!(" [{:.2?}]", elapsed);
    println!();

    let aoc = AocParser::new("inputs/day1/input.txt", Separator::Newline).unwrap();

    let now = Instant::now();
    puzzle2(aoc);
    let elapsed = now.elapsed();
    print!(" [{:.2?}]", elapsed);
    println!();
}
