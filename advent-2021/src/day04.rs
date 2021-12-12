use std::time::Instant;

/*
Designing a Pub-Sub for this...
Since bingo boards are the same, we can probably represent them using a vector
We need to store all the winning combination vectors (the rows and columns indexes that will result in wins)
10 in total, 5 rows, 5 columns


This kinda helped? https://stackoverflow.com/a/48492289

*/

pub fn day04_1_fn(input: &str) -> usize {
    let first_line = input
        .split_whitespace()
        .next()
        .expect("unexpected input format");
    let drawn_numbers = NumbersDraw::new(first_line);
    let mut boards: Vec<Board> = collect_all_boards(input).iter().map(Board::new).collect();
    let mut last_call = 0;
    drawn_numbers
        .numbers
        .iter()
        .map(|drawn_num| {
            last_call = drawn_num
                .parse::<i32>()
                .expect("Drawn number wasn't a number");
            boards.iter_mut().any(|brd| {
                brd.call(drawn_num);
                brd.response()
            })
        })
        .take_while(|b| !b)
        .last();

    if let Some(winner) = boards.iter().filter(|b| b.response()).last() {
        let sum_all_uncalled: i32 = winner
            .values
            .iter()
            .filter(|val| !val.drawn)
            .map(|val_values_only| {
                val_values_only
                    .value
                    .parse::<i32>()
                    .expect("Board value was not a number")
            })
            .sum();

        return (last_call * sum_all_uncalled) as usize;
    }

    0
}

pub fn day04_2_fn(input: &str) -> usize {
    let first_line = input
        .split_whitespace()
        .next()
        .expect("unexpected input format");
    let drawn_numbers = NumbersDraw::new(first_line);
    let mut boards: Vec<Board> = collect_all_boards(input).iter().map(Board::new).collect();
    let mut last_call = 0;
    drawn_numbers
        .numbers
        .iter()
        .map_while(|drawn_num| {
            last_call = drawn_num
                .parse::<i32>()
                .expect("Drawn number wasn't a number");
            boards.iter_mut().for_each(|brd| {
                brd.call(drawn_num);
                if brd.response() && brd.winning_time.is_none() {
                    brd.winning_time = Some(Instant::now());
                }
            });
            if boards
                .iter()
                .any(|predicate| predicate.winning_time.is_none())
            {
                return Some(());
            }
            None
        })
        .last();

    boards.sort_by_key(|b| b.winning_time);
    let winner = boards
        .pop()
        .expect("Should have at least one board after sorting");

    let sum_all_uncalled: i32 = winner
        .values
        .iter()
        .filter(|val| !val.drawn)
        .map(|val_values_only| {
            val_values_only
                .value
                .parse::<i32>()
                .expect("Board value was not a number")
        })
        .sum();

    (last_call * sum_all_uncalled) as usize
}

#[derive(Debug, PartialEq, Clone)]
struct NumbersDraw<'c> {
    numbers: Vec<&'c str>,
    // We tried to implement pub-sub this way, but borrow checker was v unhappy.
    subscribers: Vec<&'c Board<'c>>,
}

#[derive(Debug, PartialEq, Clone)]
struct Board<'a> {
    values: Vec<Value<'a>>,
    winning_combinations: Vec<Vec<usize>>,
    winning_time: Option<Instant>,
}

#[derive(Debug, PartialEq, Clone)]
struct Value<'a> {
    value: &'a str,
    drawn: bool,
}

impl<'a> Value<'a> {
    fn new(input: &'a str) -> Self {
        Value {
            value: input,
            drawn: false,
        }
    }
}

impl<'a> Board<'a> {
    fn new(board_input: &Vec<&'a str>) -> Self {
        Board {
            values: board_input
                .iter()
                .map(|item| item.split_whitespace())
                .flatten()
                .map(Value::new)
                .collect(),
            winning_combinations: Self::winning_combinations(),
            winning_time: None,
        }
    }

    fn winning_combinations() -> Vec<Vec<usize>> {
        vec![
            vec![0, 1, 2, 3, 4],
            vec![5, 6, 7, 8, 9],
            vec![10, 11, 12, 13, 14],
            vec![15, 16, 17, 18, 19],
            vec![20, 21, 22, 23, 24],
            vec![0, 5, 10, 15, 20],
            vec![1, 6, 11, 16, 21],
            vec![2, 7, 12, 17, 22],
            vec![3, 8, 13, 18, 23],
            vec![4, 9, 14, 19, 24],
        ]
    }

    fn call(&mut self, draw: &str) -> &Self {
        // Keeping this to remind myself of the pain I went through...
        // for (ii, vv) in self.values.iter_mut().enumerate() {
        //     if vv.value == draw {
        //         (*vv).drawn = true;
        //         for (j, combo) in self.winning_combinations.iter_mut().enumerate() {
        //             if let Some(pos) = (*combo).iter().position(|combo_val| combo_val == &ii) {
        //                 let l = (*combo).remove(pos);

        //                 println!(
        //                     "value={:?},  combo={:?}, i={:?}, pos={:?}, l={:?}",
        //                     vv, combo, ii, pos, l
        //                 );
        //             }
        //         }
        //     }
        // }
        self.values
            .iter_mut()
            .enumerate()
            .filter(|(_, val)| val.value == draw)
            .for_each(|(i, v)| {
                v.drawn = true;
                let combinations = &mut self.winning_combinations;
                for combo in combinations {
                    if let Some(pos) = combo.iter().position(|combo_val| combo_val == &i) {
                        (*combo).remove(pos);
                    }
                }
            });
        self
    }

    fn response(&self) -> bool {
        self.winning_combinations.iter().any(|v| v.is_empty())
    }
}

impl<'d> NumbersDraw<'d> {
    fn new(input: &'d str) -> Self {
        NumbersDraw {
            numbers: input.split(',').collect::<Vec<&'d str>>(),
            subscribers: vec![],
        }
    }

    fn subscribe(&mut self, board: &'d Board<'d>) {
        self.subscribers.push(board);
    }

    // fn publish(&mut self) -> Option<&mut Board<'d>> {
    //     self.subscribers
    //         .iter_mut()
    //         .find(|subscriber| subscriber.response("22"))
    // }
}

fn collect_all_boards<'a>(input: &'a str) -> Vec<Vec<&'a str>> {
    let init_vec: Vec<Vec<&str>> = vec![];
    input
        .split(|c| c == '\n' || c == '\r')
        .skip(2)
        .fold(init_vec, |mut acc, item| {
            if item.is_empty() {
                return acc;
            }
            if let Some(last) = acc.last_mut() {
                if last.len() == 5 {
                    acc.push(vec![item.trim()])
                } else {
                    last.push(item)
                }
            } else {
                acc.push(vec![item.trim()]);
            }

            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs::fetch_input_file;

    #[test]
    fn should_do_day04_1() {
        let input = &fetch_input_file("src/inputs/day04_test");
        let actual = day04_1_fn(input);
        assert_eq!(actual, 4512);
    }

    #[test]
    fn should_do_day04_2() {
        let input = &fetch_input_file("src/inputs/day04_test");
        let actual = day04_2_fn(input);
        assert_eq!(actual, 1924);
    }

    #[test]
    fn should_create_comma_separated_struct() {
        let input = "a,b,c,d";
        let expected = NumbersDraw {
            numbers: vec!["a", "b", "c", "d"],
            subscribers: vec![],
        };

        assert_eq!(NumbersDraw::new(input), expected)
    }
    #[test]
    fn should_find_the_boards_in_the_input() {
        let input = &fetch_input_file("src/inputs/day04_test");
        let actual = collect_all_boards(input);

        assert!(actual.len() == 3);
        actual
            .iter()
            .for_each(|board| assert!(board.len() == 5, "unexpected board length"))
    }

    #[test]
    fn should_create_board_with_single_array_values() {
        let input = &fetch_input_file("src/inputs/day04_test");
        let actual = collect_all_boards(input);

        let boards: Vec<Board> = actual.iter().map(Board::new).collect();

        assert_eq!(boards.len(), 3);
        boards
            .iter()
            .for_each(|board| assert_eq!(board.values.len(), 25))
    }
}
