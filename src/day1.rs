use libaocparser_rs::AocParser;
use std::{
    str::FromStr,
    cell::Cell,
};

#[derive(Debug, PartialEq, Eq)]
struct PuzzleError;

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

fn puzzle1(){
    let aoc = AocParser::new("inputs/day1/input.txt").unwrap();
    let aoc: Vec<PuzzleOne> = aoc.slice_as_type(None, None).unwrap();

    let mut dial: u32 = 50;
    let mut times_passed = 0;
    for i in aoc {
        dial += i.value;
        dial %= 100;
        if dial == 0 {
            times_passed += 1;
        }
    }
    println!("Puzzle 1: {}", times_passed);
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

fn puzzle2() {
    let aoc = AocParser::new("inputs/day1/input.txt").unwrap();
    let aoc: Vec<PuzzleTwo> = aoc.slice_as_type(None, None).unwrap();
    
    // Attempt brute force method
    let dial: Cell<u32> = Cell::new(50);
    let times_passed: Cell<u32> = Cell::new(0);

    fn add(dial: &Cell<u32>, times_passed: &Cell<u32>) {
        if dial.get() == 99 {
            dial.set(0);
            times_passed.set(times_passed.get() + 1);
        } else {
            dial.set(dial.get() + 1);
        }
    }

    fn sub(dial: &Cell<u32>, times_passed: &Cell<u32>) {
        if dial.get() == 1 {
            times_passed.set(times_passed.get() + 1);
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
                add(&dial, &times_passed);
            }
        } else {
            for _ in 0..i.value {
                sub(&dial, &times_passed);
            }
        }
    }

    println!("Brute force method: {}", times_passed.get());

    // Attempt modulo method
    let mut dial: i32 = 50;
    let mut times_passed = 0;

    for rotation in aoc {
        let mut value = rotation.value;
        let direction = rotation.direction;

        // convert all values to < 100
        if value > 100 {
            times_passed += value / 100;
            value %= 100;
        }

        if direction { // Right
            dial += value;
            if dial > 99 {
                times_passed += 1;
                dial %= 100;
            }
        }

        else { // Left
            let is_zero = dial == 0;
            dial -= value;
            // Value cannot be more than 100
            // If dial is currently 0, there is no chance of crossing 0 again
            if !is_zero && dial <= 0 {
                times_passed += 1;
            }
            if dial < 0 {
                dial += 100;
            }
        }
    }
    println!("Modulo method: {}", times_passed);
}

pub fn run() {
    println!("Day 1");
    puzzle1();
    puzzle2();
}
