use std::time::Instant;

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
}

fn main() {
    let solve = part_1(INPUT);
    println!("{solve}");
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

fn slide_north(thing_grid: &mut [Vec<Thing>]) {
    let start = Instant::now();
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
    let end = Instant::now();
    println!("Slide north in {:?}", end.duration_since(start));
}

fn part_1(input: &str) -> usize {
    let mut thing_grid = deserialize(input);

    for things in &thing_grid {
        println!("{things:?}");
    }
    println!();

    slide_north(&mut thing_grid);

    for things in &thing_grid {
        println!("{things:?}");
    }

    usize::MIN
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST), 136)
    }
}
