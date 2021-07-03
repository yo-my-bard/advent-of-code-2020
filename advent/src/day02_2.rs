use crate::parsers::parse_input_string;

pub fn day02_2_function(input: &str) -> i32 {
    let vec: Vec<&str> = input.split("\n").collect();
    let mut count = 0;
    for v in vec.clone() {
        match parse_input_string(v.trim().as_bytes()) {
            Err(_) => {}
            Ok((unmatched, pv)) => {
                if unmatched.is_empty() && pv.is_positionally_valid() {
                    count += 1
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::day02_2_function;

    #[test]
    fn it_should_count_total_positionally_valid_passwords() {
        let snippet =  "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        assert_eq!(day02_2_function(snippet), 1)
    }
}