pub fn day01_1_function(input: &str) -> usize {
    do_it(input)
}

pub fn day01_2_function(input: &str) -> usize {
    skip_to_my_lou(input)
}

fn do_it(input: &str) -> usize {
    let counter = Counter::default();
    input
        .split_whitespace()
        .fold(counter, |mut state, next| {
            state.previous.map(|prev| {
                if let Ok(num) = prev.parse::<i32>() {
                    if let Ok(next_num) = next.parse::<i32>() {
                        if next_num > num {
                            state.increase += 1
                        }
                    }
                }
            });
            state.previous = Some(next.to_owned());
            state
        })
        .increase
}

fn skip_to_my_lou(input: &str) -> usize {
    let mut counter = Counter::default();
    // Read the standard library implementation for an unstable feature... looks good to me!
    // https://doc.rust-lang.org/src/core/iter/traits/iterator.rs.html#3144-3148
    let mut iter_1 = input.split_whitespace();
    let mut iter_2 = input.split_whitespace().skip(3);
    loop {
        let val_1 = match iter_1.next() {
            None => break,
            Some(val) => val,
        };

        let val_2 = match iter_2.next() {
            None => break,
            Some(val) => val,
        };

        if let Ok(val_1_int) = val_1.parse::<i32>() {
            if let Ok(val_2_int) = val_2.parse::<i32>() {
                if val_2_int > val_1_int {
                    counter.increase += 1
                }
            }
        }
    }
    counter.increase
}

#[derive(Default)]
struct Counter {
    increase: usize,
    previous: Option<String>,
}

impl Counter {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_number_of_increased_values() {
        let input = "199\n\
                    200\n\
                    208\n\
                    210\n\
                    200\n\
                    207\n\
                    240\n\
                    269\n\
                    260\n\
                    263";

        assert_eq!(day01_1_function(input), 7);
    }

    #[test]
    fn should_get_increased_values_for_two_numbers() {
        let input = "199
200";
        assert_eq!(do_it(input), 1);
    }

    #[test]
    fn should_get_number_of_increased_sums_of_values() {
        let input = "199
200
208
210
200
207
240
269
260
263";

        assert_eq!(day01_2_function(input), 5);
    }
}
