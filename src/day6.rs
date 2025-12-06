use libaocparser_rs::*;
use std::{
    time::Instant,
};

#[derive(Debug)]
enum Ops {
    Add,
    Multiply,
}

fn puzzle1(aoc: AocParser) {
    fn operation(a: u64, b: u64, c: u64, d: u64, op: &Ops) -> u64 {
        match op {
            Ops::Add => a + b + c + d,
            Ops::Multiply => a * b * c * d,
        }
    }

    let mut number_lists: Vec<Vec<u64>> = Vec::new();
    for i in 0..4 {
        let num_list: Vec<u64> = aoc.data[i]
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        number_lists.push(num_list);
    }
    let ops_list: Vec<Ops> = aoc
        .data[4]
        .split_whitespace()
        .map(|s| {
            match s {
                "+" => Ops::Add,
                "*" => Ops::Multiply,
                // Shouldn't happen
                _ => Ops::Add,
            }
        })
        .collect();

    // All are of the same length!
    assert_eq!(number_lists[0].len(), number_lists[1].len());
    assert_eq!(number_lists[0].len(), number_lists[2].len());
    assert_eq!(number_lists[0].len(), number_lists[3].len());
    assert_eq!(number_lists[0].len(), ops_list.len());

    let mut total = 0;
    for i in 0..1000 {
        let a = number_lists[0][i];
        let b = number_lists[1][i];
        let c = number_lists[2][i];
        let d = number_lists[3][i];
        let op = &ops_list[i];
        total += operation(a, b, c, d, op);
    }
    print!("Puzzle 1: {}", total);
}

fn col_buf_to_int(val: &[u8]) -> u64 {
    let mut ret = 0;
    for (pos, num) in val.iter().rev().enumerate() {
        let power = u64::pow(10, pos as u32);
        let num = (num - b'0') as u64;
        ret += num * power;
    }
    ret
}

fn puzzle2(aoc: AocParser) {
    fn operation(nums: &[u64], op: &Ops) -> u64 {
        match op {
            Ops::Add => {
                nums.iter().sum()
            },
            Ops::Multiply => {
                nums.iter().product()
            },
        }
    }
    
    let mut num_lines = aoc.get();
    // Split off!
    let op_line = num_lines.pop().unwrap();
    
    let num_lines: Vec<&[u8]> = num_lines.iter().map(|s| s.as_bytes()).collect();

    // Buffers
    let mut num_list: Vec<Vec<u64>> = Vec::new();
    let mut num_buf: Vec<u64> = Vec::new();
    let mut col_buf: Vec<u8> = Vec::new();

    // Run through lines in reverse
    for idx in (0..num_lines[0].len()).rev() {
        for i in 0..num_lines.len() {
            let c = num_lines[i][idx];
            if c != 32 {
                col_buf.push(c);
            }
        }
        // If whole line is empty, reached boundary
        if col_buf.is_empty() {
            num_list.push(num_buf.clone());
            num_buf.clear();
        } else {
            let val = col_buf_to_int(&col_buf);
            num_buf.push(val);
            col_buf.clear();
        }
    }
    // Push last remaining to the vector
    num_list.push(num_buf.clone());

    let ops_list: Vec<Ops> = op_line
        .split_whitespace()
        .map(|s| {
            match s {
                "+" => Ops::Add,
                "*" => Ops::Multiply,
                // Shouldn't happen
                _ => Ops::Add,
            }
        })
        .rev() // Reverse as the numbers are also reversed!
        .collect();

    // All are of the same length!
    assert_eq!(num_list.len(), ops_list.len());

    let mut total = 0;
    for (idx, numbers) in num_list.iter().enumerate() {
        let res = operation(numbers, &ops_list[idx]);
        total += res;
    }
    print!("Puzzle 2: {}", total);
}

pub fn run() {
    println!("============= Day 6 =============");

    let aoc = AocParser::new("inputs/day6/input.txt", Separator::Newline).unwrap();

    let now = Instant::now();
    puzzle1(aoc);
    let elapsed = now.elapsed();
    print!(" [{:.2?}]", elapsed);
    println!();

    let aoc = AocParser::new("inputs/day6/input.txt", Separator::Newline).unwrap();

    let now = Instant::now();
    puzzle2(aoc);
    let elapsed = now.elapsed();
    print!(" [{:.2?}]", elapsed);
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_col_buf_to_int() {
        let test: Vec<u8> = vec![51, 53, 54];
        let out = col_buf_to_int(&test);
        assert_eq!(out, 431);

        let test: Vec<u8> = vec![32, 32, 55, 56];
        let out = col_buf_to_int(&test);
        assert_eq!(out, 78);
    }
}
