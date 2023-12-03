#[cfg(test)]
const TEST_INPUT: &str = r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;

#[cfg(test)]
const TEST_INPUT_B: &str = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;

const WORD_DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn extract_digits_from_line(l: &str) -> i32 {
    let first_digit = l.chars().find(char::is_ascii_digit).expect("should exist");
    let second_digit = l
        .chars()
        .rev()
        .find(char::is_ascii_digit)
        .expect("should exist");
    let num_str = String::new() + &*first_digit.to_string() + &*second_digit.to_string();
    num_str.parse::<i32>().unwrap()
}

fn advanced_extract_digits_from_line(l: &str) -> i32 {
    let first_digit = bad_but_simple_get_first_digit(l);
    let second_digit = bad_but_simple_get_second_digit(l);
    (first_digit + second_digit.as_str())
        .parse::<i32>()
        .unwrap()
}

fn bad_but_simple_get_first_digit(l: &str) -> String {
    let first_digit_idx = l.find(|c: char| c.is_ascii_digit()).unwrap_or(usize::MAX);
    let first_word_idx = WORD_DIGITS
        .iter()
        .enumerate()
        .map(|(idx, w)| {
            let x = l.find(w).unwrap_or(usize::MAX);
            (x, w, idx)
        })
        .reduce(|acc, e| if acc.0 <= e.0 { acc } else { e })
        .unwrap();
    if first_word_idx.0 <= first_digit_idx {
        format!("{:#}", first_word_idx.2 + 1)
    } else {
        l.chars().nth(first_digit_idx).unwrap().to_string()
    }
}

fn bad_but_simple_get_second_digit(l: &str) -> String {
    let second_digit_idx = l.rfind(|c: char| c.is_ascii_digit()).unwrap_or(usize::MAX);
    let second_word_idx = WORD_DIGITS
        .iter()
        .enumerate()
        .map(|(idx, w)| {
            let x = l.rfind(w).unwrap_or(usize::MAX);
            (x, w, idx)
        })
        .reduce(|acc, e| {
            if acc.0 != usize::MAX && acc.0 > e.0 {
                acc
            } else if e.0 != usize::MAX {
                e
            } else {
                acc
            }
        })
        .unwrap();

    // println!(
    //     "WORD {:?} --- {:?} --- {:?}",
    //     l, second_digit_idx, second_word_idx
    // );

    let last_word = format!("{:#}", second_word_idx.2 + 1);
    let last_digit = l.chars().nth(second_digit_idx).unwrap_or('0').to_string();

    if usize::MAX == second_word_idx.0 {
        last_digit
    } else if second_word_idx.0 > second_digit_idx {
        last_word
    } else if second_digit_idx != usize::MAX {
        last_digit
    } else {
        last_word
    }
}

fn main() {
    println!(
        "First Answer: {:#}",
        include_str!("day01.txt")
            .split_ascii_whitespace()
            .map(extract_digits_from_line)
            .sum::<i32>()
    );

    println!(
        "Second Answer: {:#}",
        include_str!("day01.txt")
            .split_ascii_whitespace()
            .map(advanced_extract_digits_from_line)
            // .map(|i| {
            //     println!("{:?}", i);
            //     i
            // })
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
    fn test_test_input_a() {
        let result: i32 = TEST_INPUT
            .split_ascii_whitespace()
            .map(extract_digits_from_line)
            .sum();
        assert_eq!(result, 142)
    }

    #[test]
    fn test_test_input_b() {
        let result: i32 = TEST_INPUT_B
            .split_ascii_whitespace()
            .map(advanced_extract_digits_from_line)
            .map(|i| {
                println!("{:?}", i);
                i
            })
            .sum();
        assert_eq!(result, 281);
    }
}
