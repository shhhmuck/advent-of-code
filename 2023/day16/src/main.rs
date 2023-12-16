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

#[derive(Debug, Default, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    #[default]
    Right,
}

impl Direction {
    fn offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

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
    println!("{}", part_1(INPUT));
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
    let mut beams = vec![((0, -1), Direction::default())];
    let mut new_beams = Vec::new();
    loop {
        for beam in &mut beams {
            println!("{beam:?}");
            let (pos, dir) = beam;
            let offset = dir.offset();
            *pos = (pos.0 + offset.0, pos.1 + offset.1);

            if pos.0 < 0 || pos.0 >= grid.len() as isize {
                break;
            }

            let row_idx = pos.0 as usize;
            if pos.1 < 0 || pos.1 >= grid[row_idx].len() as isize {
                break;
            }
   
            let col_idx = pos.1 as usize;

            let tile = &mut grid[row_idx][col_idx];
            
            println!("{tile:?}, Direction: {dir:?}");

            tile.is_energized = true;
            match tile.kind {
                Kind::Space => {}
                Kind::RightMirror => match dir {
                    Direction::Up => *dir = Direction::Right,
                    Direction::Down => *dir = Direction::Left,
                    Direction::Left => *dir = Direction::Down,
                    Direction::Right => *dir = Direction::Up,
                },
                Kind::LeftMirror => match dir {
                    Direction::Up => *dir = Direction::Left,
                    Direction::Down => *dir = Direction::Right,
                    Direction::Left => *dir = Direction::Up,
                    Direction::Right => *dir = Direction::Down,
                },
                Kind::HorizontalSplitter => match dir {
                    Direction::Up | Direction::Down => {
                        new_beams.push(((pos.0, pos.1), Direction::Right));
                        *dir = Direction::Left
                    }
                    Direction::Left => {}
                    Direction::Right => {}
                },
                Kind::VerticalSplitter => match dir {
                    Direction::Up => {}
                    Direction::Down => {}
                    Direction::Left | Direction::Right => {
                        new_beams.push(((pos.0, pos.1), Direction::Up));
                        *dir = Direction::Down
                    }
                },
            }

        }
        beams.extend(new_beams.drain(..))
    }


    usize::MIN
}
