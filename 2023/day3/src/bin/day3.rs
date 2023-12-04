const INPUT: &str = include_str!("../../data/input.txt");

fn main() {
    let input = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..\
";

    //     let input2 = "\
    // ...................15....904...........850.................329...................13....................................871....816....697....
    // ...........53.497........................%....906...610.......*.............735#..&...*......558...68...............68..*......&....*.......\
    // ";

    // let solved = solve_part_1(INPUT);

    let input3 = "\
...................15....904...........850.................329...................13....................................871....816....697....
...........53.497........................%....906...610.......*.............735#..&...*......558...68...............68..*......&....*.......
..........*....$....................132.........*..........844....875................350............*...............*..336.364...649........
.......726.......341..................*...186...358..................*244........57.......@.........738......*.....663.................584..
.............952.*......33......660..704............949......................518*....234.967....551........971..&.......................*...\
";

    let solved = solve_part_2(INPUT);

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

fn solve_part_1(input: &str) -> u32 {
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

fn solve_part_2(input: &str) -> u64 {
    let mut gear_ratios: Vec<u64> = Vec::new();

    let lines: Vec<&str> = input.lines().collect();

    for (line_idx, line) in lines.iter().enumerate() {
        if line_idx == 0 || line_idx == lines.len() - 1 {
            // cannot have a gear on first or last line
            println!("First or last line");
            continue;
        }

        let chars: Vec<char> = line.chars().collect();

        for (char_idx, &char) in chars.iter().enumerate() {
            if char == '*' {
                let mut adjacents: Vec<u64> = Vec::new();

                let mut backward = String::new();

                println!("check backward adj");
                let mut char_backward_idx = char_idx - 1;
                while char_backward_idx >= 0 && chars[char_backward_idx].is_digit(10) {
                    backward.push(chars[char_backward_idx]);

                    if char_backward_idx == 0 {
                        break;
                    }
                    char_backward_idx -= 1;
                }

                let has_backward_adjacent = !!backward.len() > 0;
                if has_backward_adjacent {
                    let reversed: String = backward.chars().rev().collect();
                    println!("has backward adjacent: {}", reversed);
                    adjacents.push(reversed.parse().expect("already checked this for digit"));
                }

                println!("check forward adj");
                let mut char_forward_idx = char_idx + 1;
                let mut forward = String::new();
                while char_forward_idx != chars.len() - 1 && chars[char_forward_idx].is_digit(10) {
                    forward.push(chars[char_forward_idx]);
                    char_forward_idx += 1;
                }

                let has_forward_adjacent = !!forward.len() > 0;
                if has_forward_adjacent {
                    println!("has forward adjacent: {}", forward);
                    adjacents.push(forward.parse().expect("already checked this for digit"));
                }

                println!("check prev line adj");
                let prev_line = lines[line_idx - 1];
                let prev_chars: Vec<char> = prev_line.chars().collect();
                let mut prev_line_adjacent = String::new();
                let mut prev_line_start_adj_idx = if char_idx > 0 { char_idx - 1 } else { 0 };
                let mut prev_line_end_adj_idx = if char_idx == prev_chars.len() - 1 {
                    char_idx
                } else {
                    char_idx + 1
                };

                // move the start index backward if the digit goes back from the adjacent edge
                while prev_chars[prev_line_start_adj_idx].is_digit(10)
                    && prev_line_start_adj_idx > 0
                {
                    prev_line_start_adj_idx -= 1;
                }

                // move the end index forward if the digit goes forward from the adjacent edge
                while prev_chars[prev_line_end_adj_idx].is_digit(10)
                    && prev_line_end_adj_idx < prev_chars.len() - 1
                {
                    prev_line_end_adj_idx += 1;
                }

                for prev_line_adj_idx in prev_line_start_adj_idx..=prev_line_end_adj_idx {
                    if prev_chars[prev_line_adj_idx].is_digit(10) {
                        prev_line_adjacent.push(prev_chars[prev_line_adj_idx])
                    }
                }

                let has_prev_line_adjacent = !!prev_line_adjacent.len() > 0;
                if has_prev_line_adjacent {
                    println!("has prev line adjacent: {}", prev_line_adjacent);
                    adjacents.push(
                        prev_line_adjacent
                            .parse()
                            .expect("already checked this for digit"),
                    );
                }

                println!("check next line adj");
                let next_line = lines[line_idx + 1];
                let next_chars: Vec<char> = next_line.chars().collect();
                let mut next_line_adjacent = String::new();
                let mut next_line_start_adj_idx = if char_idx > 0 { char_idx - 1 } else { 0 };
                let mut next_line_end_adj_idx = if char_idx == next_chars.len() - 1 {
                    char_idx
                } else {
                    char_idx + 1
                };

                while next_chars[next_line_start_adj_idx].is_digit(10)
                    && next_line_start_adj_idx > 0
                {
                    next_line_start_adj_idx -= 1;
                }

                while next_chars[next_line_end_adj_idx].is_digit(10)
                    && next_line_end_adj_idx < next_chars.len() - 1
                {
                    next_line_end_adj_idx += 1;
                }

                for next_line_adj_idx in next_line_start_adj_idx..=next_line_end_adj_idx {
                    if next_chars[next_line_adj_idx].is_digit(10) {
                        next_line_adjacent.push(next_chars[next_line_adj_idx])
                    }
                }

                let has_next_line_adjacent = !!next_line_adjacent.len() > 0;
                if has_next_line_adjacent {
                    println!("has next line adjacent: {}", next_line_adjacent);
                    adjacents.push(
                        next_line_adjacent
                            .parse()
                            .expect("already checked for digit"),
                    );
                }

                println!("adjacents: {:?}", adjacents);

                if adjacents.len() == 2 {
                    println!(
                        "calculating gear ratio: {} * {}",
                        adjacents[0], adjacents[1]
                    );
                    gear_ratios.push(adjacents[0] * adjacents[1])
                } else {
                    println!("too many or too few adjacents, not a gear");
                }

                adjacents.clear();
            }
        }
    }

    gear_ratios.iter().fold(0, |a, c| a + c)
}
