use libaocparser_rs::*;
use std::{
    str::FromStr,
    thread,
    sync::{
        Mutex,
        Arc,
    },
    collections::HashSet,
    time::Instant,
};
use crate::PuzzleError;

#[derive(Debug)]
struct IDRanges {
    first: u64,
    last: u64,
}

impl FromStr for IDRanges {
    type Err = PuzzleError;

    fn from_str(s: &str) -> Result<Self, PuzzleError> {
        let (first, last) = s.trim().split_once("-").ok_or(PuzzleError)?;
        let first: u64 = str::parse(first).map_err(|_|PuzzleError)?;
        let last: u64 = str::parse(last).map_err(|_|PuzzleError)?;
        Ok(IDRanges { first, last })
    }
}

fn puzzle1(threads: u8) {
    fn compute(range: IDRanges) -> u64 {
        let mut ret: u64 = 0;
        for i in range.first..range.last+1 {
            let str_num = i.to_string();
            // Must be an even number length
            let length = str_num.len();
            if length % 2 == 0 {
                let (front, back) = str_num.split_at(length / 2);
                if front == back {
                    ret += i;
                }
            }
        }
        ret
    }

    let aoc = AocParser::new("inputs/day2/input.txt", Separator::Str(",")).unwrap();
    let aoc: Arc<Mutex<Vec<IDRanges>>> = Arc::new(Mutex::new(
        aoc.slice_as_type::<IDRanges>(None, None).unwrap()
    ));
    let result_sum: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));

    let mut threadhandles = Vec::with_capacity(3);
    for _ in 0..threads {
        let work = aoc.clone();
        let result_sum = result_sum.clone();
        let handle = thread::spawn(move || {
            loop {
                let work = {
                    let mut guard = work.lock().unwrap();
                    guard.pop()
                };
                if let Some(range) = work {
                    let ret = compute(range);
                    {
                        let mut guard = result_sum.lock().unwrap();
                        *guard += ret;
                    }
                } else {
                    break;
                }
            }
        });
        threadhandles.push(handle);
    }
    for handle in threadhandles {
        handle.join().unwrap();
    }

    let results = Arc::into_inner(result_sum).unwrap().into_inner().unwrap();
    println!("Puzzle 1: {}", results);
}

// Brute forced! >:-(
fn puzzle2(threads: u8) {
    fn get_divisors(len: usize) -> Vec<usize> {
        match len {
            2 => vec![1],
            3 => vec![1],
            4 => vec![1, 2],
            5 => vec![1],
            6 => vec![1, 2, 3],
            7 => vec![1],
            8 => vec![1, 2, 4],
            9 => vec![1, 3],
            10 => vec![1, 2, 5],
            _ => vec![],
        }
    }

    fn splitter(s: &str, split_len: usize) -> Vec<String> {
        let mut tmp: Vec<String> = Vec::new();
        let (a, b) = s.split_at(split_len);
        tmp.push(a.to_string());
        if b.len() == split_len {
            tmp.push(b.to_string());
        } else {
            tmp.append(&mut splitter(b, split_len));
        }
        tmp
    }

    fn compute(range: IDRanges) -> u64 {
        let mut ret: u64 = 0;
        for i in range.first..range.last+1 {
            let str_num = i.to_string();
            let length = str_num.len();

            let divisors = get_divisors(length);
            for div in divisors {
                let tmp = splitter(&str_num, div);
                let mut tmp_set = HashSet::new();
                for segment in tmp {
                    tmp_set.insert(segment);
                }
                if tmp_set.len() == 1 {
                    ret += i;
                    break;
                }
            }
        }
        ret
    }

    let aoc = AocParser::new("inputs/day2/input.txt", Separator::Str(",")).unwrap();
    let aoc: Arc<Mutex<Vec<IDRanges>>> = Arc::new(Mutex::new(
        aoc.slice_as_type::<IDRanges>(None, None).unwrap()
    ));
    let result_sum: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));

    let mut threadhandles = Vec::with_capacity(3);
    for _ in 0..threads {
        let work = aoc.clone();
        let result_sum = result_sum.clone();
        let handle = thread::spawn(move || {
            loop {
                let work = {
                    let mut guard = work.lock().unwrap();
                    guard.pop()
                };
                if let Some(range) = work {
                    let ret = compute(range);
                    {
                        let mut guard = result_sum.lock().unwrap();
                        *guard += ret;
                    }
                } else {
                    break;
                }
            }
        });
        threadhandles.push(handle);
    }
    for handle in threadhandles {
        handle.join().unwrap();
    }

    let results = Arc::into_inner(result_sum).unwrap().into_inner().unwrap();
    println!("Puzzle 2(naive): {}", results);
}

fn puzzle2_opt() {
    fn get_divisors(len: usize) -> Vec<usize> {
        match len {
            2 => vec![1],
            3 => vec![1],
            4 => vec![1, 2],
            5 => vec![1],
            6 => vec![1, 2, 3],
            7 => vec![1],
            8 => vec![1, 2, 4],
            9 => vec![1, 3],
            10 => vec![1, 2, 5],
            _ => vec![],
        }
    }

    fn splitter(s: &str, split_len: usize) -> Vec<String> {
        let mut tmp: Vec<String> = Vec::new();
        let (a, b) = s.split_at(split_len);
        tmp.push(a.to_string());
        if b.len() == split_len {
            tmp.push(b.to_string());
        } else {
            tmp.append(&mut splitter(b, split_len));
        }
        tmp
    }

    fn repeated_digits(s: u64, n: usize) -> u64 {
        let s = s.to_string();
        let mut ret: String = String::new();
        for _ in 0..n {
            ret.push_str(&s);
        }
        u64::from_str(&ret).unwrap()
    }

    let aoc = AocParser::new("inputs/day2/input.txt", Separator::Str(",")).unwrap();
    let aoc: Vec<IDRanges> = aoc.slice_as_type::<IDRanges>(None, None).unwrap();

    // Create new vector
    // Split ranges with different lengths into two
    // All ranges are of same digit length
    let mut new_aoc: Vec<IDRanges> = Vec::new();
    for ranges in aoc {
        let first_len = ranges.first.to_string().len();
        let last_len = ranges.last.to_string().len();

        if first_len != last_len {
            let mid = repeated_digits(9, first_len);
            // Remove values < 10
            if mid != 9 {
                let top_range = IDRanges {
                    first: ranges.first,
                    last: mid,
                };
                new_aoc.push(top_range);
            }
            let bottom_range = IDRanges {
                first: mid + 1,
                last: ranges.last,
            };
            new_aoc.push(bottom_range);
        } else {
            new_aoc.push(ranges);
        }
    }

    let mut total: u64 = 0;
    for ranges in new_aoc {
        let length = ranges.first.to_string().len();
        let divisors = get_divisors(length);

        // Ensure not to add repeated results across divisors
        let mut found = HashSet::new();

        for div in divisors {
            // Get initial segment
            let segment = splitter(&ranges.first.to_string(), div);
            let segment = &segment[0];
            let segement_start = u64::from_str(segment).unwrap();
            // Get max segment
            let segment = splitter(&ranges.last.to_string(), div);
            let segment = &segment[0];
            let segement_end = u64::from_str(segment).unwrap();

            // Increment segment until out of range
            let repeats = length / div;
            for seg in segement_start..segement_end+1 {
                let val = repeated_digits(seg, repeats);
                if val >= ranges.first && val <= ranges.last {
                    if !found.contains(&val) {
                        found.insert(val);
                        total += val;
                    }
                } else {
                    continue;
                }
            }
        }
    }

    println!("Puzzle 2(optimised): {}", total);
}

pub fn run() {
    println!("============= Day 2 =============");

    let now = Instant::now();
    puzzle1(4);
    let elapsed = now.elapsed();
    println!("Time taken: {:.2?}", elapsed);

    //let now = Instant::now();
    //puzzle2(8);
    //let elapsed = now.elapsed();
    //println!("Time taken: {:.2?}", elapsed);

    puzzle2_opt();
}
