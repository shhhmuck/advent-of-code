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
enum Direction {
    North,
    East,
    South,
    West,
}

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

    fn can_move(&self) -> Option<[Direction; 2]> {
        match self {
            PipeKind::Vertical => Some([Direction::North, Direction::South]),
            PipeKind::Horizontal => Some([Direction::East, Direction::West]),
            PipeKind::NorthEast => Some([Direction::North, Direction::East]),
            PipeKind::NorthWest => Some([Direction::North, Direction::West]),
            PipeKind::SouthWest => Some([Direction::South, Direction::West]),
            PipeKind::SouthEast => Some([Direction::South, Direction::East]),
            PipeKind::Ground => None,
            PipeKind::Start => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Pipe {
    row: usize,
    column: usize,
    kind: PipeKind,
    is_start: bool,
}

impl Pipe {
    fn from(row: usize, column: usize, c: char) -> Self {
        let kind = PipeKind::from(c);
        Self {
            row,
            column,
            kind,
            is_start: kind == PipeKind::Start,
        }
    }

    fn look<'a>(&self, dir: Direction, pipes: &'a Vec<Vec<Pipe>>) -> Option<&'a Self> {
        match dir {
            Direction::North => {
                if self.row != 0 {
                    Some(&pipes[self.row - 1][self.column])
                } else {
                    None
                }
            }
            Direction::East => {
                if self.column + 1 < pipes[self.row].len() {
                    Some(&pipes[self.row][self.column + 1])
                } else {
                    None
                }
            }
            Direction::South => {
                if self.row + 1 < pipes.len() {
                    Some(&pipes[self.row + 1][self.column])
                } else {
                    None
                }
            }
            Direction::West => {
                if self.column != 0 {
                    Some(&pipes[self.row][self.column - 1])
                } else {
                    None
                }
            }
        }
    }
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
                .map(|(column, c)| Pipe::from(row, column, c))
                .collect()
        })
        .collect();

    let end = Instant::now();
    println!("Deserialized in {:?}\n", end.duration_since(start));

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

    println!("Starting Pipe: {:?}\n", current_pipe);

    let north = current_pipe.look(Direction::North, &pipes);
    let east = current_pipe.look(Direction::East, &pipes);
    let south = current_pipe.look(Direction::South, &pipes);
    let west = current_pipe.look(Direction::West, &pipes);

    println!(
        "North: {:?}\nEast: {:?}\nSouth: {:?}\nWest: {:?}",
        north, east, south, west
    );

    // let mut path_1: Vec<&Pipe> = vec![current_pipe];
    // let mut path_2: Vec<&Pipe> = vec![current_pipe];

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
