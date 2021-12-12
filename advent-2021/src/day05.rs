use std::collections::HashMap;

pub fn day05_1_fn(input: &str) -> usize {
    // Now we have all the points found on all lines
    // And we want to count up Points that occur more than once
    // Recommended is HashMap which is similar to how Counter works in Python
    // https://users.rust-lang.org/t/frequency-of-an-element-in-the-vector/43103/6

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
}

pub fn day05_2_fn(input: &str) -> usize {
    0
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

    fn get_all_points(&self) -> Vec<Point> {
        let mut points = vec![];
        // when they are the same point?
        if self.point_a == self.point_b {
            // could probably be better??
            points.push(self.point_a);
            return points;
        }

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
    fn first_test() {
        assert_eq!(day05_1_fn(INPUT), 5)
    }

    #[test]
    fn should_find_all_points_between_two_points() {
        let input = "0,9 -> 5,9";
        let actual = Line::new(input);
        assert_eq!(actual.point_a.x, 0);
        assert_eq!(actual.point_a.y, 9);
        assert_eq!(actual.point_b.x, 5);
        assert_eq!(actual.point_b.y, 9);

        assert_eq!(actual.get_all_points().len(), 6);
    }

    #[test]
    fn should_find_all_points_between_two_reverse_points() {
        let reverse_points_line = "9,4 -> 3,4";
        let another_actual = Line::new(reverse_points_line);
        assert_eq!(another_actual.get_all_points().len(), 7);
    }

    #[test]
    fn should_find_one_point_between_the_same_points() {
        let input = "9,4 -> 9,4"; // I think by definition this isn't a line??
        let actual = Line::new(input);
        assert_eq!(actual.get_all_points().len(), 1);
    }
}
