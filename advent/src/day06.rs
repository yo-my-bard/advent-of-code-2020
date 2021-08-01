use std::borrow::Borrow;
use std::collections::HashSet;

pub(crate) fn day06_1(input: &str) -> usize {
    sum_up_all_yes_all_groups(input)
}

pub(crate) fn day06_2(input: &str) -> usize {
    0
}

fn find_unique_yes_answers(group: &str) -> usize {
    let mut set: HashSet<char> = HashSet::new();
    group.chars().filter(|c| !c.is_whitespace()).for_each(|ch| {
        set.insert(ch);
    });
    set.len()
}

fn sum_up_all_yes_all_groups(groups: &str) -> usize {
    let mut group_buffer = GroupBuffer {
        groups: Vec::new(),
        buffer: Vec::new(),
    };
    group_buffer.consume(groups.lines());
    group_buffer
        .groups
        .iter()
        .map(String::borrow)
        .map(find_unique_yes_answers)
        .sum()
}

#[derive(Debug)]
struct GroupBuffer {
    groups: Vec<String>,
    buffer: Vec<String>,
}

impl GroupBuffer {
    fn consume<'a>(&'a mut self, lines: impl IntoIterator<Item = &'a str>) -> () {
        for line in lines {
            if line.is_empty() {
                self.groups.push(self.buffer.join("").to_string());
                self.buffer.drain(..);
            } else {
                self.buffer.push(line.to_string())
            }
        }

        if !self.buffer.is_empty() {
            self.groups.push(self.buffer.join("").to_string());
            self.buffer.drain(..);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_unique_yeses_in_a_given_group() {
        let snippet_1 = "abc";
        let snippet_2 = "a\nb\nc";
        let snippet_3 = "ab\nac";
        let snippet_4 = "a\na\na\na";
        let snippet_5 = "b";

        assert_eq!(find_unique_yes_answers(snippet_1), 3);
        assert_eq!(find_unique_yes_answers(snippet_2), 3);
        assert_eq!(find_unique_yes_answers(snippet_3), 3);
        assert_eq!(find_unique_yes_answers(snippet_4), 1);
        assert_eq!(find_unique_yes_answers(snippet_5), 1);
    }

    #[test]
    fn it_should_return_unique_yeses_for_multiple_groups() {
        let snippet = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
        assert_eq!(sum_up_all_yes_all_groups(snippet), 11);
    }

    #[test]
    fn it_should_parse_groups_correctly() {
        let snippet = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
        let mut group_buffer = GroupBuffer {
            groups: Vec::new(),
            buffer: Vec::new(),
        };
        group_buffer.consume(snippet.lines());
        let expected = vec!["abc", "abc", "abac", "aaaa", "b"];
        assert_eq!(group_buffer.groups, expected);
        assert!(group_buffer.buffer.is_empty())
    }
}
