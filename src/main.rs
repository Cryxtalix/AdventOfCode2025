mod day1;
mod day2;
mod day3;

#[derive(Debug, PartialEq, Eq)]
pub struct PuzzleError;

fn main() {
    println!();

    day1::run();
    println!();
    day2::run();
    println!();
    day3::run();
    println!()
}
