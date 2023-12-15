use std::{
    collections::{hash_map::Entry, HashMap},
    time::Instant,
};

const INPUT: &str = include_str!("./input.txt");
const TEST: &str = "\
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Thing {
    Rock,
    Cube,
    Space,
}

impl Thing {
    fn from(c: char) -> Thing {
        match c {
            'O' => Thing::Rock,
            '#' => Thing::Cube,
            '.' => Thing::Space,
            _ => unreachable!("input only has these chars"),
        }
    }

    fn as_char(thing: &Thing) -> char {
        match thing {
            Thing::Rock => 'O',
            Thing::Cube => '#',
            Thing::Space => '.',
        }
    }
}

fn main() {
    // let solve = part_1(INPUT);
    let solve = part_2(TEST);
    println!("{solve}");
}

fn part_1(input: &str) -> usize {
    let mut thing_grid = deserialize(input);
    slide_north(&mut thing_grid);
    calculate_load(&thing_grid)
}

fn part_2(input: &str) -> usize {
    let mut thing_grid = deserialize(input);
    let mut map: HashMap<String, usize> = HashMap::new();

    for i in 0..=1000000000 {
        let load = cycle(&mut thing_grid);

        for thing in &thing_grid {
            println!("{thing:?}");
        }

        let hash: String = thing_grid
            .iter()
            .map(|r| r.iter().map(Thing::as_char).collect::<String>())
            .collect();

        match map.entry(hash) {
            Entry::Occupied(entry) => {
                println!("Already seen this cycle! INDEX: {i} HASH: {} LOAD: {load}", entry.key());
            }
            Entry::Vacant(entry) => {
                entry.insert(load);
            }
        }
    }

    usize::MIN
}

fn deserialize(input: &str) -> Vec<Vec<Thing>> {
    let start = Instant::now();
    let deser = input
        .lines()
        .map(|l| l.chars().map(Thing::from).collect())
        .collect();
    let end = Instant::now();
    println!("Deserialized in {:?}", end.duration_since(start));

    deser
}

fn cycle(thing_grid: &mut [Vec<Thing>]) -> usize {
    slide_north(thing_grid);
    slide_west(thing_grid);
    slide_south(thing_grid);
    slide_east(thing_grid);
    calculate_load(thing_grid)
}

fn calculate_load(thing_grid: &[Vec<Thing>]) -> usize {
    thing_grid
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            let mut load = 0;
            row.iter().for_each(|&c| {
                if c == Thing::Rock {
                    load += thing_grid.len() - row_idx
                }
            });
            load
        })
        .sum::<usize>()
}

fn slide_north(thing_grid: &mut [Vec<Thing>]) {
    // let start = Instant::now();
    let mut col_idx = 0;
    while col_idx < thing_grid[0].len() {
        let mut row_idx = 1;
        while row_idx < thing_grid.len() {
            let mut peek_idx = row_idx - 1;

            while thing_grid[peek_idx + 1][col_idx] == Thing::Rock && peek_idx >= 0 {
                let peek = thing_grid[peek_idx][col_idx];
                if peek == Thing::Space {
                    // swap
                    thing_grid[peek_idx][col_idx] = thing_grid[peek_idx + 1][col_idx];
                    thing_grid[peek_idx + 1][col_idx] = peek;
                } else {
                    break;
                }
                if peek_idx == 0 {
                    break;
                } else {
                    peek_idx -= 1;
                }
            }
            row_idx += 1;
        }
        col_idx += 1;
    }
    // let end = Instant::now();
    // println!("Slide north in {:?}", end.duration_since(start));
}

fn slide_south(thing_grid: &mut [Vec<Thing>]) {
    // let start = Instant::now();
    let mut col_idx = 0;
    while col_idx < thing_grid[0].len() {
        let mut row_idx = thing_grid.len() - 2;

        loop {
            let mut peek_idx = row_idx + 1;
            while thing_grid[peek_idx - 1][col_idx] == Thing::Rock && peek_idx < thing_grid.len() {
                let peek = thing_grid[peek_idx][col_idx];
                if peek != Thing::Space {
                    break;
                }
                // swap
                thing_grid[peek_idx][col_idx] = thing_grid[peek_idx - 1][col_idx];
                thing_grid[peek_idx - 1][col_idx] = peek;

                peek_idx += 1;
            }
            if row_idx == 0 {
                break;
            }
            row_idx -= 1;
        }
        col_idx += 1;
    }
    // let end = Instant::now();
    // println!("Slide south in {:?}", end.duration_since(start));
}

fn slide_east(thing_grid: &mut [Vec<Thing>]) {
    // let start = Instant::now();
    for row in thing_grid {
        let mut col_idx = row.len() - 2;
        loop {
            let mut peek_idx = col_idx + 1;
            while row[peek_idx - 1] == Thing::Rock && peek_idx < row.len() {
                let peek = row[peek_idx];
                if peek != Thing::Space {
                    break;
                }
                // swap
                row[peek_idx] = row[peek_idx - 1];
                row[peek_idx - 1] = peek;

                peek_idx += 1;
            }

            if col_idx == 0 {
                break;
            }
            col_idx -= 1;
        }
    }
    // let end = Instant::now();
    // println!("Slide east in {:?}", end.duration_since(start));
}

fn slide_west(thing_grid: &mut [Vec<Thing>]) {
    // let start = Instant::now();
    for row in thing_grid {
        let mut col_idx = 1;
        while col_idx < row.len() {
            let mut peek_idx = col_idx - 1;
            while row[peek_idx + 1] == Thing::Rock && peek_idx >= 0 {
                let peek = row[peek_idx];
                if peek != Thing::Space {
                    break;
                }
                // swap
                row[peek_idx] = row[peek_idx + 1];
                row[peek_idx + 1] = peek;

                if peek_idx == 0 {
                    break;
                } else {
                    peek_idx -= 1;
                }
            }
            col_idx += 1;
        }
    }
    // let end = Instant::now();
    // println!("Slide west in {:?}", end.duration_since(start));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST), 136)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST), 64)
    }
}
