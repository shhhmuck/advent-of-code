const INPUT: &str = include_str!("./input.txt");
const TEST: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";

#[derive(Debug)]
enum Kind {
    Space,
    RightMirror,
    LeftMirror,
    HorizontalSplitter,
    VerticalSplitter,
}

impl Kind {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Space,
            '/' => Self::RightMirror,
            '\\' => Self::LeftMirror,
            '-' => Self::HorizontalSplitter,
            '|' => Self::VerticalSplitter,
            _ => unreachable!("All characters are handled"),
        }
    }
}

#[derive(Debug)]
struct Tile {
    is_energized: bool,
    kind: Kind,
    coordinates: (usize, usize),
}

fn main() {
    println!("{}", part_1(TEST));
}

fn deserialize(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .enumerate()
        .map(|(row_idx, r)| {
            r.chars()
                .enumerate()
                .map(|(col_idx, c)| Tile {
                    is_energized: false,
                    kind: Kind::from_char(c),
                    coordinates: (row_idx, col_idx),
                })
                .collect()
        })
        .collect()
}

fn part_1(input: &str) -> usize {
    let mut grid = deserialize(input);
    for row in &grid {
        for tile in row {
            println!("{tile:?}");
        }
        
    }
    usize::MIN
}
