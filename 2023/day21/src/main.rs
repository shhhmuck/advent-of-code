use std::collections::HashSet;
use std::time::Instant;

const INPUT: &str = include_str!("./input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SpotKind {
    Start,
    Garden,
    Rock,
}

impl SpotKind {
    fn from_char(c: char) -> Self {
        match c {
            'S' => Self::Start,
            '#' => Self::Rock,
            '.' => Self::Garden,
            _ => unreachable!("Unknown thing: {}", c),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Spot {
    position: Position,
    kind: SpotKind,
}

impl Spot {
    fn new(row: usize, col: usize, kind: SpotKind) -> Self {
        Self {
            position: Position { row, col },
            kind,
        }
    }
}

fn main() {
    let s = Instant::now();
    println!(
        "Part 1 Answer: {} in {:?}",
        part_1(INPUT, 64),
        Instant::now().duration_since(s)
    );
}

//
fn part_2(input: &str) -> usize {
    0
}

fn part_1(input: &str, max_steps: usize) -> usize {
    let garden = deser(input);

    let mut lasts: HashSet<Spot> = HashSet::new(); // the final spot of the paths that are max_steps long
    let mut stack: Vec<Vec<Spot>> = Vec::new(); // queue of paths to explore
    let mut visited: HashSet<(Spot, usize)> = HashSet::new(); // visited hash of the spot + what step we visited it on (to avoid cycles)

    let start = garden
        .iter()
        .flatten()
        .find(|spot| spot.kind == SpotKind::Start)
        .unwrap();

    stack.push(vec![*start]);

    // let mut highest_stack = 0;

    while let Some(path) = stack.pop() {
        // if stack.len() > highest_stack {
        //     highest_stack = stack.len();

        // }
        if path.len() - 1 == max_steps {
            lasts.insert(path.last().unwrap().clone());
            continue;
        }

        let cur_spot = path.last().unwrap();

        for direction in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_row = cur_spot.position.row as isize + direction.0;
            let new_col = cur_spot.position.col as isize + direction.1;

            if new_row >= 0
                && new_row < garden.len() as isize
                && new_col >= 0
                && new_col < garden[0].len() as isize
            {
                let new_row = new_row as usize;
                let new_col = new_col as usize;

                let next_spot = garden[new_row][new_col];

                if next_spot.kind != SpotKind::Rock && !visited.contains(&(next_spot, path.len())) {
                    let mut new_path = path.clone();
                    new_path.push(next_spot);
                    stack.push(new_path);
                    visited.insert((next_spot, path.len()));
                }
            }
        }
    }

    // println!("Stack: {}", highest_stack);

    lasts.len()
}

fn deser(input: &str) -> Vec<Vec<Spot>> {
    input
        .lines()
        .enumerate()
        .map(|(line_idx, line)| {
            line.chars()
                .enumerate()
                .map(|(char_idx, char)| Spot::new(line_idx, char_idx, SpotKind::from_char(char)))
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn it_works() {
        assert_eq!(part_1(TEST, 6), 16)
    }
}
