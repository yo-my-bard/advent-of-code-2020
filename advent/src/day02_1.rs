/*
TODO:
- Review parser logic. Understand types, if a non-byte is possible
- So many unwraps. Can they be more idiomatic Rust?
- Review TDD process: was able to write a test and get it to compile and green quickly by brute force.
  Stuck at what test/step should naturally come next?
 */
extern crate nom;
use nom::{
    IResult,
    bytes::complete::{take_while, is_a},
    character::{is_alphabetic, is_space, is_digit},
    character::complete::char,
    sequence::{tuple}
};
use self::nom::Err;
use self::nom::error::Error;

fn parse_occurrence(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(is_digit)(input)
}

fn parse_pattern(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(is_alphabetic)(input)
}

fn parse_password(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_while(is_alphabetic)(input)
}

pub fn parse_input_string(input: &[u8]) -> IResult<&[u8], PasswordValidator> {
    let mut parser =
        tuple((parse_occurrence, is_a("-"), parse_occurrence, is_a(" "), parse_pattern,
               is_a(":"), is_a(" "), parse_password));
    let (input, (min_o, _, max_o, _, pattern, _, _, password)) = parser(input)?;
    Ok((input,
        PasswordValidator {
            min_occurrence: String::from_utf8(min_o.to_vec()).unwrap().parse::<u8>().unwrap(),
            max_occurrence: String::from_utf8(max_o.to_vec()).unwrap().parse::<u8>().unwrap(),
            pattern: String::from_utf8(pattern.to_vec()).unwrap(),
            password: String::from_utf8(password.to_vec()).unwrap()
        }))
}

#[derive(Debug,PartialEq)]
pub struct PasswordValidator {
    min_occurrence: u8,
    max_occurrence: u8,
    pattern: String,
    password: String
}

impl PasswordValidator {
    pub fn is_valid(&self) -> bool {
        let occurrences = self.password.matches(&self.pattern).count();
        occurrences.ge(&usize::from(self.min_occurrence)) &&
            occurrences.le(&usize::from(self.max_occurrence))
    }
}

pub fn day02_1_function(input: &str) -> i32 {
    let vec: Vec<&str> = input.split("\n").collect();
    let mut count = 0;
    for v in vec.clone() {
        match parse_input_string(v.trim().as_bytes()) {
            Err(_) => {},
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
    use super::parse_input_string;
    use super::parse_occurrence;
    use crate::day02_1::PasswordValidator;

    #[test]
    fn it_should_find_number_of_valid_policies() {
        let snippet = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        assert_eq!(day02_1_function(snippet), 2);
    }

    #[test]
    fn it_should_parse_a_policy() {
        let policy = "1-3 a: abcde".as_bytes();
        let parsed_policy = parse_input_string(policy);
        assert_eq!(parsed_policy, Ok(("".as_bytes(), PasswordValidator {
            min_occurrence: 1,
            max_occurrence: 3,
            pattern: String::from("a"),
            password: String::from("abcde")
        })));
    }

    #[test]
    fn it_should_parse_occurrences() {
        let boxed_array = Box::new([b'1', b'-', b'2']);
        let array: [u8; 2] = [b'-', b'2'];
        let result = parse_occurrence(&boxed_array[..]);
        let result2 = parse_occurrence(&array);
        assert_eq!(result, Ok((Vec::from("-2").as_slice(), Vec::from("1").as_slice())));
        assert_eq!(result2, Ok((Vec::from("-2").as_slice(), Vec::from("").as_slice())));
    }

    #[test]
    fn it_should_return_true_for_valid_password_validator() {
        let pv = PasswordValidator {
            min_occurrence: 1,
            max_occurrence: 3,
            pattern: String::from("a"),
            password: String::from("abcde")
        };
        assert_eq!(pv.is_valid(), true);
    }
}