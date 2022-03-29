mod day01_1;
mod day01_2;
mod day02_1;
mod day02_2;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod inputs;
mod parsers;
#[macro_use]
extern crate pest_derive;

use inputs::fetch_input_file;

fn main() {
    println!("Solution for Day 01 Pt. 1 is {}", do_day01_1());
    println!("Solution for Day 01 Pt. 2 is {}", do_day01_2());
    println!("Solution for Day 02 Pt. 1 is {}", do_day02_1());
    println!("Solution for Day 02 Pt. 2 is {}", do_day02_2());
    println!("Solutions for Day 03: (Pt. 1, Pt. 2) is {:?}", do_day03());
    println!("Solutions for Day 04: (Pt. 1, Pt. 2) is {:?}", do_day04());
    println!("Solutions for Day 05: (Pt. 1, Pt. 2) is {:?}", do_day05());
    println!("Solutions for Day 06: (Pt. 1, Pt. 2) is {:?}", do_day06());
    println!("Solutions for Day 07: (Pt. 1, Pt. 2) is {:?}", do_day07());
}

fn do_day01_1() -> i32 {
    use day01_1::day01_1_function;
    day01_1_function(&fetch_input_file("src/inputs/day01"))
}

fn do_day01_2() -> i32 {
    use day01_2::day01_2_function;
    day01_2_function(&fetch_input_file("src/inputs/day01"))
}

fn do_day02_1() -> i32 {
    use day02_1::day02_1_function;
    day02_1_function(&fetch_input_file("src/inputs/day02"))
}

fn do_day02_2() -> i32 {
    use day02_2::day02_2_function;
    day02_2_function(&fetch_input_file("src/inputs/day02"))
}

fn do_day03() -> (usize, usize) {
    use day03::*;
    let pt1 = day03_1_function(
        &fetch_input_file("src/inputs/day03"),
        Slope { right: 3, down: 1 },
    );
    let pt2 = day03_2_function(&fetch_input_file("src/inputs/day03"), get_slopes());
    (pt1, pt2)
}

fn do_day04() -> (usize, usize) {
    use day04::*;
    let pt1 = day04_1(&fetch_input_file("src/inputs/day04"));
    let pt2 = day04_2(&fetch_input_file("src/inputs/day04"));
    (pt1, pt2)
}

fn do_day05() -> (u32, u32) {
    use day05::*;
    let pt1 = day05_1(&fetch_input_file("src/inputs/day05"));
    let pt2 = day05_2(&fetch_input_file("src/inputs/day05"));
    (pt1, pt2)
}

fn do_day06() -> (usize, usize) {
    use day06::*;
    let pt1 = day06_1(&fetch_input_file("src/inputs/day06"));
    let pt2 = day06_2(&fetch_input_file("src/inputs/day06"));
    (pt1, pt2)
}

fn do_day07() -> (usize, i32) {
    use day07::*;
    let pt1 = day07_1(&fetch_input_file("src/inputs/day07"));
    let pt2 = day07_2(&fetch_input_file("src/inputs/day07"));
    (pt1, pt2)
}
