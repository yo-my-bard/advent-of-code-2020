mod day01_1;
mod day01_2;
mod inputs;
use inputs::fetch_input_file;

fn main() {
    println!("Solution for Day 01 Pt. 1 is {}", do_day01_1());
    println!("Solution for Day 01 Pt. 2 is {}", do_day01_2());
}

fn do_day01_1() -> i32 {
    use day01_1::{day01_1_function};
    day01_1_function(&fetch_input_file("src/inputs/day01"))
}

fn do_day01_2() -> i32 {
    use day01_2::day01_2_function;
    day01_2_function(&fetch_input_file("src/inputs/day01"))
}
