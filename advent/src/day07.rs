use std::collections::{HashMap, HashSet};
extern crate pest;
use self::pest::iterators::Pairs;
use pest::Parser;

pub fn day07_1(input: &str) -> usize {
    let all_bags = parse_many_bags_data(input);
    find_possible_bag_holders_for_a_bag("shiny gold", &all_bags).len()
}

pub(crate) fn day07_2(input: &str) -> usize {
    0
}

fn parse_single_bag_data(rule: &str) -> Bag {
    let bag_data: Result<Pairs<Rule>, pest::error::Error<Rule>> = BagParser::parse(Rule::bag, rule);
    let mut color = "";
    let mut contains: Vec<HashMap<String, String>> = Vec::new();
    bag_data
        .and_then(|mut pairs| match pairs.next() {
            Some(pair) => Ok(pair),
            _ => unreachable!(),
        })
        .and_then(|pair| {
            for innards in pair.into_inner() {
                match innards.as_rule() {
                    Rule::subject => {
                        color = innards.as_str();
                    }
                    Rule::bag_item => {
                        let mut contains_map: HashMap<String, String> = HashMap::new();
                        for descriptor in innards.into_inner() {
                            match descriptor.as_rule() {
                                Rule::color => {
                                    contains_map.insert(
                                        "color".to_string(),
                                        descriptor.as_str().to_string(),
                                    );
                                }
                                Rule::quant => {
                                    contains_map.insert(
                                        "quantity".to_string(),
                                        descriptor.as_str().to_string(),
                                    );
                                }
                                _ => (),
                            }
                        }
                        contains.push(contains_map);
                    }
                    _ => (),
                }
            }
            Ok(())
        });

    Bag {
        color: color.to_string(),
        contains,
    }
}

fn parse_many_bags_data(bags: &str) -> Vec<Bag> {
    let mut bags_list: Vec<Bag> = Vec::new();
    let to_parse: Vec<&str> = bags.split("\n").collect();
    for maybe_bag in to_parse {
        bags_list.push(parse_single_bag_data(maybe_bag))
    }
    bags_list
}

fn find_direct_bag_holders_for_a_bag<'a, 'b>(color: &'b str, bags: &'a Vec<Bag>) -> Vec<&'a Bag> {
    bags.iter()
        .filter(|&bag| {
            bag.contains
                .iter()
                .any(|baguette| baguette.get("color").map_or(false, |col| color == col))
        })
        .collect()
}

fn find_possible_bag_holders_for_a_bag<'a, 'b>(
    init_color: &'b str,
    bags: &'a Vec<Bag>,
) -> Vec<&'a Bag> {
    let mut color_stack: Vec<&Bag> = Vec::new();
    let mut possible_holders_stack: Vec<&'a Bag> = Vec::new();
    let mut visited_colors = HashSet::new();
    let mut done = false;
    let mut color = init_color;

    while !done {
        if visited_colors.contains(color) {
            if color_stack.is_empty() {
                done = true;
            } else {
                let next_bag = color_stack.pop().unwrap();
                color = next_bag.color.as_str();
            }
            continue;
        }
        let mut possibilities = find_direct_bag_holders_for_a_bag(color, bags);
        color_stack.append(&mut possibilities);
        visited_colors.insert(color);
        if color_stack.is_empty() {
            done = true;
        } else {
            let next_bag = color_stack.pop().unwrap();
            possible_holders_stack.push(next_bag);
            color = next_bag.color.as_str();
        }
    }
    possible_holders_stack
}

#[derive(Debug, PartialEq)]
struct Bag {
    color: String,
    contains: Vec<HashMap<String, String>>,
}

#[derive(Parser)]
#[grammar = "./parsers/bags.pest"]
struct BagParser {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_parse_bags_and_what_they_contain() {
        let snippet = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let mut bright_white = HashMap::new();
        bright_white.insert(String::from("color"), "bright white".to_string());
        bright_white.insert("quantity".to_string(), "1".to_string());

        let mut muted_yellow = HashMap::new();
        muted_yellow.insert("color".to_string(), "muted yellow".to_string());
        muted_yellow.insert("quantity".to_string(), "2".to_string());

        let expected = Bag {
            color: "light red".to_string(),
            contains: vec![bright_white, muted_yellow],
        };

        assert_eq!(parse_single_bag_data(snippet), expected)
    }

    #[test]
    fn it_can_parse_many_bags() {
        let snippet = "light red bags contain 1 bright white bag, 2 muted yellow bags.
bright white bags contain 1 shiny gold bag.
faded blue bags contain no other bags.";

        let mut bright_white = HashMap::new();
        bright_white.insert(String::from("color"), "bright white".to_string());
        bright_white.insert("quantity".to_string(), "1".to_string());

        let mut muted_yellow = HashMap::new();
        muted_yellow.insert("color".to_string(), "muted yellow".to_string());
        muted_yellow.insert("quantity".to_string(), "2".to_string());

        let bag_1 = Bag {
            color: "light red".to_string(),
            contains: vec![bright_white, muted_yellow],
        };

        let mut shiny_gold = HashMap::new();
        shiny_gold.insert(String::from("color"), "shiny gold".to_string());
        shiny_gold.insert("quantity".to_string(), "1".to_string());

        let bag_2 = Bag {
            color: "bright white".to_string(),
            contains: vec![shiny_gold],
        };

        let bag_3 = Bag {
            color: "faded blue".to_string(),
            contains: Vec::new(),
        };

        let expected = vec![bag_1, bag_2, bag_3];
        let actual = parse_many_bags_data(snippet);
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_return_all_direct_bags_that_can_contain_that_color() {
        let snippet = "light red bags contain 1 bright white bag, 2 muted yellow bags.
bright white bags contain 1 shiny gold bag.
faded blue bags contain 1 bright white bag.";

        let mut bright_white = HashMap::new();
        bright_white.insert(String::from("color"), "bright white".to_string());
        bright_white.insert("quantity".to_string(), "1".to_string());

        let mut bright_white_2 = HashMap::new();
        bright_white_2.insert(String::from("color"), "bright white".to_string());
        bright_white_2.insert("quantity".to_string(), "1".to_string());

        let mut muted_yellow = HashMap::new();
        muted_yellow.insert("color".to_string(), "muted yellow".to_string());
        muted_yellow.insert("quantity".to_string(), "2".to_string());

        let bag_1 = Bag {
            color: "light red".to_string(),
            contains: vec![bright_white, muted_yellow],
        };

        let mut shiny_gold = HashMap::new();
        shiny_gold.insert(String::from("color"), "shiny gold".to_string());
        shiny_gold.insert("quantity".to_string(), "1".to_string());

        let bag_2 = Bag {
            color: "bright white".to_string(),
            contains: vec![shiny_gold],
        };

        let bag_3 = Bag {
            color: "faded blue".to_string(),
            contains: vec![bright_white_2],
        };

        let expected = vec![&bag_1, &bag_3];
        let all_bags = parse_many_bags_data(snippet);
        let actual = find_direct_bag_holders_for_a_bag("bright white", &all_bags);
        assert_eq!(actual, expected);
        assert_eq!(
            find_direct_bag_holders_for_a_bag("shiny gold", &all_bags),
            vec![&bag_2]
        );
        let no_bag: Vec<&Bag> = Vec::new();
        assert_eq!(
            find_direct_bag_holders_for_a_bag("unreal", &all_bags),
            no_bag
        );
    }

    #[test]
    fn should_return_all_bags_that_can_contain_that_color() {
        let snippet = "light red bags contain 1 bright white bag, 2 muted yellow bags.
bright white bags contain 1 shiny gold bag.
faded blue bags contain 1 bright white bag.";

        let mut bright_white = HashMap::new();
        bright_white.insert(String::from("color"), "bright white".to_string());
        bright_white.insert("quantity".to_string(), "1".to_string());

        let mut bright_white_2 = HashMap::new();
        bright_white_2.insert(String::from("color"), "bright white".to_string());
        bright_white_2.insert("quantity".to_string(), "1".to_string());

        let mut muted_yellow = HashMap::new();
        muted_yellow.insert("color".to_string(), "muted yellow".to_string());
        muted_yellow.insert("quantity".to_string(), "2".to_string());

        let bag_1 = Bag {
            color: "light red".to_string(),
            contains: vec![bright_white, muted_yellow],
        };

        let mut shiny_gold = HashMap::new();
        shiny_gold.insert(String::from("color"), "shiny gold".to_string());
        shiny_gold.insert("quantity".to_string(), "1".to_string());

        let bag_2 = Bag {
            color: "bright white".to_string(),
            contains: vec![shiny_gold],
        };

        let bag_3 = Bag {
            color: "faded blue".to_string(),
            contains: vec![bright_white_2],
        };

        let expected = vec![&bag_2, &bag_3, &bag_1];
        let all_bags = parse_many_bags_data(snippet);
        let actual = find_possible_bag_holders_for_a_bag("shiny gold", &all_bags);
        assert_eq!(actual, expected);
    }
}
