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

    fn can_travel(&self) -> Vec<Direction> {
        match self {
            PipeKind::Vertical => vec![Direction::North, Direction::South],
            PipeKind::Horizontal => vec![Direction::East, Direction::West],
            PipeKind::NorthEast => vec![Direction::North, Direction::East],
            PipeKind::NorthWest => vec![Direction::North, Direction::West],
            PipeKind::SouthWest => vec![Direction::South, Direction::West],
            PipeKind::SouthEast => vec![Direction::South, Direction::East],
            PipeKind::Start => vec![
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ],
            PipeKind::Ground => vec![],
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

    fn walk<'a>(&self, prev: &Pipe, pipes: &'a [Vec<Pipe>]) -> &'a Self {
        let pipes: Vec<&Pipe> = self
            .kind
            .can_travel()
            .iter()
            .map(|d| match d {
                Direction::North => &pipes[self.row - 1][self.column],
                Direction::East => &pipes[self.row][self.column + 1],
                Direction::South => &pipes[self.row + 1][self.column],
                Direction::West => &pipes[self.row][self.column - 1],
            })
            .collect();

        pipes
            .iter()
            .find(|&&p| p != prev)
            .expect("pipes will be looping and not hit bounds")
    }
}

fn main() {
    let solve = part_1(INPUT);
    println!("Answer: {}", solve);
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

    let start = Instant::now();
    let mut main_loop: Vec<&Pipe> = vec![current_pipe];

    // logic to start the main loop
    for dir in current_pipe.kind.can_travel() {
        match dir {
            Direction::North => {
                let r = current_pipe.row - 1;
                if r != 0 {
                    let p = &pipes[r][current_pipe.column];
                    if p.kind.can_travel().contains(&Direction::South) {
                        main_loop.push(p);
                        break;
                    }
                }
            }
            Direction::East => {
                let c = current_pipe.column + 1;
                if c < pipes[current_pipe.row].len() {
                    let p = &pipes[current_pipe.row][c];
                    if p.kind.can_travel().contains(&Direction::West) {
                        main_loop.push(p);
                        break;
                    }
                }
            }
            Direction::South => {
                let r = current_pipe.row + 1;
                if r < pipes.len() {
                    let p = &pipes[r][current_pipe.column];
                    if p.kind.can_travel().contains(&Direction::North) {
                        main_loop.push(p);
                        break;
                    }
                }
            }
            Direction::West => {
                let c = current_pipe.column - 1;
                if c != 0 {
                    let p = &pipes[current_pipe.row][c];
                    if p.kind.can_travel().contains(&Direction::East) {
                        main_loop.push(p);
                        break;
                    }
                }
            }
        }
    }

    loop {
        let p = main_loop[main_loop.len() - 1];
        if p.is_start {
            break;
        }
        let n = p.walk(main_loop[main_loop.len() - 2], &pipes);
        main_loop.push(n);
    }

    let end = Instant::now();
    println!("Processed in {:?}\n", end.duration_since(start));

    // println!("loop 1: {:?}", main_loop.len() / 2);

    (main_loop.len() / 2) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deser() {
        let input = "\
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

        assert_eq!(deserialize(input), expected)
    }

    #[test]
    fn test_part_1_simple() {
        let input = "\
.....
.S-7.
.|.|.
.L-J.
.....\
";

        assert_eq!(part_1(input), 4);
    }

    #[test]
    fn test_part_1_complex() {
        let input = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...\
";

        assert_eq!(part_1(input), 8)
    }
}
