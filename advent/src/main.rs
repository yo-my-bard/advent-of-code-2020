mod day01pt1;
mod day01pt2;
use day01pt1::fetch_the_input;

fn main() {
    println!("Solution for Day 01 Pt. 1 is {}", do_day01pt01());
    println!("Solution for Day 01 Pt. 2 is {}", do_day_1_2());
}

fn do_day01pt01() -> i32 {
    use day01pt1::{day1_1_function};
    day1_1_function(fetch_the_input())
}

fn do_day_1_2() -> i32 {
    use day01pt2::day1_2_function;
    day1_2_function(fetch_the_input())
}
