use std::time::Instant;

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
    pos: (usize, usize),
}

#[derive(Debug)]
struct Beam {
    complete: bool,
    pos: (isize, isize),
    dir: Direction,
}

impl Default for Beam {
    fn default() -> Self {
        Self {
            complete: false,
            pos: (0, -1),
            dir: Direction::default(),
        }
    }
}

fn main() {
    // println!("{}", part_1(INPUT));
    println!("{}", part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    let mut grid = deserialize(input);

    let s = Instant::now();

    let mut beams = vec![Beam::default()];
    let mut new_beams = Vec::new();

    loop {
        if beams.is_empty() {
            break;
        }

        for beam in &mut beams {
            let offset = beam.dir.offset();

            beam.pos.0 += offset.0;
            beam.pos.1 += offset.1;

            if beam.pos.0 < 0 || beam.pos.0 >= grid.len() as isize {
                beam.complete = true;
                break;
            }
            let row_idx = beam.pos.0 as usize;

            if beam.pos.1 < 0 || beam.pos.1 >= grid[row_idx].len() as isize {
                beam.complete = true;
                break;
            }
            let col_idx = beam.pos.1 as usize;

            match grid[row_idx][col_idx].kind {
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
                        if !grid[row_idx][col_idx].is_energized {
                            new_beams.push(Beam {
                                complete: false,
                                pos: beam.pos,
                                dir: Direction::Right,
                            });
                            new_beams.push(Beam {
                                complete: false,
                                pos: beam.pos,
                                dir: Direction::Left,
                            });
                        }
                        beam.complete = true;
                    }
                    Direction::Left => {}
                    Direction::Right => {}
                },
                Kind::VerticalSplitter => match beam.dir {
                    Direction::Up => {}
                    Direction::Down => {}
                    Direction::Left | Direction::Right => {
                        if !grid[row_idx][col_idx].is_energized {
                            new_beams.push(Beam {
                                complete: false,
                                pos: beam.pos,
                                dir: Direction::Up,
                            });
                            new_beams.push(Beam {
                                complete: false,
                                pos: beam.pos,
                                dir: Direction::Down,
                            });
                        }
                        beam.complete = true;
                    }
                },
            }
            grid[row_idx][col_idx].is_energized = true;
        }
        beams.retain(|b| !b.complete);
        beams.append(&mut new_beams);

        // println!("BEAMS: {beams:?}");
    }

    let mut count = 0;
    for row in grid {
        for tile in row {
            if tile.is_energized {
                count += 1;
            }
        }
    }

    let e = Instant::now();
    println!("Processed in {:?}", e.duration_since(s));

    count
}

fn part_2(input: &str) -> usize {
    let s = Instant::now();
    let mut biggest = 0;

    let g = deserialize(input);

    // right side
    for i in 0..g.len() {
        let mut grid = g.clone();

        let mut beams = vec![Beam {
            complete: false,
            pos: (i as isize, (grid[i].len()) as isize),
            dir: Direction::Left,
        }];
        let mut new_beams = Vec::new();

        loop {
            if beams.is_empty() {
                break;
            }

            for beam in &mut beams {
                let offset = beam.dir.offset();

                beam.pos.0 += offset.0;
                beam.pos.1 += offset.1;

                if beam.pos.0 < 0 || beam.pos.0 >= grid.len() as isize {
                    beam.complete = true;
                    break;
                }
                let row_idx = beam.pos.0 as usize;

                if beam.pos.1 < 0 || beam.pos.1 >= grid[row_idx].len() as isize {
                    beam.complete = true;
                    break;
                }
                let col_idx = beam.pos.1 as usize;

                match grid[row_idx][col_idx].kind {
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
                            if !grid[row_idx][col_idx].is_energized {
                                new_beams.push(Beam {
                                    complete: false,
                                    pos: beam.pos,
                                    dir: Direction::Right,
                                });
                                new_beams.push(Beam {
                                    complete: false,
                                    pos: beam.pos,
                                    dir: Direction::Left,
                                });
                            }
                            beam.complete = true;
                        }
                        Direction::Left => {}
                        Direction::Right => {}
                    },
                    Kind::VerticalSplitter => match beam.dir {
                        Direction::Up => {}
                        Direction::Down => {}
                        Direction::Left | Direction::Right => {
                            if !grid[row_idx][col_idx].is_energized {
                                new_beams.push(Beam {
                                    complete: false,
                                    pos: beam.pos,
                                    dir: Direction::Up,
                                });
                                new_beams.push(Beam {
                                    complete: false,
                                    pos: beam.pos,
                                    dir: Direction::Down,
                                });
                            }
                            beam.complete = true;
                        }
                    },
                }
                grid[row_idx][col_idx].is_energized = true;
            }
            beams.retain(|b| !b.complete);
            beams.append(&mut new_beams);

            // println!("BEAMS: {beams:?}");
        }

        let mut count = 0;
        for row in grid {
            for tile in row {
                if tile.is_energized {
                    count += 1;
                }
            }
        }

        if count > biggest {
            biggest = count;
        }
    }

    // left side
    for i in 0..g.len() {
        let mut grid = g.clone();

        let mut beams = vec![Beam {
            complete: false,
            pos: (i as isize, -1),
            dir: Direction::Right,
        }];
        let mut new_beams = Vec::new();

        loop {
            if beams.is_empty() {
                break;
            }

            for beam in &mut beams {
                let offset = beam.dir.offset();

                beam.pos.0 += offset.0;
                beam.pos.1 += offset.1;

                if beam.pos.0 < 0 || beam.pos.0 >= grid.len() as isize {
                    beam.complete = true;
                    break;
                }
                let row_idx = beam.pos.0 as usize;

                if beam.pos.1 < 0 || beam.pos.1 >= grid[row_idx].len() as isize {
                    beam.complete = true;
                    break;
                }
                let col_idx = beam.pos.1 as usize;

                match grid[row_idx][col_idx].kind {
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
                            if !grid[row_idx][col_idx].is_energized {
                                new_beams.push(Beam {
                                    complete: false,
                                    pos: beam.pos,
                                    dir: Direction::Right,
                                });
                                new_beams.push(Beam {
                                    complete: false,
                                    pos: beam.pos,
                                    dir: Direction::Left,
                                });
                            }
                            beam.complete = true;
                        }
                        Direction::Left => {}
                        Direction::Right => {}
                    },
                    Kind::VerticalSplitter => match beam.dir {
                        Direction::Up => {}
                        Direction::Down => {}
                        Direction::Left | Direction::Right => {
                            if !grid[row_idx][col_idx].is_energized {
                                new_beams.push(Beam {
                                    complete: false,
                                    pos: beam.pos,
                                    dir: Direction::Up,
                                });
                                new_beams.push(Beam {
                                    complete: false,
                                    pos: beam.pos,
                                    dir: Direction::Down,
                                });
                            }
                            beam.complete = true;
                        }
                    },
                }
                grid[row_idx][col_idx].is_energized = true;
            }
            beams.retain(|b| !b.complete);
            beams.append(&mut new_beams);

            // println!("BEAMS: {beams:?}");
        }

        let mut count = 0;
        for row in grid {
            for tile in row {
                if tile.is_energized {
                    count += 1;
                }
            }
        }

        if count > biggest {
            biggest = count;
        }
    }

    // top
    for i in 0..g[0].len() {
        let mut grid = g.clone();

        let mut beams = vec![Beam {
            complete: false,
            pos: (-1, i as isize),
            dir: Direction::Down,
        }];
        let mut new_beams = Vec::new();

        loop {
            if beams.is_empty() {
                break;
            }

            for beam in &mut beams {
                let offset = beam.dir.offset();

                beam.pos.0 += offset.0;
                beam.pos.1 += offset.1;

                if beam.pos.0 < 0 || beam.pos.0 >= grid.len() as isize {
                    beam.complete = true;
                    break;
                }
                let row_idx = beam.pos.0 as usize;

                if beam.pos.1 < 0 || beam.pos.1 >= grid[row_idx].len() as isize {
                    beam.complete = true;
                    break;
                }
                let col_idx = beam.pos.1 as usize;

                match grid[row_idx][col_idx].kind {
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
                            if !grid[row_idx][col_idx].is_energized {
                                new_beams.push(Beam {
                                    complete: false,
                                    pos: beam.pos,
                                    dir: Direction::Right,
                                });
                                new_beams.push(Beam {
                                    complete: false,
                                    pos: beam.pos,
                                    dir: Direction::Left,
                                });
                            }
                            beam.complete = true;
                        }
                        Direction::Left => {}
                        Direction::Right => {}
                    },
                    Kind::VerticalSplitter => match beam.dir {
                        Direction::Up => {}
                        Direction::Down => {}
                        Direction::Left | Direction::Right => {
                            if !grid[row_idx][col_idx].is_energized {
                                new_beams.push(Beam {
                                    complete: false,
                                    pos: beam.pos,
                                    dir: Direction::Up,
                                });
                                new_beams.push(Beam {
                                    complete: false,
                                    pos: beam.pos,
                                    dir: Direction::Down,
                                });
                            }
                            beam.complete = true;
                        }
                    },
                }
                grid[row_idx][col_idx].is_energized = true;
            }
            beams.retain(|b| !b.complete);
            beams.append(&mut new_beams);

            // println!("BEAMS: {beams:?}");
        }

        let mut count = 0;
        for row in grid {
            for tile in row {
                if tile.is_energized {
                    count += 1;
                }
            }
        }

        if count > biggest {
            biggest = count;
        }
    }

    // bottom
    for i in 0..g[0].len() {
        let mut grid = g.clone();

        let mut beams = vec![Beam {
            complete: false,
            pos: (grid.len() as isize, i as isize),
            dir: Direction::Up,
        }];
        let mut new_beams = Vec::new();

        loop {
            if beams.is_empty() {
                break;
            }

            for beam in &mut beams {
                let offset = beam.dir.offset();

                beam.pos.0 += offset.0;
                beam.pos.1 += offset.1;

                if beam.pos.0 < 0 || beam.pos.0 >= grid.len() as isize {
                    beam.complete = true;
                    break;
                }
                let row_idx = beam.pos.0 as usize;

                if beam.pos.1 < 0 || beam.pos.1 >= grid[row_idx].len() as isize {
                    beam.complete = true;
                    break;
                }
                let col_idx = beam.pos.1 as usize;

                match grid[row_idx][col_idx].kind {
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
                            if !grid[row_idx][col_idx].is_energized {
                                new_beams.push(Beam {
                                    complete: false,
                                    pos: beam.pos,
                                    dir: Direction::Right,
                                });
                                new_beams.push(Beam {
                                    complete: false,
                                    pos: beam.pos,
                                    dir: Direction::Left,
                                });
                            }
                            beam.complete = true;
                        }
                        Direction::Left => {}
                        Direction::Right => {}
                    },
                    Kind::VerticalSplitter => match beam.dir {
                        Direction::Up => {}
                        Direction::Down => {}
                        Direction::Left | Direction::Right => {
                            if !grid[row_idx][col_idx].is_energized {
                                new_beams.push(Beam {
                                    complete: false,
                                    pos: beam.pos,
                                    dir: Direction::Up,
                                });
                                new_beams.push(Beam {
                                    complete: false,
                                    pos: beam.pos,
                                    dir: Direction::Down,
                                });
                            }
                            beam.complete = true;
                        }
                    },
                }
                grid[row_idx][col_idx].is_energized = true;
            }
            beams.retain(|b| !b.complete);
            beams.append(&mut new_beams);

            // println!("BEAMS: {beams:?}");
        }

        let mut count = 0;
        for row in grid {
            for tile in row {
                if tile.is_energized {
                    count += 1;
                }
            }
        }

        if count > biggest {
            biggest = count;
        }
    }

    let e = Instant::now();
    println!("Processed in {:?}", e.duration_since(s));

    biggest
}

fn deserialize(input: &str) -> Vec<Vec<Tile>> {
    let s = Instant::now();
    let grid = input
        .lines()
        .enumerate()
        .map(|(row_idx, r)| {
            r.chars()
                .enumerate()
                .map(|(col_idx, c)| Tile {
                    is_energized: false,
                    kind: Kind::from_char(c),
                    pos: (row_idx, col_idx),
                })
                .collect()
        })
        .collect();
    let e = Instant::now();
    println!("Deserialized in {:?}", e.duration_since(s));

    grid
}
