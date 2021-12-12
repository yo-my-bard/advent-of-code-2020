mod day01;
mod day02;
mod day03;
mod day04;
mod inputs;

use inputs::fetch_input_file;

fn main() {
    println!("Solutions for Day 01: (Pt. 1, Pt. 2) is {:?}", do_day01());
    println!("Solutions for Day 02: (Pt. 1, Pt. 2) is {:?}", do_day02());
    println!("Solutions for Day 03: (Pt. 1, Pt. 2) is {:?}", do_day03());
    println!("Solutions for Day 04: (Pt. 1, Pt. 2) is {:?}", do_day04());
}

fn do_day01() -> (usize, usize) {
    use day01::*;
    let pt1 = day01_1_function(&fetch_input_file("src/inputs/day01"));
    let pt2 = day01_2_function(&fetch_input_file("src/inputs/day01"));
    (pt1, pt2)
}

fn do_day02() -> (usize, usize) {
    use day02::*;
    let pt1 = day02_1_fn(&fetch_input_file("src/inputs/day02"));
    let pt2 = day02_2_fn(&fetch_input_file("src/inputs/day02"));
    (pt1, pt2)
}

fn do_day03() -> (usize, usize) {
    use day03::*;
    let pt1 = day03_1_fn(&fetch_input_file("src/inputs/day03"));
    let pt2 = day03_2_fn(&fetch_input_file("src/inputs/day03"));
    (pt1, pt2)
}

fn do_day04() -> (usize, usize) {
    use day04::*;
    let pt1 = day04_1_fn(&fetch_input_file("src/inputs/day04"));
    let pt2 = day04_2_fn(&fetch_input_file("src/inputs/day04"));
    (pt1, pt2)
}
