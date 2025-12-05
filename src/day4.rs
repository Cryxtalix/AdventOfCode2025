use libaocparser_rs::*;
use std::{
    time::Instant,
};

enum RelativePos {
    TopLeft,
    Top,
    TopRight,
    BottomLeft,
    Bottom,
    BottomRight,
    Left,
    Right,
}

fn get_coordinate(pos: &RelativePos, coord: (usize, usize)) -> Option<(usize, usize)> {
    match pos {
        RelativePos::TopLeft => {
            let x = coord.0.checked_sub(1)?;
            let y = coord.1.checked_sub(1)?;
            Some((x, y))
        },
        RelativePos::Top => {
            let x = coord.0;
            let y = coord.1.checked_sub(1)?;
            Some((x, y))
        },
        RelativePos::TopRight => {
            let x = Some(coord.0.strict_add(1)).filter(|i| *i<139)?;
            let y = coord.1.checked_sub(1)?;
            Some((x, y))
        },
        RelativePos::Left => {
            let x = coord.0.checked_sub(1)?;
            let y = coord.1;
            Some((x, y))
        },
        RelativePos::Right => {
            let x = Some(coord.0.strict_add(1)).filter(|i| *i<139)?;
            let y = coord.1;
            Some((x, y))
        },
        RelativePos::BottomLeft => {
            let x = coord.0.checked_sub(1)?;
            let y = Some(coord.1.strict_add(1)).filter(|i| *i<139)?;
            Some((x, y))
        },
        RelativePos::Bottom => {
            let x = coord.0;
            let y = Some(coord.1.strict_add(1)).filter(|i| *i<139)?;
            Some((x, y))
        },
        RelativePos::BottomRight => {
            let x = Some(coord.0.strict_add(1)).filter(|i| *i<139)?;
            let y = Some(coord.1.strict_add(1)).filter(|i| *i<139)?;
            Some((x, y))
        },
    }
}

fn puzzle1(aoc: AocParser) {
    fn is_paper(coord: (usize, usize), buf: &[[u8; 139]; 139]) -> bool {
        buf[coord.0][coord.1] == 64
    }

    fn check_accessible(coord: (usize, usize), buf: &[[u8; 139]; 139]) -> bool {
        let relative_list = [
            RelativePos::TopLeft,
            RelativePos::Top,
            RelativePos::TopRight,
            RelativePos::Left,
            RelativePos::Right,
            RelativePos::BottomLeft,
            RelativePos::Bottom,
            RelativePos::BottomRight,
        ];
        let mut total = 0;

        for pos in &relative_list {
            if let Some(pos_coord) = get_coordinate(pos, coord) && is_paper(pos_coord, buf) {
                total += 1;
            }
        }
        total < 4
    }

    let aoc = aoc.get();
    let mut buf: [[u8; 139]; 139] = [[0; 139]; 139];
    for (idx, line) in aoc.iter().enumerate() {
        buf[idx] = line.as_bytes().try_into().unwrap();
    }

    let mut total = 0;
    for x in 0..139 {
        for y in 0..139 {
            if is_paper((x, y), &buf) && check_accessible((x, y), &buf) {
                total += 1;
            }
        }
    }
    print!("Puzzle 1: {}", total);
}


fn puzzle2(aoc: AocParser) {
    fn is_paper(coord: (usize, usize), buf: &[[u8; 139]; 139]) -> bool {
        let item = buf[coord.0][coord.1];
        item == 64 || item == 35
    }

    fn count_paper(buf: &[[u8; 139]; 139]) -> u32 {
        let mut total = 0;
        for x in 0..139 {
            for y in 0..139 {
                if is_paper((x, y), buf) {
                    total += 1;
                }
            }
        }
        total
    }

    fn mark_accessible(buf: &mut [[u8; 139]; 139]) -> bool {
        let relative_list = [
            RelativePos::TopLeft,
            RelativePos::Top,
            RelativePos::TopRight,
            RelativePos::Left,
            RelativePos::Right,
            RelativePos::BottomLeft,
            RelativePos::Bottom,
            RelativePos::BottomRight,
        ];

        let mut found = false;

        for x in 0..139 {
            for y in 0..139 {
                if !is_paper((x, y), buf) {
                    continue;
                }
                let mut total_surround = 0;
                for pos in &relative_list {
                    if let Some(pos_coord) = get_coordinate(pos, (x, y)) && is_paper(pos_coord, buf) {
                        total_surround += 1;
                    }
                }
                if total_surround < 4 {
                    found |= true;
                    buf[x][y] = 35;
                }
            }
        }
        found
    }

    fn remove_accessible(buf: &mut [[u8; 139]; 139]) {
        for x in 0..139 {
            for y in 0..139 {
                if buf[x][y] == 35 {
                    buf[x][y] = 46;
                }
            }
        }
    }

    let aoc = aoc.get();
    let mut buf: [[u8; 139]; 139] = [[0; 139]; 139];
    for (idx, line) in aoc.iter().enumerate() {
        buf[idx] = line.as_bytes().try_into().unwrap();
    }

    let start = count_paper(&buf);

    loop {
        // Mark all accessible
        if mark_accessible(&mut buf) {
            // Remove all accessible
            remove_accessible(&mut buf);
        } else {
            break;
        }
    }

    let end = count_paper(&buf);
    let removed = start - end;
    print!("Puzzle 2: {}", removed);
}

pub fn run() {
    println!("============= Day 4 =============");

    let aoc = AocParser::new("inputs/day4/input.txt", Separator::Newline).unwrap();

    let now = Instant::now();
    puzzle1(aoc);
    let elapsed = now.elapsed();
    print!(" [{:.2?}]", elapsed);
    println!();

    let aoc = AocParser::new("inputs/day4/input.txt", Separator::Newline).unwrap();

    let now = Instant::now();
    puzzle2(aoc);
    let elapsed = now.elapsed();
    print!(" [{:.2?}]", elapsed);
    println!();
}
