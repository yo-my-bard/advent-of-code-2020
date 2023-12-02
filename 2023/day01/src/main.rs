const TEST_INPUT: &str = r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;

fn extract_digits_from_line(l: &str) -> i32 {
    let first_digit = l.chars().find(char::is_ascii_digit).expect("should exist");
    let second_digit = l.chars().rev().find(char::is_ascii_digit).expect("should exist");
    let num_str = String::new() + &*first_digit.to_string() + &*second_digit.to_string();

    num_str.parse::<i32>().unwrap()
}

fn main() {
    println!(
        "{:#}",
        include_str!("day01a.txt")
            .split_ascii_whitespace()
            .map(extract_digits_from_line)
            .sum::<i32>()
    )
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_extracting_digits_from_string() {
        let t = "a1bc23";
        let result = extract_digits_from_line(t);
        assert_eq!(result, 13)
    }

    #[test]
    fn test_test_input() {
        let result: i32 = TEST_INPUT.split_ascii_whitespace().map(extract_digits_from_line).sum();
        assert_eq!(result, 142)
    }
}
