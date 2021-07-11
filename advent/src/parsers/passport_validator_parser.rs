use std::collections::HashMap;
extern crate nom;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{
        alpha1, anychar, digit1, line_ending, multispace0, multispace1, not_line_ending, space1,
    },
    combinator::{map, map_res, not, peek, rest},
    error::{Error, ParseError},
    multi::{count, fold_many0, many0, many1, many_till, separated_list1},
    sequence::{delimited, pair, preceded, terminated},
    IResult,
};
use regex::Regex;
use std::num::ParseIntError;
use std::str::FromStr;

pub fn parse_key_value(input: &str) -> HashMap<&str, &str> {
    parse_key_values(input.split(" ").collect())
}

fn parse_key_values(input: Vec<&str>) -> HashMap<&str, &str> {
    let mut hash: HashMap<&str, &str> = HashMap::new();
    input.iter().for_each(|&item| {
        let split: Vec<&str> = item.split(":").collect();
        hash.insert(split[0], split[1]);
    });
    hash
}

pub fn parse_possible_passports(input: &str) -> IResult<&str, Vec<String>> {
    let mut parser_rest = separated_list1(
        line_ending,
        not_line_ending, // fold_many0(
                         //     terminated(not_line_ending, line_ending),
                         //     String::new(),
                         //     |acc, item| (acc + " " + item).trim().parse().unwrap(),)
    );
    let (leftover, mut parse_01_results) = parse_01(input)?;
    if !leftover.is_empty() {
        let (cool, cooler) = parser_rest(leftover)?;
        parse_01_results.push((cooler, cool));
    }
    combine_parse_01_results(&parse_01_results)
    // map(parser, are_you_kidding_me)(input)
}

// fn are_you_kidding_me(input: Vec<&str>) -> Vec<String> {
//     println!("{:?}", input);
//     let mut start_idx = 0;
//     let mut new_vec: Vec<String> = Vec::new();
//     for (idx, item) in input.iter().enumerate() {
//         let val = input[start_idx..idx].join(" ");
//         if item.is_empty() {
//             new_vec.push(val);
//             start_idx = idx + 1;
//         }
//     }
//     let last_val = input[start_idx..].join(" ");
//     new_vec.push(last_val);
//     new_vec
// }

fn parse_01(s: &str) -> IResult<&str, Vec<(Vec<&str>, &str)>> {
    many0(many_till(
        terminated(not_line_ending, line_ending),
        line_ending,
    ))(s)
}

fn combine_parse_01_results<'a, T>(
    results: &Vec<(Vec<&str>, &str)>,
) -> Result<(&'a str, Vec<String>), T> {
    let mut combined_results: Vec<String> = Vec::new();
    for item in results {
        combined_results.push(item.0.join(" "));
    }
    Ok(("", combined_results))
}

#[derive(Debug, PartialEq)]
pub struct Passport {
    byr: Option<u16>,
    iyr: Option<u16>,
    eyr: Option<u16>,
    hgt: Option<Hgt>,
    hcl: Option<String>,
    ecl: Option<EyeColor>,
    pid: Option<String>,
    cid: Option<String>,
}

#[derive(Debug, PartialEq)]
enum EyeColor {
    Amb,
    Blu,
    Brn,
    Gry,
    Grn,
    Hzl,
    Oth,
}

// https://stackoverflow.com/a/61948093
impl FromStr for EyeColor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "amb" | "Amb" | "AMB" => Ok(EyeColor::Amb),
            "blu" | "Blu" | "BLU" => Ok(EyeColor::Blu),
            "brn" | "Brn" | "BRN" => Ok(EyeColor::Brn),
            "gry" | "Gry" | "GRY" => Ok(EyeColor::Gry),
            "grn" | "Grn" | "GRN" => Ok(EyeColor::Grn),
            "hzl" | "Hzl" | "HZL" => Ok(EyeColor::Hzl),
            "oth" | "Oth" | "OTH" => Ok(EyeColor::Oth),
            _ => Err(String::from("Ya done goofed")),
        }
    }
}

impl Passport {
    pub fn new(parsed_key_values: HashMap<&str, &str>) -> Self {
        Self {
            byr: Self::parse_numeric_u16(&parsed_key_values, "byr"),
            iyr: Self::parse_numeric_u16(&parsed_key_values, "iyr"),
            eyr: Self::parse_numeric_u16(&parsed_key_values, "eyr"),
            hgt: Self::parse_height(&parsed_key_values, "hgt"),
            hcl: Self::parse_string(&parsed_key_values, "hcl"),
            ecl: Self::parse_eye_color(&parsed_key_values, "ecl"),
            pid: Self::parse_string(&parsed_key_values, "pid"),
            cid: Self::parse_string(&parsed_key_values, "cid"),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.has_required_fields()
            && self.has_valid_byr()
            && self.has_valid_iyr()
            && self.has_valid_eyr()
            && self.has_valid_hgt()
            && self.has_valid_hcl()
            && self.has_valid_pid()
    }

    fn has_required_fields(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn has_valid_byr(&self) -> bool {
        self.byr.unwrap() >= 1920 && self.byr.unwrap() <= 2002
    }

    fn has_valid_iyr(&self) -> bool {
        self.iyr.unwrap() >= 2010 && self.iyr.unwrap() <= 2020
    }

    fn has_valid_eyr(&self) -> bool {
        self.eyr.unwrap() >= 2020 && self.eyr.unwrap() <= 2030
    }

    fn has_valid_hgt(&self) -> bool {
        self.hgt.as_ref().unwrap().is_valid()
    }

    fn has_valid_hcl(&self) -> bool {
        match parse_hcl(self.hcl.as_ref().unwrap()) {
            None => false,
            Some(v) => true,
        }
    }

    fn has_valid_pid(&self) -> bool {
        let re = Regex::new(r"^[0-9]{9}$");
        match re {
            Err(_) => false,
            Ok(r) => r.is_match(self.pid.as_ref().unwrap()),
        }
    }

    fn parse_numeric_u16(m: &HashMap<&str, &str>, k: &str) -> Option<u16> {
        match m.get(k) {
            None => None,
            Some(&v) => v.parse::<u16>().ok(),
        }
    }

    fn parse_string(m: &HashMap<&str, &str>, k: &str) -> Option<String> {
        match m.get(k) {
            None => None,
            Some(&v) => Some(String::from(v)),
        }
    }

    fn parse_eye_color(m: &HashMap<&str, &str>, k: &str) -> Option<EyeColor> {
        match m.get(k) {
            None => None,
            Some(&v) => EyeColor::from_str(v).ok(),
        }
    }

    fn parse_height(m: &HashMap<&str, &str>, k: &str) -> Option<Hgt> {
        match m.get(k) {
            None => None,
            Some(&v) => match parser_hgt(v).ok() {
                None => None,
                Some((_, hgt)) => Some(hgt),
            },
        }
    }
}

#[derive(Debug, PartialEq)]
struct Hgt(u8, String);
impl Hgt {
    fn is_valid(&self) -> bool {
        return if self.1.eq_ignore_ascii_case("cm") {
            self.0 >= 150 && self.0 <= 193
        } else if self.1.eq_ignore_ascii_case("in") {
            self.0 >= 59 && self.0 <= 76
        } else {
            false
        };
    }
}

fn parser_hgt(input: &str) -> IResult<&str, Hgt> {
    map(
        pair(digit1, alt((tag("in"), tag("cm")))),
        |pp: (&str, &str)| Hgt(pp.0.parse::<u8>().unwrap(), pp.1.to_owned()),
    )(input)
}

fn parse_hcl(input: &str) -> Option<String> {
    let re = Regex::new(r"^#[0-9a-f]{6}$");
    match re {
        Err(_) => None,
        Ok(r) => {
            if r.is_match(input) {
                Some(String::from(input))
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INVALID_PASSPORTS: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    const VALID_PASSPORTS: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    #[test]
    fn it_should_parse_key_value() {
        let input = String::from("eyr:1972 cid:100 hcl:#18171d");
        let mut hash: HashMap<&str, &str> = HashMap::new();
        hash.insert("eyr", "1972");
        hash.insert("cid", "100");
        hash.insert("hcl", "#18171d");
        assert_eq!(parse_key_value(&input), hash);
    }

    #[test]
    fn it_should_create_a_passport_from_key_values() {
        let mut hash: HashMap<&str, &str> = HashMap::new();
        hash.insert("eyr", "1972");
        hash.insert("cid", "100");
        hash.insert("hcl", "#18171d");
        let expected = Passport {
            byr: None,
            iyr: None,
            eyr: Some(1972),
            hgt: None,
            hcl: Some(String::from("#18171d")),
            ecl: None,
            pid: None,
            cid: Some(String::from("100")),
        };
        assert_eq!(Passport::new(hash), expected);
    }

    #[test]
    fn it_should_invalidate_passport_with_missing_required_fields() {
        let mut hash: HashMap<&str, &str> = HashMap::new();
        hash.insert("eyr", "1972");
        hash.insert("cid", "100");
        hash.insert("hcl", "#18171d");
        let expected = Passport {
            byr: None,
            iyr: None,
            eyr: Some(1972),
            hgt: None,
            hcl: Some(String::from("#18171d")),
            ecl: None,
            pid: None,
            cid: Some(String::from("100")),
        };
        assert_eq!(expected.is_valid(), false);
    }

    #[test]
    fn it_should_invalidate_passport_with_invalid_byr() {
        let mut hash: HashMap<&str, &str> = HashMap::new();
        hash.insert("eyr", "1972");
        hash.insert("cid", "100");
        hash.insert("hcl", "#18171d");
        let expected = Passport {
            byr: Some(1919),
            iyr: Some(2010),
            eyr: Some(2020),
            hgt: Some(Hgt(74, "in".to_owned())),
            hcl: Some(String::from("#623a2f")),
            ecl: EyeColor::from_str("grn").ok(),
            pid: Some(String::from("087499704")),
            cid: Some(String::from("100")),
        };
        assert_eq!(expected.is_valid(), false);
    }

    #[test]
    fn it_should_invalidate_passport_with_invalid_iyr() {
        let mut hash: HashMap<&str, &str> = HashMap::new();
        hash.insert("eyr", "1972");
        hash.insert("cid", "100");
        hash.insert("hcl", "#18171d");
        let expected = Passport {
            byr: Some(1920),
            iyr: Some(2009),
            eyr: Some(2020),
            hgt: Some(Hgt(74, "in".to_owned())),
            hcl: Some(String::from("#623a2f")),
            ecl: EyeColor::from_str("grn").ok(),
            pid: Some(String::from("087499704")),
            cid: Some(String::from("100")),
        };
        assert_eq!(expected.is_valid(), false);
    }

    #[test]
    fn it_should_invalidate_passport_with_invalid_eyr() {
        let mut hash: HashMap<&str, &str> = HashMap::new();
        hash.insert("eyr", "1972");
        hash.insert("cid", "100");
        hash.insert("hcl", "#18171d");
        let expected = Passport {
            byr: Some(1920),
            iyr: Some(2010),
            eyr: Some(2031),
            hgt: Some(Hgt(74, "in".to_owned())),
            hcl: Some(String::from("#623a2f")),
            ecl: EyeColor::from_str("grn").ok(),
            pid: Some(String::from("087499704")),
            cid: Some(String::from("100")),
        };
        assert_eq!(expected.is_valid(), false);
    }

    #[test]
    fn it_should_invalidate_passport_with_invalid_hgt() {
        let mut hash: HashMap<&str, &str> = HashMap::new();
        hash.insert("eyr", "1972");
        hash.insert("cid", "100");
        hash.insert("hcl", "#18171d");
        let expected = Passport {
            byr: Some(1920),
            iyr: Some(2010),
            eyr: Some(2030),
            hgt: Some(Hgt(77, "in".to_owned())),
            hcl: Some(String::from("#623a2f")),
            ecl: EyeColor::from_str("grn").ok(),
            pid: Some(String::from("087499704")),
            cid: Some(String::from("100")),
        };
        assert_eq!(expected.is_valid(), false);
    }

    #[test]
    fn it_should_invalidate_passport_with_invalid_hcl() {
        let mut hash: HashMap<&str, &str> = HashMap::new();
        hash.insert("eyr", "1972");
        hash.insert("cid", "100");
        hash.insert("hcl", "#18171d");
        let expected = Passport {
            byr: Some(1920),
            iyr: Some(2010),
            eyr: Some(2030),
            hgt: Some(Hgt(76, "in".to_owned())),
            hcl: Some(String::from("#123abz")),
            ecl: EyeColor::from_str("grn").ok(),
            pid: Some(String::from("087499704")),
            cid: Some(String::from("100")),
        };
        assert_eq!(expected.is_valid(), false);
    }

    #[test]
    fn it_should_invalidate_passport_with_invalid_ecl() {
        let mut hash: HashMap<&str, &str> = HashMap::new();
        hash.insert("eyr", "1972");
        hash.insert("cid", "100");
        hash.insert("hcl", "#18171d");
        let expected = Passport {
            byr: Some(1920),
            iyr: Some(2010),
            eyr: Some(2030),
            hgt: Some(Hgt(76, "in".to_owned())),
            hcl: Some(String::from("#623a2f")),
            ecl: EyeColor::from_str("abc").ok(),
            pid: Some(String::from("087499704")),
            cid: Some(String::from("100")),
        };
        assert_eq!(expected.is_valid(), false);
    }

    #[test]
    fn it_should_invalidate_passport_with_invalid_pid() {
        let mut hash: HashMap<&str, &str> = HashMap::new();
        hash.insert("eyr", "1972");
        hash.insert("cid", "100");
        hash.insert("hcl", "#18171d");
        let expected = Passport {
            byr: Some(1920),
            iyr: Some(2010),
            eyr: Some(2030),
            hgt: Some(Hgt(76, "in".to_owned())),
            hcl: Some(String::from("#623a2f")),
            ecl: EyeColor::from_str("grn").ok(),
            pid: Some(String::from("0123456789")),
            cid: Some(String::from("100")),
        };
        assert_eq!(expected.is_valid(), false);
    }

    #[test]
    fn it_should_parse_correct_number_of_passports() {
        let expected = vec![
            String::from(
                "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
            ),
            String::from("iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946"),
            String::from(
                "hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
            ),
            String::from("hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007"),
        ];
        let bb = parse_possible_passports(INVALID_PASSPORTS).unwrap();
        assert_eq!(
            parse_possible_passports(INVALID_PASSPORTS),
            Ok(("", expected))
        );
    }

    #[test]
    fn test_count() {
        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            count(line_ending, 2)(s)
        }
        let snippet = "\ns";
        println!("{:?}", not(parser)(snippet).ok());
    }

    #[test]
    fn test_terminated() {
        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            let mut p01 = many0(many_till(
                terminated(not_line_ending, line_ending),
                line_ending,
            ));

            let (input, mut thang) = p01(s)?;
            let (blah, blah2) = separated_list1(line_ending, not_line_ending)(input)?;
            thang.push((blah2, blah));
            println!("{} {:?}", input, thang);

            Ok(("", Vec::new()))
        }
        println!("{:?}", parser(INVALID_PASSPORTS).unwrap())
    }

    #[test]
    fn test_hgt_parser() {
        fn parser(s: &str) -> IResult<&str, (&str, &str)> {
            pair(digit1, alt((tag("in"), tag("cm"))))(s)
        }
        let invalid_snippet = "123in";
        let valid_snippet = "150cm";
        println!("{:?}", parser(invalid_snippet).unwrap());
        assert_eq!(
            parser_hgt(invalid_snippet),
            Ok(("", Hgt(123, "in".to_owned())))
        );
        assert_eq!(parser_hgt(valid_snippet).unwrap().1.is_valid(), true);
    }

    #[test]
    fn test_hair_color_regex() {
        let invalid_snippet = "#123abcz";
        let valid_snippet = "#623a2f";
        assert_eq!(parse_hcl(invalid_snippet), None);
        assert_eq!(parse_hcl(valid_snippet), Some("#623a2f".to_owned()));
    }
}
