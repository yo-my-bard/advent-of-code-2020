/*
Our goal: count the number of trees touched on the way down the slope
We think one way we might be able to do this is to find a way to rep the data
We think one way to do so is a 2d array
 */
/*
TODO:
- Learn about str to char conversions???
- Learn, once again, what is str *really*?
 */


pub fn day03_1_function(input: &str, slope: Slope) -> usize {
    let tree = '#';
    find_map_spots_visited(transform_to_2d(input), slope.right, slope.down)
        .iter()
        // Double reference https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter
        .filter(|&&spot| spot == tree)
        .count()
}

pub fn day03_2_function(input: &str, slopes: Vec<Slope>) -> usize {
    let mut multi = 1;
    for slope in slopes {
        multi *= day03_1_function(input, slope)
    }
    multi
}

fn transform_to_2d(input: &str) -> Vec<Vec<char>> {
    let vec: Vec<&str> = input.split("\n").collect();
    let mut return_vec: Vec<Vec<char>> = Vec::new();
    for v in vec {
        return_vec.push(v.trim().chars().collect())
    }

    return_vec
}

fn find_map_spots_visited(ndarray: Vec<Vec<char>>,
                          right: usize,
                          down: usize) -> Vec<char> {
    if ndarray.is_empty() {
        return Vec::new();
    }
    // Subtract 1 because starts in the top left corner
    let max_down_visits = (ndarray.len() - 1) / down;
    let mut map_spots: Vec<char> = Vec::new();

    for i in 0..max_down_visits {
        let map_spot_down = (i + 1) * down;
        let map_spot_right = (right * (i + 1)) % ndarray[0].len();
        map_spots.push(ndarray[map_spot_down][map_spot_right]);
    }
    map_spots
}

pub struct Slope {
    pub right: usize,
    pub down: usize
}

pub fn get_slopes() -> Vec<Slope> {
    vec![Slope{right: 1, down: 1},
         Slope{right: 3, down: 1},
         Slope{right: 5, down: 1},
         Slope{right: 7 , down: 1},
         Slope{right: 1, down: 2}]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SNIPPET: &str = "..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#";

    #[test]
    fn it_should_count_the_number_of_trees_encountered() {
        let result = day03_1_function(SNIPPET, Slope {right: 3, down: 1});
        assert_eq!(result, 7);
        assert_eq!(day03_1_function(SNIPPET, Slope {right: 1, down: 1}), 2);
        assert_eq!(day03_1_function(SNIPPET, Slope {right: 5, down: 1}), 3);
        assert_eq!(day03_1_function(SNIPPET, Slope {right: 7, down: 1}), 4);
        assert_eq!(day03_1_function(SNIPPET, Slope {right: 1, down: 2}), 2);
    }

    #[test]
    fn it_should_create_2d_vector() {
        let snippet = "..##.......
        #...#...#..
        .#....#..#.";
        let result = transform_to_2d(snippet);
        let expected: Vec<Vec<char>> = vec![vec!['.', '.', '#', '#', '.', '.', '.', '.', '.', '.', '.'],
            vec!['#', '.', '.', '.', '#', '.', '.', '.', '#', '.', '.'],
            vec!['.', '#', '.', '.', '.', '.', '#', '.', '.', '#', '.']
        ];
        assert_eq!(result, expected);
        assert_eq!(result[0][0], '.');
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn it_should_return_all_the_map_spots_visited() {
        let snippet = "..##.......
        #...#...#..
        .#....#..#.";
        let array2d = transform_to_2d(snippet);
        let slope = Slope {right: 3, down: 1};
        let result = find_map_spots_visited(array2d, slope.right, slope.down);
        let expected: Vec<char> = vec!['.', '#'];
        assert_eq!(result, expected)
    }

    #[test]
    fn it_should_multiply_the_number_of_trees_encountered_for_all_given_slopes() {
        assert_eq!(day03_2_function(SNIPPET, get_slopes()), 336);
    }
}