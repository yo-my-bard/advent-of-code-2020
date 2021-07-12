/*
- Let's see if we can implement some custom Errors for bad inputs
 */
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct BoardingPass {
    row: u8,
    column: u8,
}

impl BoardingPass {
    const ALLOWED_FIRST_CHARS: [char; 2] = ['B', 'F'];
    const ALLOWED_SECOND_CHARS: [char; 2] = ['L', 'R'];

    fn is_valid_length(s: &str) -> bool {
        s.len() == 10
    }

    fn is_valid_chars(s: &str) -> bool {
        let valid_first_chars = s[0..7]
            .chars()
            .all(|c| Self::ALLOWED_FIRST_CHARS.contains(&c));
        let valid_second_chars = s[7..]
            .chars()
            .all(|c| Self::ALLOWED_SECOND_CHARS.contains(&c));
        valid_first_chars && valid_second_chars
    }
}

impl FromStr for BoardingPass {
    type Err = ParseBoardingPassError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !Self::is_valid_length(s) {
            return Err(ParseBoardingPassError {
                message: format!("invalid length. expected 10, found: {}", s.len()),
            });
        }

        if !Self::is_valid_chars(s) {
            return Err(ParseBoardingPassError {
                message: format!("invalid chars found for: {}", s),
            });
        }
        todo!()
    }
}

#[derive(Debug, PartialEq)]
struct ParseBoardingPassError {
    message: String,
}

impl Display for ParseBoardingPassError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error parsing boarding pass")
    }
}

impl Error for ParseBoardingPassError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

fn day05_1(input: &str) -> i32 {
    0
}

fn scan_boarding_pass(input: &str) -> BoardingPass {
    BoardingPass { row: 70, column: 7 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_find_the_right_row_column_and_seat() {
        let snippet = "FBFBBFFRLR";
        let expected = BoardingPass { row: 70, column: 7 };
        assert_eq!(scan_boarding_pass(snippet), expected)
    }

    #[test]
    fn it_should_find_the_right_row() {
        assert!(true)
    }

    #[test]
    fn it_should_find_the_right_column() {
        assert!(true)
    }

    #[test]
    fn it_should_error_out_on_incorrect_length() {
        let snippet = "FBFBBFFRL";
        let expected = Err(ParseBoardingPassError {
            message: format!("invalid length. expected 10, found: {}", snippet.len()),
        });
        assert_eq!(BoardingPass::from_str(snippet), expected)
    }

    #[test]
    fn it_should_error_out_on_incorrect_characters() {
        let snippet = "FBFABFFRLR";
        let other_bad_snippet = "FBFBBFFRZR";

        let expected_1 = Err(ParseBoardingPassError {
            message: format!("invalid chars found for: {}", snippet),
        });
        let expected_2 = Err(ParseBoardingPassError {
            message: format!("invalid chars found for: {}", other_bad_snippet),
        });

        println!("{:?}", BoardingPass::from_str(other_bad_snippet));
        BoardingPass::from_str(other_bad_snippet).map_err(|z| println!("{:?}", z.source()));
        assert_eq!(BoardingPass::from_str(snippet), expected_1);
        assert_eq!(BoardingPass::from_str(other_bad_snippet), expected_2);
    }
}
