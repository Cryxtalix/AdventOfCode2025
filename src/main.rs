mod day1;
mod day2;

#[derive(Debug, PartialEq, Eq)]
pub struct PuzzleError;

fn main() {
    day1::run();
    println!();
    day2::run();
    println!();
}
