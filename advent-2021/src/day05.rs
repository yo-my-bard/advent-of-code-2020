use std::collections::{HashMap, HashSet};

pub fn day05_1_fn(input: &str) -> usize {
    // Now we have all the points found on all lines
    // And we want to count up Points that occur more than once
    // Recommended is HashMap which is similar to how Counter works in Python
    // https://users.rust-lang.org/t/frequency-of-an-element-in-the-vector/43103/6

    input
        .split(|c| c == '\n' || c == '\r')
        .filter(|n| !n.is_empty())
        .map(Line::new)
        .flat_map(|line| line.get_all_horizontal_vertical_points())
        .fold(HashMap::<Point, usize>::new(), |mut acc, val| {
            let value_count = acc.entry(val).or_insert(0);
            *value_count += 1;
            acc
        })
        .iter()
        .filter(|(_, count)| **count > 1)
        .count()
}

pub fn day05_2_fn(input: &str) -> usize {
    input
        .split(|c| c == '\n' || c == '\r')
        .filter(|n| !n.is_empty())
        .map(Line::new)
        .flat_map(|line| line.get_all_points())
        .fold(HashMap::<Point, usize>::new(), |mut acc, val| {
            let value_count = acc.entry(val).or_insert(0);
            *value_count += 1;
            acc
        })
        .iter()
        .filter(|(_, count)| **count > 1)
        .count()
    // dbg!(f);
    // 2,2 has 3; 7,4 has 3; 6,4 has 2; *1,1* shouldn't exist, 5,5 has 3; 6,6 shouldn't exist, 3,3 shouldn't, 7,7 shouldn't
    // 12
}

#[derive(Debug)]
struct Line {
    point_a: Point,
    point_b: Point,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i16,
    y: i16,
}

impl Line {
    fn new(input: &str) -> Self {
        let points: Vec<Point> = input.split("->").map(Point::new).collect();
        Line {
            point_a: points[0],
            point_b: points[1],
        }
    }

    fn get_all_horizontal_vertical_points(&self) -> Vec<Point> {
        let mut points = vec![];
        // when they are the same point?
        if self.point_a == self.point_b {
            // could probably be better??
            points.push(self.point_a);
            return points;
        }

        // vertical points
        if self.point_a.x == self.point_b.x {
            if self.point_a.y > self.point_b.y {
                for i in self.point_b.y..=self.point_a.y {
                    points.push(self.point_a.from_point_with_new_y(i));
                }
            } else {
                for i in self.point_a.y..=self.point_b.y {
                    points.push(self.point_a.from_point_with_new_y(i));
                }
            }
        }

        // horizontal points
        if self.point_a.y == self.point_b.y {
            if self.point_a.x > self.point_b.x {
                for i in self.point_b.x..=self.point_a.x {
                    points.push(self.point_a.from_point_with_new_x(i));
                }
            } else {
                for i in self.point_a.x..=self.point_b.x {
                    points.push(self.point_a.from_point_with_new_x(i));
                }
            }
        }

        points
    }

    fn get_all_points(&self) -> Vec<Point> {
        let mut points = vec![];
        // when they are the same point?
        if self.point_a == self.point_b {
            // could probably be better??
            points.push(self.point_a);
            return points;
        }

        // https://stackoverflow.com/a/47648303
        let mut uniques = HashSet::<Point>::new();
        points.push(self.point_a);
        points.push(self.point_b);

        // vertical points
        if self.point_a.x == self.point_b.x {
            if self.point_a.y > self.point_b.y {
                for i in self.point_b.y..=self.point_a.y {
                    points.push(self.point_a.from_point_with_new_y(i));
                }
            } else {
                for i in self.point_a.y..=self.point_b.y {
                    points.push(self.point_a.from_point_with_new_y(i));
                }
            }
        }

        // horizontal points
        if self.point_a.y == self.point_b.y {
            if self.point_a.x > self.point_b.x {
                for i in self.point_b.x..=self.point_a.x {
                    points.push(self.point_a.from_point_with_new_x(i));
                }
            } else {
                for i in self.point_a.x..=self.point_b.x {
                    points.push(self.point_a.from_point_with_new_x(i));
                }
            }
        }

        // diagonal points
        /*
         Since it's 45 degrees, assuming slope is always 1. but have to limit it
        to points in between the two only.

        Compare x's and y's, converge toward the points.
        if x is larger, add 1. if y is smaller, subtract 1.
        converging on x's, will be the same number of moves you have to make on y.
        */

        points.extend(get_diagonal_points(&self.point_a, &self.point_b));
        points.retain(|p| uniques.insert(*p));

        points
    }
}

impl Point {
    fn new(input: &str) -> Self {
        let points: Vec<i16> = input
            .trim()
            .split(',')
            .map(|num| num.parse::<i16>().unwrap())
            .collect();

        Point {
            x: points[0],
            y: points[1],
        }
    }

    fn from_point_with_new_x(&self, new_x: i16) -> Point {
        Point {
            x: new_x,
            y: self.y,
        }
    }

    fn from_point_with_new_y(&self, new_y: i16) -> Point {
        Point {
            x: self.x,
            y: new_y,
        }
    }

    fn from_x_and_y(x_y: (i16, i16)) -> Point {
        Point { x: x_y.0, y: x_y.1 }
    }
}

fn get_diagonal_points(point_a: &Point, point_b: &Point) -> Vec<Point> {
    // need to subtract point_a's x by 1 until point_b's x, exclusive
    // have to worry about downward slope, vs up and right slope
    // down and right slope x,y increase together
    // up and right slope x,y increase in reverse

    let x_range = if point_b.x > point_a.x {
        (point_a.x + 1)..point_b.x
    } else {
        (point_b.x + 1)..point_a.x
    };

    let y_range = if point_b.y > point_a.y {
        (point_a.y + 1)..point_b.y
    } else {
        (point_b.y + 1)..point_a.y
    };

    // up and to right, needs reverse
    if (point_b.x > point_a.x && point_b.y < point_a.y)
        || (point_b.x < point_a.x && point_b.y > point_a.y)
    {
        x_range
            .zip(y_range.rev())
            .map(Point::from_x_and_y)
            .collect::<Vec<Point>>()
    } else {
        x_range
            .zip(y_range)
            .map(Point::from_x_and_y)
            .collect::<Vec<Point>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "0,9 -> 5,9\n\
    8,0 -> 0,8\n\
    9,4 -> 3,4\n\
    2,2 -> 2,1\n\
    7,0 -> 7,4\n\
    6,4 -> 2,0\n\
    0,9 -> 2,9\n\
    3,4 -> 1,4\n\
    0,0 -> 8,8\n\
    5,5 -> 8,2";

    #[test]
    fn should_reproduce_day05_1_test() {
        assert_eq!(day05_1_fn(INPUT), 5)
    }

    #[test]
    fn should_reproduce_day05_2_test() {
        assert_eq!(day05_2_fn(INPUT), 12)
    }

    #[test]
    fn should_find_all_points_between_two_points() {
        let input = "0,9 -> 5,9";
        let actual = Line::new(input);
        assert_eq!(actual.point_a.x, 0);
        assert_eq!(actual.point_a.y, 9);
        assert_eq!(actual.point_b.x, 5);
        assert_eq!(actual.point_b.y, 9);

        assert_eq!(actual.get_all_horizontal_vertical_points().len(), 6);
    }

    #[test]
    fn should_find_all_points_between_two_reverse_points() {
        let reverse_points_line = "9,4 -> 3,4";
        let another_actual = Line::new(reverse_points_line);
        assert_eq!(another_actual.get_all_horizontal_vertical_points().len(), 7);
    }

    #[test]
    fn should_find_one_point_between_the_same_points() {
        let input = "9,4 -> 9,4"; // I think by definition this isn't a line??
        let actual = Line::new(input);
        assert_eq!(actual.get_all_horizontal_vertical_points().len(), 1);
    }

    #[test]
    fn should_get_diagonal_lines_simple() {
        let pa = Point { x: 9, y: 7 };
        let pb = Point { x: 7, y: 9 };
        let diagonals = get_diagonal_points(&pa, &pb);
        let expected = vec![Point { x: 8, y: 8 }];
        assert_eq!(diagonals, expected)
    }

    #[test]
    fn should_get_diagonal_lines_again() {
        let pa = Point { x: 1, y: 1 };
        let pb = Point { x: 3, y: 3 };
        let diagonals = get_diagonal_points(&pa, &pb);
        let expected = vec![Point { x: 2, y: 2 }];
        assert_eq!(diagonals, expected)
    }

    #[test]
    fn should_get_diagonal_lines_edge_case_1() {
        // up and to the left
        let pa = Point { x: 8, y: 0 };
        let pb = Point { x: 0, y: 8 };
        let diagonals = get_diagonal_points(&pa, &pb);
        let expected = vec![
            Point { x: 1, y: 7 },
            Point { x: 2, y: 6 },
            Point { x: 3, y: 5 },
            Point { x: 4, y: 4 },
            Point { x: 5, y: 3 },
            Point { x: 6, y: 2 },
            Point { x: 7, y: 1 },
        ];
        assert_eq!(diagonals, expected)
    }

    #[test]
    fn should_get_diagonal_lines_edge_case_2() {
        // down and to the right
        let pa = Point { x: 0, y: 0 };
        let pb = Point { x: 8, y: 8 };
        let diagonals = get_diagonal_points(&pa, &pb);
        let expected = vec![
            Point { x: 1, y: 1 },
            Point { x: 2, y: 2 },
            Point { x: 3, y: 3 },
            Point { x: 4, y: 4 },
            Point { x: 5, y: 5 },
            Point { x: 6, y: 6 },
            Point { x: 7, y: 7 },
        ];
        assert_eq!(diagonals, expected)
    }

    #[test]
    fn should_get_diagonal_lines_edge_case_3() {
        // PA is to right of PB, and PB is below
        // down and to the right
        let pa = Point { x: 6, y: 4 };
        let pb = Point { x: 2, y: 0 };
        let diagonals = get_diagonal_points(&pa, &pb);
        let expected = vec![
            Point { x: 3, y: 1 },
            Point { x: 4, y: 2 },
            Point { x: 5, y: 3 },
        ];
        assert_eq!(diagonals, expected)
    }
}
