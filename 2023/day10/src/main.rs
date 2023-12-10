use std::time::Instant;

const INPUT: &str = include_str!("./input.txt");
const TEST: &str = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ\
";

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
            '|' => PipeKind::Vertical,
            '-' => PipeKind::Horizontal,
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

#[derive(Debug, PartialEq, Eq)]
struct Pipe {
    row: usize,
    column: usize,
    kind: PipeKind,
}

impl Pipe {
    fn from(row: usize, column: usize, c: char) -> Self {
        Self {
            row,
            column,
            kind: PipeKind::from(c),
        }
    }
}

fn main() {
    part_1(INPUT);
}

fn deserialize(input: &str) -> Vec<Vec<Pipe>> {
    let start = Instant::now();

    let pipes: Vec<Vec<Pipe>> = input
        .lines()
        .enumerate()
        .map(|(row, l)| {
            l.chars()
                .enumerate()
                .map(|(column, c)| Pipe::from(row, column, c))
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

    assert_eq!(current_pipe.kind, PipeKind::Start);

    println!("{:?}", pipes[current_pipe.row - 1][current_pipe.column + 1]);

    let mut path_1: Vec<&Pipe> = vec![current_pipe];
    let mut path_2: Vec<&Pipe> = vec![current_pipe];

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deser() {
        let test = "\
|-LJ7F.S
S.F7JL-|\
";

        let expected: Vec<Vec<Pipe>> = vec![
            vec![
                Pipe::from(0, 0, '|'),
                Pipe::from(0, 1, '-'),
                Pipe::from(0, 2, 'L'),
                Pipe::from(0, 3, 'J'),
                Pipe::from(0, 4, '7'),
                Pipe::from(0, 5, 'F'),
                Pipe::from(0, 6, '.'),
                Pipe::from(0, 7, 'S'),
            ],
            vec![
                Pipe::from(1, 0, 'S'),
                Pipe::from(1, 1, '.'),
                Pipe::from(1, 2, 'F'),
                Pipe::from(1, 3, '7'),
                Pipe::from(1, 4, 'J'),
                Pipe::from(1, 5, 'L'),
                Pipe::from(1, 6, '-'),
                Pipe::from(1, 7, '|'),
            ],
        ];

        assert_eq!(deserialize(test), expected)
    }
}
