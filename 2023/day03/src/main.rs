#[cfg(test)]
const TEST_STRING: &str = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;

/// What we might want to do is:
/// - Traverse each line and note the full numbers, their starting index and their ending index
/// - Mark symbols and their index (non-digit, non-dot)
/// - For each symbol, finding what it touches from the number's indexes
/// - Get the set of numbers and add them up
/// I really have no idea how else you would do this even though it seems like there is probably a more clever approach
fn main() {
    // 523948 is too low
    println!("Final Answer: {:?}", read_schematics(include_str!("day03.txt")));
}

#[derive(Clone, Debug, Default)]
struct EngineReader {
    current_line: Option<LineReader>,
    previous_line: Option<LineReader>,
    accumulated_numbers: Vec<usize>,
}

impl EngineReader {
    fn check_numbers(&mut self) {
        let pl = self.previous_line.clone().unwrap_or_default();
        let cl = self.current_line.clone().unwrap_or_default();
        if let Some(u) = &mut self.previous_line {
            for i in u.line_numbers.iter_mut() {
                if ((cl.line_symbols.contains(&i.start_pos.unwrap()))
                    || (cl.line_symbols.contains(&i.end_pos.unwrap()))
                    || (pl.line_symbols.contains(&i.start_pos.unwrap()))
                    || (pl.line_symbols.contains(&i.end_pos.unwrap())))
                    && !i.found {
                    i.found = true;
                    self.accumulated_numbers.push(i.number)
                }
            }
        }

        if let Some(v) = &mut self.current_line {
            for j in v.line_numbers.iter_mut() {
                if ((pl.line_symbols.contains(&j.start_pos.unwrap()))
                    || (pl.line_symbols.contains(&j.end_pos.unwrap()))
                    || (cl.line_symbols.contains(&j.start_pos.unwrap()))
                    || (cl.line_symbols.contains(&j.end_pos.unwrap()))
                )
                    && !j.found {
                    j.found = true;
                    self.accumulated_numbers.push(j.number)
                }
            }
        }
    }

    fn push(&mut self, lr: LineReader) {
        if let Some(cl) = &self.current_line {
            self.previous_line = Some(cl.clone());
            self.current_line = Some(lr);
        } else {
            self.current_line = Some(lr);
        }
    }

    fn calculate(&self) -> usize {
        self.accumulated_numbers.iter().sum()
    }
}

#[derive(Clone, Debug, Default)]
struct LineReader {
    line_symbols: Vec<usize>,
    // set might be better here, but uniqueness isn't super important for what i need right now
    line_numbers: Vec<Number>,
    current_number: Option<Number>,
}

fn read_schematics(s: &str) -> usize {
    let mut reader = EngineReader::default();
    for l in s.lines() {
        println!("{:?}", l);
        println!("{:?}", parse_line(l));
        reader.push(parse_line(l.trim()));
        reader.check_numbers();
        println!("{:?}", reader);
        println!("{:?}", reader.accumulated_numbers);
        println!("=================");
    }
    reader.calculate()
}

fn parse_line(s: &str) -> LineReader {
    s.char_indices().fold(LineReader::default(), |mut acc, cn_tuple| {
        if cn_tuple.1 == '.' {
            if let Some(mut num) = acc.current_number.clone() {
                num.parse_number();
                acc.line_numbers.push(num);
            }
            acc.current_number = None;
            return acc;
        }

        if cn_tuple.1 != '.' && !cn_tuple.1.is_ascii_digit() {
            if let Some(mut num) = acc.current_number {
                num.parse_number();
                acc.line_numbers.push(num);
            }

            acc.current_number = None;
            acc.line_symbols.extend((cn_tuple.0 - 1)..=(cn_tuple.0 + 1));
            return acc;
        }

        if let Some(mut num) = acc.current_number.clone() {
            if num.start_pos.is_none() {
                num.start_pos = Some(cn_tuple.0);
            }

            num.end_pos = Some(cn_tuple.0);
            num.chars.push(cn_tuple.1);
            acc.current_number = Some(num.clone());
            if s.len() - 1 == cn_tuple.0 {
                // last loop
                num.parse_number();
                acc.line_numbers.push(num);
            }
        } else {
            let new_number = Number {
                number: 0,
                start_pos: Some(cn_tuple.0),
                end_pos: Some(cn_tuple.0),
                chars: vec![cn_tuple.1],
                found: false,
            };
            acc.current_number = Some(new_number);
        }

        acc
    })
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
struct Number {
    number: usize,
    start_pos: Option<usize>,
    end_pos: Option<usize>,
    chars: Vec<char>,
    found: bool,
}

impl Number {
    fn parse_number(&mut self) {
        let s: String = self.chars.iter().collect();
        self.number = s.parse().unwrap_or(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing() {
        println!("{:?}", read_schematics(TEST_STRING));
    }

    #[test]
    fn testy() {
        let number_at_edge = ".........699....*.........=............15*619.......................*......515....487........................808...............*.....611*121";
        println!("{:?}", read_schematics(number_at_edge));
    }
}
