/*
TODO:
Optimize both day 1 solutions

 */
pub fn day01_2_function(input: &str) -> i32{
    let mut result: i32 = 0;
    let v: Vec<&str> = input.split("\n").collect();
    let mut v2 = Vec::new();
    for item in v {
        v2.push(item.trim().parse::<i32>().unwrap());
    }
    let v3 = v2.clone();
    let v4 = v2.clone();
    let mut sum: i32 = 0;
    for item in v2 {
        if sum == 2020 {
            break
        }
        for item2 in v3.clone() {
            if sum == 2020 {
                break
            }
            for item3 in v4.clone() {
                sum = item + item2 + item3;
                if sum == 2020 {
                    result = item * item2 * item3;
                    break
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::day01_2_function;

    #[test]
    fn it_should_find_three_entries_that_add_to_2020() {
        let snippet = "1721\n979\n366\n299\n675\n1456";
        assert_eq!(day01_2_function(snippet), 241861950);
    }
}