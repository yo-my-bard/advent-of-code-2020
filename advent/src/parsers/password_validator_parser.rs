extern crate nom;
use nom::{
    IResult,
    bytes::complete::{take_while, is_a},
    character::{is_alphabetic, is_digit},
    sequence::{tuple}
};

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
            min_occurrence: String::from_utf8(min_o.to_vec()).unwrap().parse::<usize>().unwrap(),
            max_occurrence: String::from_utf8(max_o.to_vec()).unwrap().parse::<usize>().unwrap(),
            pattern: String::from_utf8(pattern.to_vec()).unwrap(),
            password: String::from_utf8(password.to_vec()).unwrap()
        }))
}

#[derive(Debug,PartialEq)]
pub struct PasswordValidator {
    min_occurrence: usize,
    max_occurrence: usize,
    pattern: String,
    password: String
}

impl PasswordValidator {
    pub fn is_valid(&self) -> bool {
        let occurrences = self.password.matches(&self.pattern).count();
        occurrences.ge(&usize::from(self.min_occurrence)) &&
            occurrences.le(&usize::from(self.max_occurrence))
    }

    pub fn is_positionally_valid(&self) -> bool {
        (self.password.chars().nth(self.min_occurrence - 1).unwrap() == self.pattern.parse().unwrap() ||
            self.password.chars().nth(self.max_occurrence - 1).unwrap() == self.pattern.parse().unwrap()) &&
            !(self.password.chars().nth(self.min_occurrence - 1).unwrap() == self.password.chars().nth(self.max_occurrence - 1).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_occurrence, PasswordValidator, parse_input_string};

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

    #[test]
    fn it_should_be_valid_when_only_one_position_matches_the_pattern() {
        let pv = PasswordValidator {
            min_occurrence: 1,
            max_occurrence: 3,
            pattern: String::from("a"),
            password: String::from("abcde")
        };
        assert_eq!(pv.is_positionally_valid(), true);
    }

    #[test]
    fn it_should_be_invalid_when_neither_position_matches_the_pattern() {
        let pv = PasswordValidator {
            min_occurrence: 1,
            max_occurrence: 3,
            pattern: String::from("b"),
            password: String::from("cdefg")
        };
        assert_eq!(pv.is_positionally_valid(), false);
    }

    #[test]
    fn it_should_be_invalid_when_both_positions_match_the_pattern() {
        let pv = PasswordValidator {
            min_occurrence: 1,
            max_occurrence: 3,
            pattern: String::from("c"),
            password: String::from("ccccccccc")
        };
        assert_eq!(pv.is_positionally_valid(), false);
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
}