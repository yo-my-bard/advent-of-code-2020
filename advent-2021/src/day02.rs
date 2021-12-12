pub fn day02_1_fn(input: &str) -> usize {
    let mut compass = Compass::default();
    input
        .split(|c| (c == '\n') || (c == '\r'))
        .filter(|s| !s.is_empty())
        .for_each(|item| {
            let distance = item.split_whitespace().last().map_or_else(
                || 0,
                |last| last.parse::<usize>().map_or_else(|_| 0, |ok| ok),
            );
            match Compass::navigate(item.to_string()) {
                Direction::FORWARD => {
                    compass.go_forward(distance);
                }
                Direction::DOWN => {
                    compass.go_deeper(distance);
                }
                Direction::UP => {
                    compass.surface(distance);
                }
                Direction::LESS => {}
            }
        });
    compass.x_position * compass.depth
}

pub fn day02_2_fn(input: &str) -> usize {
    let mut compass = Compass::default();
    input
        .split(|c| (c == '\n') || (c == '\r'))
        .filter(|s| !s.is_empty())
        .for_each(|item| {
            let distance = item.split_whitespace().last().map_or_else(
                || 0,
                |last| last.parse::<usize>().map_or_else(|_| 0, |ok| ok),
            );
            match Compass::navigate(item.to_string()) {
                Direction::FORWARD => {
                    compass.go_forward_and_deeper(distance);
                }
                Direction::DOWN => {
                    compass.aim_higher(distance);
                }
                Direction::UP => {
                    compass.aim_lower(distance);
                }
                Direction::LESS => {}
            }
        });
    compass.x_position * compass.depth
}

enum Direction {
    FORWARD,
    DOWN,
    UP,
    LESS,
}

#[derive(Default)]
struct Compass {
    pub x_position: usize,
    pub depth: usize,
    pub aim: usize,
}

trait ILikeTheWayYouMoveItMoveIt {
    fn go_forward(&mut self, distance: usize) -> usize;
    fn go_deeper(&mut self, distance: usize) -> usize;
    fn surface(&mut self, distance: usize) -> usize;
    fn aim_higher(&mut self, distance: usize) -> usize;
    fn aim_lower(&mut self, distance: usize) -> usize;
    fn go_forward_and_deeper(&mut self, distance: usize) -> usize;
    fn navigate(input: String) -> Direction;
}

impl ILikeTheWayYouMoveItMoveIt for Compass {
    fn go_forward(&mut self, distance: usize) -> usize {
        self.x_position += distance;
        self.x_position
    }

    fn go_deeper(&mut self, distance: usize) -> usize {
        self.depth += distance;
        self.depth
    }

    fn surface(&mut self, distance: usize) -> usize {
        self.depth -= distance;
        self.depth
    }

    fn aim_higher(&mut self, distance: usize) -> usize {
        self.aim += distance;
        self.aim
    }

    fn aim_lower(&mut self, distance: usize) -> usize {
        self.aim -= distance;
        self.aim
    }

    fn go_forward_and_deeper(&mut self, distance: usize) -> usize {
        self.go_forward(distance);
        self.depth += self.aim * distance;
        self.depth
    }

    fn navigate(input: String) -> Direction {
        if input.contains("forward") {
            Direction::FORWARD
        } else if input.contains("down") {
            Direction::DOWN
        } else if input.contains("up") {
            Direction::UP
        } else {
            Direction::LESS
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_move_according_to_directions() {
        let input = "\r\n\
        forward 5\n\
        down 5\n\
        forward 8\n\
        up 3\n\
        down 8\n\
        forward 2";
        assert_eq!(day02_1_fn(input), 150)
    }

    #[test]
    fn should_aim_correctly() {
        let input = "\r\n\
        forward 5\n\
        down 5\n\
        forward 8\n\
        up 3\n\
        down 8\n\
        forward 2";
        assert_eq!(day02_2_fn(input), 900)
    }
}
