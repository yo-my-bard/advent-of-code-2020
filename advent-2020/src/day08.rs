use std::collections::HashSet;
use std::convert::TryInto;
use std::ops::Index;

pub fn day08_1(input: &str) -> i32 {
    return_accumulator(input)
}

pub fn day08_2(input: &str) -> i32 {
    0
}

fn split_operation_argument(input: &str) -> [String; 2] {
    match input
        .trim()
        .split_whitespace()
        .map(str::to_owned)
        .collect::<Vec<_>>()
        .try_into()
    {
        Ok(arr) => arr,
        Err(_) => [String::new(), String::new()],
    }
}

fn create_all_operations(input: &str) -> Vec<[String; 2]> {
    input.split('\n').map(split_operation_argument).collect()
}

// We probably want to store the index, and what to know when we've added an index that we've already added previously
// It sounds like the use case for a set, but is there a data structure that tells you when you've added a duplicate?
// HashSet.insert can do this; returns true if did not have value; false if it did

fn return_accumulator(input: &str) -> i32 {
    let mut acc = 0;
    let mut idx: usize = 0;
    let mut set: HashSet<usize> = HashSet::new();
    let instructions = create_all_operations(input);
    // We don't need to iterate everything, we just need to give it a start until we get false
    while set.insert(idx) && idx < instructions.len() {
        let [operation, argument] = instructions.index(idx);
        match operation.as_str() {
            "acc" => {
                acc += argument
                    .parse::<i32>()
                    .expect("Argument should be parseable as a signed integer");
                idx += 1;
            }
            "jmp" => {
                let movement = argument
                    .parse::<i32>()
                    .expect("Argument should be parseable as a signed integer");

                if movement.is_negative() {
                    idx -= movement.abs() as usize
                } else {
                    idx += movement.abs() as usize;
                }
            }
            "nop" => {
                idx += 1;
            }
            _ => {
                panic!("Shouldn't have made it here");
            }
        }
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_program_execution() {
        /*
        acc increases or decreases a single global value called the accumulator by the value given in the argument
        jmp jumps to a new instruction relative to itself. The next instruction to execute is found using the argument as an offset from the jmp instruction
        nop stands for No OPeration; on to the next

        Immediately before any instruction is executed a second time, what value is in the accumulator?
        */
        let input = r"nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6";

        let actual = return_accumulator(input);
        assert_eq!(actual, 5);
    }

    #[test]
    fn test_operation_arg_split() {
        let input = "nop +0";
        let expected = [String::from("nop"), String::from("+0")];
        assert_eq!(split_operation_argument(input), expected)
    }

    #[test]
    fn test_create_all_operations() {
        let input = r"nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6";

        let expected = vec![
            [String::from("nop"), "+0".to_string()],
            ["acc".to_string(), "+1".to_string()],
            ["jmp".to_string(), "+4".to_string()],
            ["acc".to_string(), "+3".to_string()],
            ["jmp".to_string(), "-3".to_string()],
            ["acc".to_string(), "-99".to_string()],
            ["acc".to_string(), "+1".to_string()],
            ["jmp".to_string(), "-4".to_string()],
            ["acc".to_string(), "+6".to_string()],
        ];

        let actual = create_all_operations(input);

        assert_eq!(actual.len(), 9);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parsing_positive_negative_to_int() {
        let input = "-99";
        let parsed = input.parse::<i32>().unwrap();
        assert_eq!(parsed, -99);
        assert_eq!("+99".parse::<i32>().unwrap(), 99);
    }
}
