/*
TODO:
- Learn more about `as` for conversion/casting between usize to i32, for example.
- Read about Rust vectors (Vec)
- Read about the ? operator
- Practice reading from a file -- got several path/directory issues (no such file errors)
 */

pub fn day01_1_function(input: &str) -> i32 {
    let mut done: bool = false;
    let mut multiple_1: i32 = 0;
    let mut multiple_2: i32 = 0;
    let v: Vec<&str> = input.split("\n").collect();
    for num in v.clone() {
        multiple_1 = str::parse(num.trim()).unwrap();
        for other_num in v.clone() {
            multiple_2 = str::parse(other_num.trim()).unwrap();
            if multiple_1 + multiple_2 == 2020 {
                done = true;
                break;
            }
        }
        if done {
            break;
        }
    }
    multiple_1 * multiple_2
}

#[cfg(test)]
mod tests {
    use super::day01_1_function;

    #[test]
    fn it_should_find_two_entries_that_add_to_2020() {
        let snippet = "1721\n979\n366\n299\n675\n1456";
        assert_eq!(day01_1_function(snippet), 514579);
    }
}
