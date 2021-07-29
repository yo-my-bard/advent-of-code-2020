/*
- Let's see if we can implement some custom Errors for bad inputs
- - Comes down to implementing a bunch of traits for your error type, but not necessarily required to implement Error trait: see
- - https://stackoverflow.com/questions/42584368/how-do-you-define-custom-error-types-in-rust

 So our problem *appears* to be a tree problem, or rather it appears like we should be able to solve it using
 a tree structure, based on the info we have been given so far. Idk though could be wrong.
 Let's see if we might be able to type out a representation of what we read in Advent of Code page
 BUT FIRST, a code walkthrough so far.
 I am practicing TDD (Test Driven Development) so I let the test be a guide for what I should be implementing
 I am also very new to Rust and still a baby software engineer. When I get my mic, I can talk more about that :)
 So the code so far... I have some tests, and some implementations
 */
use indextree::{Arena, NodeId};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct BoardingPass {
    row: u32,
    column: u32,
    seat_id: u32,
}

impl BoardingPass {
    const ALLOWED_FIRST_CHARS: [char; 2] = ['B', 'F'];
    const ALLOWED_SECOND_CHARS: [char; 2] = ['L', 'R'];

    fn is_valid_length(s: &str) -> bool {
        s.len() == 10
    }

    fn is_valid_chars(s: &str) -> bool {
        let valid_first_chars = s[0..7] // the first 7 characters
            .chars()
            .all(|c| Self::ALLOWED_FIRST_CHARS.contains(&c));
        let valid_second_chars = s[7..] // the rest
            .chars()
            .all(|c| Self::ALLOWED_SECOND_CHARS.contains(&c));
        valid_first_chars && valid_second_chars
    }

    fn find_the_row(s: &str) -> u32 {
        /*
        input: FBFBBFFRLR
        Rows are 0 - 127, 2^7
        That's hard math to do in my head. Can I think of a smaller number that can help, maybe 2^3?
        Rows would be 0 - 7
                      R
                  F      B
               F   B    F  B
             F B  F B  F B F B
         Gosh there's probably a website for this... I'm just going to Google haha.
         I was wrong -- the internet is a scary place.
         I'm double checking my math here, but I think I did it right?? 2^0, 2^1, 2^2, 2^3 at each level.
         guess we'll find out when I'm wrong.
         2^3 input: FBF (row 2), BFF (row 4?)
         2^1 -- F = 0-3, B = 4-7?
         2^2 -- F = 0-1, B = 2-3 (inclusive) ; F = 4-5, B = 6-7 (inclusive)
         2^3 -- the row left to right -> 0, 1, 2, 3, 4, 5, 6, 7
         OK, this is a good stopping place.
         We have a smaller example to work with (from 128 rows, to 8 rows). We think trees are still a good idea.
         Next we can write some simple tests that test this smaller tree. We also need to learn how to write a tree.
         Sneak peeking at some Rust docs.... then done with stream. Thanks for watching!
         */
        let tree_array = Self::generate_boarding_pass_tree(8);
        let mut node = Node::new(tree_array, s);
        node.traverse_tree_array();
        node.get_current_value() - node.offset
    }

    fn find_the_column(s: &str) -> u32 {
        let tree_array = Self::generate_boarding_pass_tree(4);
        let mut node = Node::new(tree_array, s);
        node.traverse_tree_array();
        node.get_current_value() - node.offset // faked it and it passed; now we need to remove duplication
    }

    /// Generates a tree using a vector, breadth-first representation
    /// Wiki: https://en.wikipedia.org/wiki/Binary_tree#Arrays
    /// Levels here is referring to the depth of a perfect binary tree (2 children all internal nodes, and leaf nodes are at the same level).
    /// https://towardsdatascience.com/5-types-of-binary-tree-with-cool-illustrations-9b335c430254
    /// For example:
    ///                       R
    ///                   F      B
    ///               F   B    F  B
    ///              F B  F B  F B F B
    /// This tree's levels = 4.
    fn generate_boarding_pass_tree(levels: u32) -> TreeArray {
        let base: u32 = 2;
        let max_nodes = base.pow(levels) - 1;
        let terminal_nodes = base.pow(levels - 1);
        // index_value_offset is the left most value of lowest value, it's also the offset to get the numerical value of the row
        let index_value_offset = max_nodes - terminal_nodes;

        let tree_array: Vec<u32> = (0..max_nodes).collect();
        TreeArray {
            array: tree_array,
            max_nodes,
            terminal_nodes,
            index_value_offset,
        }
    }
}

impl FromStr for BoardingPass {
    type Err = ParseBoardingPassError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !Self::is_valid_length(s) {
            return Err(ParseBoardingPassError {
                message: format!("invalid length. expected 10, found: {}", s.len()),
            });
        }

        if !Self::is_valid_chars(s) {
            return Err(ParseBoardingPassError {
                message: format!("invalid chars found for: {}", s),
            });
        }

        Ok(BoardingPass {
            row: Self::find_the_row(&s[..7]),
            column: Self::find_the_column(&s[7..]),
            seat_id: (Self::find_the_row(&s[..7]) * 8) + Self::find_the_column(&s[7..]),
        })
    }
}

#[derive(Debug, PartialEq)]
struct ParseBoardingPassError {
    message: String,
}

impl Display for ParseBoardingPassError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error parsing boarding pass")
    }
}

impl Error for ParseBoardingPassError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}

pub(crate) fn day05_1(input: &str) -> u32 {
    input
        .split("\r\n")
        .map(BoardingPass::from_str)
        .filter(|bp| bp.is_ok())
        .map(|bp_ok| bp_ok.unwrap())
        .max_by_key(|good_bp| good_bp.seat_id)
        .map_or(0, |final_bp| final_bp.seat_id)
}

fn scan_boarding_pass(input: &str) -> Result<BoardingPass, ParseBoardingPassError> {
    // We have code duplication here as a result, not to mention we are not actually using our input!
    BoardingPass::from_str(input)
}

struct TreeArray {
    array: Vec<u32>,
    max_nodes: u32,
    terminal_nodes: u32,
    index_value_offset: u32,
}
struct Node {
    tree_array: Vec<u32>,
    boarding_pass: String,
    current: Option<u32>,
    offset: u32,
}

impl Node {
    fn new(tree_array: TreeArray, boarding_pass: &str) -> Node {
        Node {
            tree_array: tree_array.array,
            boarding_pass: boarding_pass.to_owned(),
            current: Some(0),
            offset: tree_array.index_value_offset,
        }
    }

    fn traverse_tree_array(&mut self) {
        let mut error = false;
        for c in self.boarding_pass.chars() {
            if c == 'F' || c == 'L' {
                self.current = match self.current {
                    Some(num) => Some((2 * num) + 1),
                    None => {
                        error = true;
                        break;
                    }
                }
            } else {
                self.current = match self.current {
                    Some(num) => Some((2 * num) + 2),
                    None => {
                        error = true;
                        break;
                    }
                }
            }
        }
        if error {
            println!("Something bad is happening in Oz");
        }
    }

    fn get_current_value(&self) -> u32 {
        match self.current {
            Some(num) => num,
            None => 0,
        }
    }

    // fn is_legit_row(&self, tree_array: &TreeArray) -> bool {
    //     match self.current {
    //         Some(num) => tree_array.array.clone().len() > u32::try_usize(num).unwrap(),
    //         None => false,
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    // This would basically be the final solution -- If I can write code that does this reliably, then
    // I can feel confident that I have solved the puzzle (see? This is why tests are so cool!)
    #[test]
    fn it_should_find_the_right_row_column_and_seat() {
        let snippet = "FBFBBFFRLR"; // test input
        let another_snippet = "BBFFBBFRLL";
        let expected = BoardingPass {
            row: 44,
            column: 5,
            seat_id: 357,
        }; // representation of a boarding pass
        let another_expected = Ok(BoardingPass {
            row: 102,
            column: 4,
            seat_id: 820,
        });
        assert_eq!(BoardingPass::from_str(snippet), Ok(expected)); // if I scan the input, I should get the output
        assert_eq!(BoardingPass::from_str(another_snippet), another_expected);
    }

    // The above test is pretty close to what I need, if not 100%. The problem is it's too big of a problem.
    // I'd love to leap to the solution, but I have too much to learn before I can do that
    // So how might we be able to *break the problem down*? A very common practice in software engineering
    // What if I write code that just finds the right row first?
    #[test]
    fn it_should_find_the_right_row() {
        let snippet = "FBFBBFFRLR";
        assert_eq!(BoardingPass::find_the_row(&snippet[..7]), 44); // We deliberately write a test that fails but has what we want
    }

    // Then what if I write code that just finds the right column?
    #[test]
    fn it_should_find_the_right_column() {
        let snippet = "FBFBBFFRLR";
        assert_eq!(BoardingPass::find_the_column(&snippet[7..]), 5);
    }

    /*
    Both of those sound like great starting points -- but still I'm not LeBron in his prime or Space Jam
    Before I can even attempt to do those two things, I need to just *consume the input* and do something with it
    Well I immediately know that I can *fail fast* if the input doesn't meet length requirements
    Length seems like the easiest so let's start here
     */
    #[test]
    fn it_should_error_out_on_incorrect_length() {
        let snippet = "FBFBBFFRL";
        let expected = Err(ParseBoardingPassError {
            message: format!("invalid length. expected 10, found: {}", snippet.len()),
        });
        assert_eq!(BoardingPass::from_str(snippet), expected) // given a string (string slice for Rust) input, return error
    }

    // And I should error on incorrect characters used
    #[test]
    fn it_should_error_out_on_incorrect_characters() {
        let snippet = "FBFABFFRLR";
        let other_bad_snippet = "FBFBBFFRZR";

        let expected_1 = Err(ParseBoardingPassError {
            message: format!("invalid chars found for: {}", snippet),
        });
        let expected_2 = Err(ParseBoardingPassError {
            message: format!("invalid chars found for: {}", other_bad_snippet),
        });

        BoardingPass::from_str(other_bad_snippet).map_err(|z| println!("{:?}", z.source()));
        assert_eq!(BoardingPass::from_str(snippet), expected_1);
        assert_eq!(BoardingPass::from_str(other_bad_snippet), expected_2);
    }

    #[test]
    fn i_play_with_arena() {
        let arena = &mut Arena::new();
        let a = arena.new_node(1);
        let b = arena.new_node(2);
        let c = arena.new_node(3);
        a.append(b, arena);
        a.append(c, arena);
        a.descendants(arena).for_each(|node| println!("{:#}", node));
        b.following_siblings(arena)
            .filter(|&n| n != b)
            .for_each(|node| println!("{:?}", node));
        b.preceding_siblings(arena)
            .filter(|&n| n != b)
            .for_each(|node_id| println!("{:?}", node_id));
    }

    #[test] // The purpose of this test changed since I started, so renaming
    fn it_should_create_a_simple_node_with_a_right_and_left_child() {
        // I don't really know what I'm doing with this tree data structure, so I'm going to see if I can fake it
        // Let's create a node representation. Nodes can have zero, one, or two children
        let arena = &mut Arena::new();
        let root = arena.new_node("root");
        let left = arena.new_node("left");
        let right = arena.new_node("right");

        // No children yet
        assert!(root.children(arena).next().is_none());
        // Add one child
        root.append(left, arena);
        let mut children = root.children(arena);
        assert_eq!(children.next(), Some(left));
        assert!(children.next().is_none());
        // Add second child
        root.append(right, arena);
        let mut more_children = root.children(arena);
        assert_eq!(more_children.next(), Some(left));
        assert_eq!(more_children.next(), Some(right));
        assert!(more_children.next().is_none());
    }

    #[test]
    fn node_should_know_its_parent() {
        let arena = &mut Arena::new();
        let root = arena.new_node(9999);
        let child = arena.new_node(9999);
        root.append(child, arena);

        assert!(arena.get(root).is_some());
        arena.get(root).map(|n| assert!(n.parent().is_none()));

        assert!(arena.get(child).is_some());
        arena.get(child).map(|n| assert_eq!(n.parent(), Some(root)));
    }

    #[test]
    fn create_binary_tree_at_1_level() {
        let tree_array = BoardingPass::generate_boarding_pass_tree(1);
        // let actual = root.descendants(arena).collect::<Vec<NodeId>>().len();
        let actual = tree_array.array.len();
        assert_eq!(actual, 1); // creates 1 nodes
    }

    #[test]
    fn create_binary_tree_at_2_levels() {
        let tree_array = BoardingPass::generate_boarding_pass_tree(2);
        let actual = tree_array.array.len();
        assert_eq!(actual, 3); // creates 3 nodes
    }

    #[test]
    fn create_binary_tree_at_3_levels() {
        let tree_array = BoardingPass::generate_boarding_pass_tree(3);
        let actual = tree_array.array.len();
        assert_eq!(actual, 7);
    }

    #[test]
    fn create_binary_tree_at_n_levels() {
        let tree_array = BoardingPass::generate_boarding_pass_tree(8);
        let actual = tree_array.array.len();
        assert_eq!(actual, 255)
    }

    #[test]
    fn should_find_max_seat_id() {
        let snippet = "BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL";
        assert_eq!(day05_1(snippet), 820);
    }
}

/*
How can we represent this as a structure? Or how can we write the test for this? This is where I spin my wheels :)
So we have a pretty basic node structure...what would be a helpful abstraction to test for next?
Trees have a root node, but at end of day it's nodes all the way down. It's also nodes all the way up.
It would be helpful for the children to know their parents.
TODO: Let's look into 'trees with backlinks': https://doc.rust-lang.org/std/rc/index.html#examples
Also:  https://doc.rust-lang.org/std/cell/index.html , https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/
http://jamesmcm.github.io/blog/2020/07/25/intro-dod/
 https://rust-unofficial.github.io/too-many-lists/
Well that's it for ze code. Going to read the suggestion above and call it a day - thanks!
 */
