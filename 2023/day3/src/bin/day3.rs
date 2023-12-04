const INPUT: &str = include_str!("../../data/input.txt");

fn main() {
//     let input = "\
// 467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..\
// ";

//     let input2 = "\
// ...................15....904...........850.................329...................13....................................871....816....697....
// ...........53.497........................%....906...610.......*.............735#..&...*......558...68...............68..*......&....*.......\
// ";

    assert!('*'.is_ascii_punctuation());

    let solved = solve(INPUT);
    println!("{solved}");
}

#[derive(Default, Debug, Clone, Copy)]
struct Number {
    integer: u32,
    index: usize,
    len: usize,
}

impl Number {
    fn end_index(&self) -> usize {
        self.index + self.len - 1
    }
}

#[derive(Default, Debug, Clone)]
struct Line {
    numbers: Vec<Number>,
}

fn solve(input: &str) -> u32 {
    let mut adjacent_numbers = Vec::new();

    let lines: Vec<&str> = input.lines().collect();

    for (line_index, &line) in lines.iter().enumerate() {
        let mut l = Line::default();

        let chars: Vec<char> = line.chars().collect();

        let mut char_index = 0;

        while char_index < chars.len() {
            if chars[char_index].is_numeric() {
                let mut n: Number = Number {
                    index: char_index,
                    integer: 0,
                    len: 0,
                };
                let mut number_builder: Vec<char> = Vec::new();

                while char_index < chars.len() && chars[char_index].is_numeric() {
                    let c = chars[char_index];
                    number_builder.push(c);
                    char_index += 1;
                }

                n.len = number_builder.len();
                n.integer = number_builder
                    .iter()
                    .collect::<String>()
                    .parse()
                    .expect("checked is numeric");

                l.numbers.push(n);

                continue;
            } else {
                char_index += 1;

                continue;
            }
        }

        for n in l.numbers {
            println!("current number {:?} end idx: {}", n, n.end_index());

            // check current line for adjacent
            if n.index > 0 {
                println!("Check current line prev char");
                if chars[n.index - 1].is_ascii_punctuation() && chars[n.index - 1] != '.' {
                    adjacent_numbers.push(n);
                    println!("true");
                    continue;
                }
            }

            if n.end_index() + 1 < chars.len() - 1 {
                println!("Check current line next char");
                let next_char = chars[n.end_index() + 1];
                if next_char.is_ascii_punctuation() && next_char != '.' {
                    adjacent_numbers.push(n);
                    println!("true");
                    continue;
                }
            }

            if line_index > 0 {
                // check prev line for adjacent
                println!("Check prev line adjacent");
                let peek_line = lines[line_index - 1];
                let mut found = false;
                for (peek_char_index, peek_char) in peek_line.chars().enumerate() {
                    if peek_char.is_ascii_punctuation()
                        && peek_char != '.'
                        && peek_char_index >= if n.index > 1 { n.index - 1 } else { 0 }
                        && peek_char_index <= n.end_index() + 1
                    {
                        adjacent_numbers.push(n);
                        found = true;
                        println!("true");
                        break;
                    }
                }
                if found {
                    continue;
                }
            }

            if line_index < lines.len() - 1 {
                // check next line for adjacent
                println!("Check next line adjacent");
                let peek_line = lines[line_index + 1];
                let mut found = false;
                for (peek_char_index, peek_char) in peek_line.chars().enumerate() {
                    if peek_char.is_ascii_punctuation()
                        && peek_char != '.'
                        && peek_char_index >= if n.index > 1 { n.index - 1 } else { 0 }
                        && peek_char_index <= n.end_index() + 1
                    {
                        adjacent_numbers.push(n);
                        found = true;
                        println!("true");
                        break;
                    }
                }
                if found {
                    continue;
                }
            }
        }
    }

    adjacent_numbers.iter().fold(0, |a, c| c.integer + a)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
