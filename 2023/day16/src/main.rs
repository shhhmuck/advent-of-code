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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Tile {
    is_energized: bool,
    kind: Kind,
    coordinates: (usize, usize),
}

#[derive(Debug)]
struct Beam {
    complete: bool,
    row: isize,
    col: isize,
    dir: Direction,
    tiles: Vec<Tile>
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
    let mut beams = vec![Beam {
        complete: false,
        row: 0,
        col: -1,
        dir: Direction::default(),
        tiles: Vec::new(),
    }];
    let mut new_beams = Vec::new();

    loop {
        beams.retain(|b| !b.complete);
        if beams.len() == 0 {
            break;
        }
        

        for beam in &mut beams {
            println!("{beam:?}\n");
            let offset = beam.dir.offset();

            beam.row = beam.row + offset.0;
            beam.col = beam.col + offset.1;

            if beam.row < 0 || beam.row >= grid.len() as isize {
                beam.complete = true;
                break;
            }

            let row_idx = beam.row as usize;

            if beam.col < 0 || beam.col >= grid[row_idx].len() as isize {
                beam.complete = true;
                break;
            }

            let col_idx = beam.col as usize;

            let tile = &mut grid[row_idx][col_idx];

            // beam is cycling
            if beam.tiles.contains(tile) {
                println!("cycling, beam complete");
                beam.complete = true;
                break;
            }

            beam.tiles.push(tile.clone());
            // println!("{tile:?}");

            tile.is_energized = true;
            match tile.kind {
                Kind::Space => {}
                Kind::RightMirror => match beam.dir {
                    Direction::Up => beam.dir = Direction::Right,
                    Direction::Down => beam.dir = Direction::Left,
                    Direction::Left => beam.dir = Direction::Down,
                    Direction::Right => beam.dir = Direction::Up,
                },
                Kind::LeftMirror => match beam.dir {
                    Direction::Up => beam.dir = Direction::Left,
                    Direction::Down => beam.dir = Direction::Right,
                    Direction::Left => beam.dir = Direction::Up,
                    Direction::Right => beam.dir = Direction::Down,
                },
                Kind::HorizontalSplitter => match beam.dir {
                    Direction::Up | Direction::Down => {
                        new_beams.push(Beam {
                            complete: false,
                            row: beam.row,
                            col: beam.col,
                            tiles: vec![tile.clone()],
                            dir: Direction::Right,
                        });
                        beam.dir = Direction::Left
                    }
                    Direction::Left => {}
                    Direction::Right => {}
                },
                Kind::VerticalSplitter => match beam.dir {
                    Direction::Up => {}
                    Direction::Down => {}
                    Direction::Left | Direction::Right => {
                        new_beams.push(Beam {
                            complete: false,
                            row: beam.row,
                            col: beam.col,
                            tiles: vec![tile.clone()],
                            dir: Direction::Up,
                        });
                        beam.dir = Direction::Down
                    }
                },
            }
        }
        beams.extend(new_beams.drain(..))
    }

    let mut count = 0;
    for row in grid {
        for tile in row {
            if tile.is_energized {
                count += 1;
            }
        }
    }

    count
}
