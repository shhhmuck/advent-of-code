use std::{iter::zip, time::Instant};

const INPUT: &str = include_str!("./input.txt");
const TEST: &str = "\
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#\
";
const TEST2: &str = "\
..##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#...##..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#\
";

fn main() {
    let solve = part_1(INPUT);
    println!("ANS: {solve}");
}

fn deser(input: &str) -> Vec<Vec<Vec<char>>> {
    let start = Instant::now();

    let pattern_grids: Vec<Vec<Vec<char>>> = input
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|&s| s.lines().map(|l| l.chars().collect()).collect())
        .collect();

    let end = Instant::now();
    println!("Deserialized in {:?}", end.duration_since(start));

    pattern_grids
}

fn find_row_mirror(pattern_grid: &[Vec<char>]) -> usize {
    let pattern_len = pattern_grid.len();
    let middle_index = pattern_len / 2;

    let mut mirror_rows: Vec<usize> = Vec::new();
    let mut row_idx = 1;

    while row_idx < pattern_len {
        let d = pattern_len - row_idx;
        let (left, right) = if row_idx <= middle_index {
            ((0, row_idx), (row_idx, row_idx + row_idx))
        } else {
            (
                (usize::abs_diff(row_idx, d), row_idx),
                (row_idx, row_idx + d),
            )
        };

        let mut col_idx = 0;
        let mut is_mirror = true;
        while col_idx < pattern_grid[row_idx].len() {
            for (left, right) in zip((left.0..left.1).rev(), right.0..right.1) {
                let up = pattern_grid[left][col_idx];
                let down = pattern_grid[right][col_idx];
                if up != down {
                    is_mirror = false;
                    break;
                }
            }
            col_idx += 1;
        }

        if is_mirror {
            mirror_rows.push(row_idx);
        }
        row_idx += 1;
    }

    mirror_rows.iter().fold(0, |a, c| a + c * 100)
}

fn find_column_mirror(pattern_grid: &[Vec<char>]) -> usize {
    let pattern_len = pattern_grid[0].len();
    let middle_index = pattern_len / 2;

    let mut mirror_cols: Vec<usize> = Vec::new();
    let mut col_idx = 1;

    while col_idx < pattern_len {
        let d = pattern_len - col_idx;
        let (left, right) = if col_idx <= middle_index {
            ((0, col_idx), (col_idx, col_idx + col_idx))
        } else {
            (
                (usize::abs_diff(col_idx, d), col_idx),
                (col_idx, col_idx + d),
            )
        };
        if pattern_grid.iter().all(|row| {
            for (left, right) in zip((left.0..left.1).rev(), right.0..right.1) {
                let l = row[left];
                let r = row[right];
                if l != r {
                    return false;
                }
            }
            true
        }) {
            mirror_cols.push(col_idx);
        }
        col_idx += 1;
    }

    mirror_cols.iter().sum::<usize>()
}

fn part_1(input: &str) -> usize {
    let mut total = 0;

    let pattern_grids = deser(input);

    let start = Instant::now();

    for pattern_grid in pattern_grids {
        total += find_row_mirror(&pattern_grid);
        total += find_column_mirror(&pattern_grid);
    }

    let end = Instant::now();
    println!("Processed in {:?}", end.duration_since(start));

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST2), 400);
    }
}
