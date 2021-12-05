/*
TODO:
- Review parser logic. Understand types, if a non-byte is possible
- So many unwraps. Can they be more idiomatic Rust?
- Review TDD process: was able to write a test and get it to compile and green quickly by brute force.
  Stuck at what test/step should naturally come next?
 */
use crate::parsers::parse_input_string;

pub fn day02_1_function(input: &str) -> i32 {
    let vec: Vec<&str> = input.split("\n").collect();
    let mut count = 0;
    for v in vec.clone() {
        match parse_input_string(v.trim().as_bytes()) {
            Err(_) => {}
            Ok((unmatched, pv)) => {
                if unmatched.is_empty() && pv.is_valid() {
                    count += 1
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::day02_1_function;

    #[test]
    fn it_should_find_number_of_valid_policies() {
        let snippet = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        assert_eq!(day02_1_function(snippet), 2);
    }
}
