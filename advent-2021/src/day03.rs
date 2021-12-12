use std::borrow::Borrow;
use std::iter::FromIterator;

pub fn day03_1_fn(input: &str) -> usize {
    let mut counter = Counter::new(input);
    counter.process_data();
    counter.get_gamma_rate() * counter.get_epsilon_rate()
}

pub fn day03_2_fn(input: &str) -> usize {
    let mut counter = Counter::new(input);
    counter.process_data();
    counter.get_oxygen_generator_rating() * counter.get_co2_scrubber_rating()
}

#[derive(Debug)]
struct Counter {
    raw_data: String,
    data_array: Vec<Vec<usize>>,
    columns: usize,
}

trait Day03_1 {
    fn get_gamma_rate(&self) -> usize;
    fn get_epsilon_rate(&self) -> usize;
}

trait Day03_2 {
    fn get_oxygen_generator_rating(&self) -> usize;
    fn get_co2_scrubber_rating(&self) -> usize;
}

impl Counter {
    pub fn new(input: &str) -> Self {
        Counter {
            raw_data: String::from(input),
            data_array: Self::make_data_array(input),
            columns: Self::number_of_columns(input),
        }
    }

    pub fn process_data(&mut self) {
        self.raw_data.split_whitespace().for_each(|line| {
            for x in 0..self.columns {
                self.data_array[x].push(line.get(x..=x).unwrap().parse::<usize>().unwrap())
            }
        });
    }

    fn number_of_columns(input: &str) -> usize {
        input
            .split(|c| c == '\n' || c == '\r')
            .next()
            .map_or(0, |s| s.len())
    }

    fn make_data_array(input: &str) -> Vec<Vec<usize>> {
        let mut v = Vec::<Vec<usize>>::with_capacity(Self::number_of_columns(input));
        v.resize(Self::number_of_columns(input), Vec::new());
        v
    }
}

impl Day03_1 for Counter {
    fn get_gamma_rate(&self) -> usize {
        // columns tells how length of the binary 001 == len == 3
        // We need to count each item in data array to know whether there are more 1s or 0s
        // Sum up all the values. If they are greater than half of the length, then 1, otherwise 0

        let mut gamma_binary = Vec::<&str>::with_capacity(self.columns);
        for g in 0..self.columns {
            let num_of_rows = self.data_array[g].len();
            let sum: usize = self.data_array[g].iter().sum();

            if sum > num_of_rows / 2 {
                gamma_binary.push("1");
            } else {
                gamma_binary.push("0");
            }
        }
        i32::from_str_radix(String::from_iter(gamma_binary).borrow(), 2).unwrap() as usize
    }

    fn get_epsilon_rate(&self) -> usize {
        // It sounds like we could just use gamma rate and bitwise negation,
        // but I haven't figured out how to represent that yet
        // Negating it ends up negating the leading zeroes too.
        let mut gamma_binary = Vec::<&str>::with_capacity(self.columns);
        for g in 0..self.columns {
            let num_of_rows = self.data_array[g].len();
            let sum: usize = self.data_array[g].iter().sum();

            if sum > num_of_rows / 2 {
                gamma_binary.push("0");
            } else {
                gamma_binary.push("1");
            }
        }

        i32::from_str_radix(String::from_iter(gamma_binary).borrow(), 2).unwrap() as usize
    }
}

impl Day03_2 for Counter {
    fn get_oxygen_generator_rating(&self) -> usize {
        let mut one_index_stack = Vec::<usize>::new();
        let mut zero_index_stack = Vec::<usize>::new();

        let boo_radley = (0..self.columns)
            .fold(Vec::<usize>::new(), |mut acc, col| {
                one_index_stack.clear();
                zero_index_stack.clear();

                self.data_array[col] // change column
                    .iter()
                    .enumerate()
                    .filter(|(ind, _)| acc.is_empty() || acc.contains(ind))
                    .for_each(|(i, &val)| {
                        if val == 0 {
                            zero_index_stack.push(i);
                        } else {
                            one_index_stack.push(i);
                        }
                    });

                // could change min/max logic
                acc.clear();
                if one_index_stack.len() > zero_index_stack.len() {
                    acc.append(&mut one_index_stack);
                } else if zero_index_stack.len() > one_index_stack.len() {
                    acc.append(&mut zero_index_stack);
                } else {
                    acc.append(&mut one_index_stack);
                }

                acc
            })
            .first()
            .map_or(String::new(), |&row| {
                self.data_array
                    .iter()
                    .map(|vec| vec.get(row).unwrap().to_string())
                    .reduce(|a, b| a + b.borrow())
                    .unwrap()
            });

        i32::from_str_radix(boo_radley.borrow(), 2).unwrap() as usize
    }

    fn get_co2_scrubber_rating(&self) -> usize {
        let mut one_index_stack = Vec::<usize>::new();
        let mut zero_index_stack = Vec::<usize>::new();

        let boo_radley = (0..self.columns)
            .fold(Vec::<usize>::new(), |mut acc, col| {
                if acc.len() == 1 {
                    return acc;
                }
                one_index_stack.clear();
                zero_index_stack.clear();

                self.data_array[col] // change column
                    .iter()
                    .enumerate()
                    .filter(|(ind, _)| acc.is_empty() || acc.contains(ind))
                    .for_each(|(i, &val)| {
                        if val == 0 {
                            zero_index_stack.push(i);
                        } else {
                            one_index_stack.push(i);
                        }
                    });

                // could change min/max logic
                acc.clear();
                if one_index_stack.len() < zero_index_stack.len() {
                    acc.append(&mut one_index_stack);
                } else if zero_index_stack.len() < one_index_stack.len() {
                    acc.append(&mut zero_index_stack);
                } else {
                    acc.append(&mut zero_index_stack);
                }

                acc
            })
            .first()
            .map_or(String::new(), |&row| {
                self.data_array
                    .iter()
                    .map(|vec| vec.get(row).unwrap().to_string())
                    .reduce(|a, b| a + b.borrow())
                    .unwrap()
            });

        i32::from_str_radix(boo_radley.borrow(), 2).unwrap() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_do_things() {
        let input = "00100\n\
        11110\n\
        10110\n\
        10111\n\
        10101\n\
        01111\n\
        00111\n\
        11100\n\
        10000\n\
        11001\n\
        00010\n\
        01010";

        assert_eq!(day03_1_fn(input), 198)
    }

    #[test]
    fn should_count_how_many_columns() {
        let input = "00100\n\
        11110\n\
        10110";
        assert_eq!(Counter::new(input).columns, 5)
    }

    #[test]
    fn should_process_columns_and_rows() {
        let input = "00100\n\
        11110\n\
        10110";
        let mut counter = Counter::new(input);
        counter.process_data();

        assert_eq!(counter.data_array.len(), 5);
        assert_eq!(counter.data_array.len(), counter.columns);
        counter
            .data_array
            .iter()
            .for_each(|col| assert_eq!(col.len(), 3));
    }

    #[test]
    fn should_return_gamma_rate_in_decimal() {
        let input = "00100\n\
        11110\n\
        10110\n\
        10111\n\
        10101\n\
        01111\n\
        00111\n\
        11100\n\
        10000\n\
        11001\n\
        00010\n\
        01010";
        let mut counter = Counter::new(input);
        counter.process_data();

        assert_eq!(counter.get_gamma_rate(), 22)
    }

    #[test]
    fn should_return_epsilon_rate_in_decimal() {
        let input = "00100\n\
        11110\n\
        10110\n\
        10111\n\
        10101\n\
        01111\n\
        00111\n\
        11100\n\
        10000\n\
        11001\n\
        00010\n\
        01010";
        let mut counter = Counter::new(input);
        counter.process_data();

        assert_eq!(counter.get_epsilon_rate(), 9)
    }

    #[test]
    fn should_get_oxygen_rating() {
        let input = "00100\n\
        11110\n\
        10110\n\
        10111\n\
        10101\n\
        01111\n\
        00111\n\
        11100\n\
        10000\n\
        11001\n\
        00010\n\
        01010";
        let mut counter = Counter::new(input);
        counter.process_data();

        assert_eq!(counter.get_oxygen_generator_rating(), 23)
    }

    #[test]
    fn should_get_co2_rating() {
        let input = "00100\n\
        11110\n\
        10110\n\
        10111\n\
        10101\n\
        01111\n\
        00111\n\
        11100\n\
        10000\n\
        11001\n\
        00010\n\
        01010";
        let mut counter = Counter::new(input);
        counter.process_data();

        assert_eq!(counter.get_co2_scrubber_rating(), 10)
    }
}
