use libaocparser_rs::*;

fn puzzle1() {
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

    let aoc = AocParser::new("inputs/day3/input.txt", Separator::Newline).unwrap();
    let aoc = aoc.get();

    let mut total = 0;
    for bank in aoc {
        let (first_digit, pos) = get_highest_first(&bank);
        let (_, sub) = bank.split_at(pos + 1);
        let second_digit = get_highest_second(sub);
        let final_val = (first_digit * 10) + second_digit;
        total += final_val;
    }
    println!("Day 3: {}", total);
}

pub fn run() {
    println!("============= Day 3 =============");
    puzzle1();
}
