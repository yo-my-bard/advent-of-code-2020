use std::cmp::max;
#[cfg(test)]
const TEST_INPUT: &str = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;

fn main() {
    let puzzle = include_str!("day02.txt")
        .split(|c| c == '\n')
        .map(parse_game)
        .filter(CubeTracker::should_count)
        .fold(0, |mut acc, cube| {
            acc += cube.game_num;
            acc
        });
    println!("Part I: {:?}", puzzle);
    // 2634 too high (condition was max >= cube_num)
    // 2078 too low (condition was max > cube_num)

    println!(
        "Part II: {:?}",
        include_str!("day02.txt")
            .split(|c| c == '\n')
            .map(parse_minimum_game)
            .map(MinCubesTracker::power)
            .sum::<usize>()
    )
}

#[derive(Default, Debug, Clone)]
struct CubeTracker {
    should_game_count: bool,
    game_num: usize,
    cube_num: usize,
    cube_color: String,
    partial_parse: String,
}

impl CubeTracker {
    fn new() -> CubeTracker {
        CubeTracker {
            should_game_count: true,
            game_num: 0,
            cube_num: 0,
            cube_color: String::new(),
            partial_parse: String::new(),
        }
    }

    fn should_count(&self) -> bool {
        println!("{:?}", self);
        self.should_game_count
    }
}

fn parse_game(l: &str) -> CubeTracker {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    l.chars().skip(5).fold(CubeTracker::new(), |mut acc, c| {
        // println!("{:?}---{:?}", c, acc);

        if !acc.should_game_count {
            return acc;
        }

        if acc.game_num == 0 && c != ':' {
            acc.partial_parse = acc.partial_parse.clone() + c.to_string().as_str();
            return acc;
        }

        if acc.game_num == 0 {
            acc.game_num = acc.partial_parse.parse::<usize>().unwrap();
            acc.partial_parse = String::new();
            return acc;
        }

        if !c.is_alphanumeric() {
            return acc;
        }

        if c.is_ascii_digit() {
            acc.partial_parse = acc.partial_parse.clone() + c.to_string().as_str();
            return acc;
        }

        if acc.cube_num == 0 {
            acc.cube_num = acc.partial_parse.parse::<usize>().unwrap();
            acc.partial_parse = c.to_string();
            return acc;
        }

        acc.partial_parse = acc.partial_parse.clone() + c.to_string().as_str();

        return match acc.partial_parse.as_str() {
            "red" | "green" | "blue" => {
                // At this point, I probably don't need to track the color anymore
                acc.cube_color = acc.partial_parse.clone();
                match acc.cube_color.as_str() {
                    "red" => {
                        acc.should_game_count = max_red >= acc.cube_num;
                    }
                    "blue" => {
                        acc.should_game_count = max_blue >= acc.cube_num;
                    }
                    "green" => {
                        acc.should_game_count = max_green >= acc.cube_num;
                    }
                    _ => {
                        panic!("YOOO??");
                    }
                }
                if acc.should_game_count {
                    acc.cube_num = 0;
                    acc.cube_color = String::new();
                    acc.partial_parse = String::new();
                }
                acc
            }
            _ => acc,
        };
    })
}

#[derive(Debug, Clone)]
struct MinCubesTracker {
    min_red: usize,
    min_green: usize,
    min_blue: usize,
    cube_num: usize,
    cube_color: String,
    partial_parse: String,
}

impl MinCubesTracker {
    fn new() -> MinCubesTracker {
        MinCubesTracker {
            min_red: 0,
            min_green: 0,
            min_blue: 0,
            cube_num: 0,
            cube_color: String::new(),
            partial_parse: String::new(),
        }
    }

    fn power(self) -> usize {
        self.min_red * self.min_green * self.min_blue
    }
}

fn parse_minimum_game(l: &str) -> MinCubesTracker {
    l.chars()
        .skip_while(|&c| c != ':')
        .skip(1)
        .fold(MinCubesTracker::new(), |mut acc, c| {
            // println!("{:?}---{:?}", c, acc);

            if !c.is_alphanumeric() {
                return acc;
            }

            if c.is_ascii_digit() {
                acc.partial_parse = acc.partial_parse.clone() + c.to_string().as_str();
                return acc;
            }

            if acc.cube_num == 0 {
                acc.cube_num = acc.partial_parse.parse::<usize>().unwrap();
                acc.partial_parse = c.to_string();
                return acc;
            }

            acc.partial_parse = acc.partial_parse.clone() + c.to_string().as_str();

            return match acc.partial_parse.as_str() {
                "red" | "green" | "blue" => {
                    // At this point, I probably don't need to track the color anymore
                    acc.cube_color = acc.partial_parse.clone();
                    match acc.cube_color.as_str() {
                        "red" => {
                            acc.min_red = max(acc.min_red, acc.cube_num);
                        }
                        "blue" => {
                            acc.min_blue = max(acc.min_blue, acc.cube_num);
                        }
                        "green" => {
                            acc.min_green = max(acc.min_green, acc.cube_num);
                        }
                        _ => {
                            panic!("YOOO??");
                        }
                    }

                    acc.cube_num = 0;
                    acc.cube_color = String::new();
                    acc.partial_parse = String::new();

                    acc
                }
                _ => acc,
            };
        })
}

#[cfg(test)]
mod tests {
    use crate::{parse_game, parse_minimum_game, CubeTracker, MinCubesTracker, TEST_INPUT};

    #[test]
    fn test_part_a_input() {
        let result = TEST_INPUT
            .split(|c| c == '\n')
            .map(parse_game)
            .filter(CubeTracker::should_count)
            .fold(0, |mut acc, cube| {
                acc += cube.game_num;
                acc
            });
        assert_eq!(result, 8);
    }

    #[test]
    fn test_failed_parsing_this_one_from_real_input() {
        // The failing edge case is that fold actually is always one char behind (the char wasn't taken into account for partial_parse in the previous implementation)
        // so when I match on color, it ends up being a partial word like `gree`. It allows a case like this through when it should have failed.
        let result = "Game 27: 4 green, 13 blue, 2 red; 2 red, 7 green, 10 blue; 14 blue, 11 green, 1 red; 10 blue, 15 green"
            .split(|c| c == '\n')
            .map(parse_game)
            .filter(CubeTracker::should_count)
            .fold(0, |mut acc, cube| {
                acc += cube.game_num;
                acc
            });
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part_b() {
        let result: usize = TEST_INPUT
            .split(|c| c == '\n')
            .map(parse_minimum_game)
            .map(MinCubesTracker::power)
            .sum();
        assert_eq!(result, 2286);
    }
}
