const INPUT: &str = include_str!("../../data/input.txt");
const RADIX: u32 = 10;

fn main() {
    println!("{INPUT}");
}

#[derive(Default)]
struct Number {
    chars: Vec<char>,
    index: usize,
}

impl Number {
    fn as_u32(&self) -> u32 {
        let num_string: String = self.chars.iter().collect();
        num_string
            .parse()
            .expect("chars in number struct should parse into a number")
    }
}

#[derive(Default)]
struct Symbol {
    index: usize,
}

#[derive(Default)]
struct Line {
    text: &'static str,
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

fn solve(input: &str) -> u32 {
    let adjacent_numbers = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    for (line_index, &line) in lines.iter().enumerate() {
        let mut l = Line::default();
        let chars: Vec<char> = line.chars().collect();
        
        let mut char_index = 0;
        while char_index < chars.len() {
            if chars[char_index] == '.' {
                char_index += 1;
                continue;
            } else if chars[char_index].is_numeric() {
                let mut n = Number {
                    index: char_index,
                    chars: Vec::new(),
                };
                while char_index < chars.len() && chars[char_index].is_numeric() {
                    let c = chars[char_index];
                    n.chars.push(c);
                    char_index += 1;
                }
                l.numbers.push(n);
                continue;
            } else if chars[char_index].is_ascii_punctuation() {
                let s = Symbol {
                    index: char_index
                };
                l.symbols.push(s);
                char_index += 1;
            } else {
                char_index += 1;
            }
        }  

        for n in l.numbers {
            for s in l.symbols {
                if n.index - 1 == s.index || n.index + 1 == s.index {
                    adjacent_numbers.push(n);
                    break;
                }

                if line_index == 0 {
                    let peek_line = lines[line_index + 1];
                    let peek_line_chars = peek_line.chars();
                    
                }
            }

        }

        // assess current line
        for num in &nums {
            if symbol_index.contains(num.index - 1)
                || symbol_index.contains(num.get_end_index() + 1)
            {
                adjacent_numbers.push()
            }
        }

        if line_index == 0 {
            // only peek to next line
        } else if line_index == lines.len() - 1 {
            // only peek to prev line
        } else {
            // check both next and prev for diagonal
        }
    }

    0
}

#[cfg(test)]
mod tests {}
