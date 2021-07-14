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
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct BoardingPass {
    row: u8,
    column: u8,
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

    fn find_the_row(s: &str) -> i32 {
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
         TODO: Write *tests* for a smaller tree, implement the tree; write tests for bigger tree, implement tree.
         */
        70 // faked it and it passed; now we need to remove duplication
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
        todo!() // if the code reaches here then validations look good.
                // But now we need to represent the letters in a way that lets us translate them to rows/columns
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

#[derive(Debug, Default, PartialEq)]
struct Node<'a> {
    left: Option<Box<&'a Node<'a>>>, // Seems sus to have the same lifetime as parent node, maybe best to have diff lifetimes?
    right: Option<Box<&'a Node<'a>>>, // Then again, I probably need to read more about lifetimes anyways
    parent: Option<Box<&'a Node<'a>>>, // having both sounds weird?
}

impl Node {
    pub fn new<'a>(left: Option<Box<&'a Node>>, right: Option<Box<&'a Node>>) -> Self {
        // bit of a bummer to *have* to supply both, but that can be a limitation of the new method
        // You can always use Node {} with default to supply None as well
        Node {
            left,
            right,
            ..Default::default()
        }
    }
}

fn day05_1(input: &str) -> i32 {
    0
}

fn scan_boarding_pass(input: &str) -> BoardingPass {
    // We have code duplication here as a result, not to mention we are not actually using our input!
    BoardingPass { row: 70, column: 7 } // Note part of TDD: if you don't know the answer right away, fake it to get to Green test result quickly
}

#[cfg(test)]
mod tests {
    use super::*;

    // This would basically be the final solution -- If I can write code that does this reliably, then
    // I can feel confident that I have solved the puzzle (see? This is why tests are so cool!)
    #[test]
    fn it_should_find_the_right_row_column_and_seat() {
        let snippet = "FBFBBFFRLR"; // test input
        let expected = BoardingPass { row: 70, column: 7 }; // representation of a boarding pass
        assert_eq!(scan_boarding_pass(snippet), expected) // if I scan the input, I should get the output
    }

    // The above test is pretty close to what I need, if not 100%. The problem is it's too big of a problem.
    // I'd love to leap to the solution, but I have too much to learn before I can do that
    // So how might we be able to *break the problem down*? A very common practice in software engineering
    // What if I write code that just finds the right row first?
    #[test]
    fn it_should_find_the_right_row() {
        let snippet = "FBFBBFFRLR";
        assert_eq!(BoardingPass::find_the_row(snippet), 70); // We deliberately write a test that fails but has what we want
    }

    // Then what if I write code that just finds the right column?
    #[test]
    fn it_should_find_the_right_column() {
        assert!(true)
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

        println!("{:?}", BoardingPass::from_str(other_bad_snippet));
        BoardingPass::from_str(other_bad_snippet).map_err(|z| println!("{:?}", z.source()));
        assert_eq!(BoardingPass::from_str(snippet), expected_1);
        assert_eq!(BoardingPass::from_str(other_bad_snippet), expected_2);
    }

    #[test] // The purpose of this test changed since I started, so renaming
    fn it_should_create_a_simple_node_with_a_right_and_left_child() {
        // I don't really know what I'm doing with this tree data structure, so I'm going to see if I can fake it
        // Let's create a node representation. Nodes can have zero, one, or two children
        let two_children = Node {
            right: Some(Box::new(&Node {
                ..Default::default()
            })),
            left: Some(Box::new(&Node {
                ..Default::default()
            })),
            parent: None,
        };
        let zero_children = Node {
            right: None,
            left: None,
            parent: None,
        }; // Options should be safer than using nulls in Rust
        let one_child = Node {
            left: Some(Box::new(&Node {
                ..Default::default()
            })),
            ..Default::default()
        }; // There should be a notation to have structs fill in the rest - Google
           // Let's see if we can fix some compilation errors
        assert_eq!(Node::new(None, None), zero_children);
        assert_eq!(Node::new(Some(Default::default()), None), one_child); //Ah okay thought this passed, but it was the failure.
        assert_eq!(
            Node::new(Some(Default::default()), Some(Default::default())),
            two_children
        ); // oops switched them
           // Since we originally wanted to create a tree, the children actually need to be Nodes themselves
           // So let's fix the tests to what we want
           // I wonder if using Default::default() as arguments like this is idiomatic...
           // Ok - found recursion so once more unto the breach we go...
    }

    #[test]
    fn node_should_know_its_parent() {
        let child: Node = Default::default();
        let child_in_box = Box::new(&child);
        let parent = Node::new(Some(child_in_box), None);
        // Seems I am using Owned values in places that could benefit from using Borrowed values. Let's find out.
        assert_eq!(child_in_box.parent, Some(Box::new(parent)))
        // On basic TDD principles, we get to create the API we *think* we want so let's try this one
        // If we add a parent as a field like above, our constructor ::new() would have to decide how to handle it
        // Should children know their parents when constructed or we need another API for that?
        // I have no clue right now so I'll make a best guess here
        // interestingly, wonder if the comparison has to be boxed as well? Like Some(Box::new(parent))? There's probably some symbol that would help...maybe
        // Borrow checker to the rescue? I don't fully understand what I read so let's listen to the borrow checker here
        // but how the heck do I borrow this...Google
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
