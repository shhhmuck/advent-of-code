use std::time::Instant;

const INPUT: &str = include_str!("./input.txt");
const TEST: &str = "\
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ\
";

const TEST2: &str = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L\
";

const TEST3: &str = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...\
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

    fn as_char(&self) -> char {
        match self {
            PipeKind::Vertical => '|',
            PipeKind::Horizontal => '-',
            PipeKind::NorthEast => '┕',
            PipeKind::NorthWest => '┙',
            PipeKind::SouthWest => '┑',
            PipeKind::SouthEast => '┍',
            PipeKind::Ground => '.',
            PipeKind::Start => 'S',
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
    // let solve = part_1(INPUT);
    // println!("Answer: {}", solve);
    let solve = part_2(TEST3);
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
    let mut pipes = deserialize(input);

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

fn part_2(input: &str) -> u64 {
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
                if current_pipe.row != 0 {
                    let p = &pipes[current_pipe.row - 1][current_pipe.column];
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
                if current_pipe.column != 0 {
                    let p = &pipes[current_pipe.row][current_pipe.column - 1];
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

    // println!("loop 1: {:?}", main_loop.len() / 2);

    // Paint the loop with ~

    // #┍┑┍S┍┑┍┑┍┑┍┑┍┑┍---┑
    // #|┕┙||||||||||||┍--┙
    // #┕-┑┕┙┕┙||||||┕┙┕-┑#
    // ┍--┙┍--┑||┕┙┕┙#┍┑┍┙#
    // ┕---┙┍-┙┕┙####┍┙┕┙##
    // ###┍-┙┍---┑###┕┑####
    // ##┍┙┍┑┕┑┍-┙┍┑##┕---┑
    // ##┕-┙┕┑||┍┑|┕┑┍-┑┍┑|
    // #####┍┙|||||┍┙┕┑||┕┙
    // #####┕-┙┕┙┕┙┕--┙┕┙##

    // .┍----┑┍┑┍┑┍┑┍-┑....
    // .|┍--┑||||||||┍┙....
    // .||.┍┙||||||||┕┑....
    // ┍┙┕┑┕┑┕┙┕┙||┕┙.┕-┑..
    // ┕--┙.┕┑...┕┙S┑┍-┑┕┑.
    // ....┍-┙..┍┑┍┙|┕┑┕┑┕┑
    // ....┕┑.┍┑||┕┑|.┕┑┕┑|
    // .....|┍┙┕┙|┍┙|┍┑|.┕┙
    // ....┍┙┕-┑.||.||||...
    // ....┕---┙.┕┙.┕┙┕┙...

    let mut enclosed_count = 0;

    // skip row 1 cuz it cannot be enclosed
    let mut row_idx = 1;

    while row_idx < pipes.len() {
        let mut toggle = false;
        let row = &pipes[row_idx];
        let mut col_idx = 0;
        
        while col_idx < row.len() {
            println!("coord: ({row_idx},{col_idx}), toggle: {toggle}, enclosed: {enclosed_count}");

            let current_pipe = &row[col_idx];

            if main_loop.contains(&current_pipe) {
                println!("main loop pipe");
                let next_idx = col_idx + 1;
                if current_pipe.kind == PipeKind::Vertical {
                    // vertical pipe is an edge
                    toggle = !toggle;
                    col_idx += 1;
                } else if next_idx < row.len() {
                    // TODO: read thru horizontal lines to next angle
                    // next two pipes make an edge
                    let peek_pipe = &row[next_idx];
                    if current_pipe.kind == PipeKind::SouthEast
                        && peek_pipe.kind == PipeKind::NorthWest
                        || current_pipe.kind == PipeKind::NorthEast
                            && peek_pipe.kind == PipeKind::SouthWest
                    {
                        toggle = !toggle;
                        col_idx += 2;                        
                    } else {
                        col_idx += 1;
                    }
                } else {
                    // no toggle
                    col_idx += 1;                   
                }
                continue;
            }

            // skip counting any 0th or last cols
            if current_pipe.column == 0 || current_pipe.column == pipes[current_pipe.row].len() - 1
            {
                col_idx += 1;
                continue;
            }

            if toggle {
                // prev_was_line = false;
                println!("HELLO");
                enclosed_count += 1;
            }

            col_idx += 1;
        }

        row_idx += 1;
    }

    // for (row_idx, row) in pipes.iter().enumerate().skip(1) {
    //     let mut toggle = false;

    //     for (col_idx, cur_pipe) in row.iter().enumerate() {
    //         println!("enclosed: {enclosed_count}");

    //         if main_loop.contains(&cur_pipe) {
    //             println!(
    //                 "coords: {},{} toggle from {toggle} to {}",
    //                 cur_pipe.row, cur_pipe.column, !toggle
    //             );
    //             toggle = !toggle;

    //             continue;
    //         }

    //         if cur_pipe.column == 0 || cur_pipe.column == pipes[cur_pipe.row].len() - 1 {
    //             println!(
    //                 "coords: {},{} is an edge, ignoring",
    //                 cur_pipe.row, cur_pipe.column
    //             );
    //             continue;
    //         }

    //         if toggle {
    //             // prev_was_line = false;
    //             println!("HELLO");
    //             enclosed_count += 1;
    //         }
    //     }
    // }

    let end = Instant::now();
    println!("Processed in {:?}\n", end.duration_since(start));

    // let mut all: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    // for (r_idx, r) in all.iter_mut().enumerate() {
    //     for (c_idx, c) in r.iter_mut().enumerate() {
    //         if !main_loop.contains(&&Pipe {
    //             row: r_idx,
    //             column: c_idx,
    //             kind: PipeKind::from(*c),
    //             is_start: PipeKind::from(*c) == PipeKind::Start,
    //         }) {
    //             *c = '.';
    //         }
    //     }
    // }

    // for pipe in &main_loop {
    //     all[pipe.row][pipe.column] = PipeKind::from(all[pipe.row][pipe.column]).as_char();
    // }

    // let mut s: String = String::new();

    // for l in all {
    //     for c in l {
    //         s.push(c);
    //     }
    //     s.push('\n');
    // }

    // println!("{}", s);

    enclosed_count

    // for (row_idx, row) in all.iter().enumerate() {
    //     for (col_idx, col) in row.iter().enumerate() {
    //         // println!("{:?}", all[row_idx][col_idx]);
    //         // is main loop
    //         if *col == '0' {
    //             continue;
    //         }
    //         // edges can't be enclosed
    //         if row_idx == 0
    //             || row_idx == all.len() - 1
    //             || col_idx == 0
    //             || col_idx == all[row_idx].len() - 1
    //         {
    //             continue;
    //         }
    //         let mut escaped = false;
    //         // north
    //         for i in (0..=row_idx - 1).rev() {
    //             if all[i][col_idx] == '0' {
    //                 break;
    //             }
    //             escaped = true;
    //         }
    //         if !escaped {
    //             continue;
    //         }
    //         // south
    //         for i in row_idx + 1..all.len() {
    //             if all[i][col_idx] == '0' {
    //                 break;
    //             }
    //             escaped = true;
    //         }
    //         if !escaped {
    //             continue;
    //         }
    //         // east
    //         for i in col_idx + 1..all[row_idx].len() {
    //             if all[row_idx][i] == '0' {
    //                 break;
    //             }
    //             escaped = true;
    //         }
    //         if !escaped {
    //             continue;
    //         }
    //         // west
    //         for i in (0..=col_idx - 1).rev() {
    //             if all[row_idx][i] == '0' {
    //                 break;
    //             }
    //             escaped = true;
    //         }
    //         if !escaped {
    //             continue;
    //         }
    //     }
    //     enclosed += 1;
    // }
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

    #[test]
    fn test_part_2_small() {
        let input = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...\
";

        assert_eq!(part_2(input), 8)
    }

    #[test]
    fn test_part_2_big() {
        let input = "\
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L\
";

        // #┍┑┍S┍┑┍┑┍┑┍┑┍┑┍---┑
        // #|┕┙||||||||||||┍--┙
        // #┕-┑┕┙┕┙||||||┕┙┕-┑#
        // ┍--┙┍--┑||┕┙┕┙#┍┑┍┙#
        // ┕---┙┍-┙┕┙####┍┙┕┙##
        // ###┍-┙┍---┑###┕┑####
        // ##┍┙┍┑┕┑┍-┙┍┑##┕---┑
        // ##┕-┙┕┑||┍┑|┕┑┍-┑┍┑|
        // #####┍┙|||||┍┙┕┑||┕┙
        // #####┕-┙┕┙┕┙┕--┙┕┙##

        assert_eq!(part_2(input), 10)
    }
}
