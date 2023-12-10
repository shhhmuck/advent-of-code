use std::time::Instant;

const INPUT: &str = include_str!("./input.txt");
const TEST: &str = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ\
";

#[derive(Debug, PartialEq, Clone, Copy)]
enum PipeKind {
    Vertical,   // |
    Horizontal, // -
    NorthEast,  // L
    NorthWest,  // J
    SouthWest,  // 7
    SouthEast,  // F
    Ground,     // .
    Start,      // S
}

impl PipeKind {
    fn from(c: char) -> Self {
        match c {
            '|' => PipeKind::Horizontal,
            '-' => PipeKind::Vertical,
            'L' => PipeKind::NorthEast,
            'J' => PipeKind::NorthWest,
            '7' => PipeKind::SouthWest,
            'F' => PipeKind::SouthEast,
            '.' => PipeKind::Ground,
            'S' => PipeKind::Start,
            _ => unreachable!("All characters in the input are handled"),
        }
    }
}

#[derive(Debug)]
struct Pipe {
    row: usize,
    column: usize,
    kind: PipeKind,
}

fn main() {
    part_1(TEST);
}

fn deserialize(input: &str) -> Vec<Vec<Pipe>> {
    let start = Instant::now();

    let pipes: Vec<Vec<Pipe>> = input
        .lines()
        .enumerate()
        .map(|(row, l)| {
            l.chars()
                .enumerate()
                .map(|(column, c)| Pipe {
                    kind: PipeKind::from(c),
                    row,
                    column,
                })
                .collect()
        })
        .collect();

    let end = Instant::now();
    println!("Deserialized in {:?}", end.duration_since(start));

    pipes
}

fn part_1(input: &str) -> u64 {
    let pipes = deserialize(input);

    let mut current_pipe = &pipes[0][0];

    let mut row_idx = 0;

    while row_idx < pipes.len() {
        let row = &pipes[row_idx];
        let mut col_idx = 0;
        while col_idx < row.len() {
            let pipe = &row[col_idx];
            if pipe.kind == PipeKind::Start {
                current_pipe = pipe;
                break;
            }
            col_idx += 1;
        }
        row_idx += 1;
    }

    println!("{:?}", current_pipe);

    0
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_deser() {
        
    }
}