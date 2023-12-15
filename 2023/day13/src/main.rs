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

fn main() {
    let ans = solve(INPUT);
    println!("ANS: {ans}");
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
        let (up, down) = if row_idx <= middle_index {
            ((0, row_idx), (row_idx, row_idx + row_idx))
        } else {
            (
                (usize::abs_diff(row_idx, d), row_idx),
                (row_idx, row_idx + d),
            )
        };

        let mut is_mirror = true;
        let mut smudge_count = 0;

        for (u, d) in zip((up.0..up.1).rev(), down.0..down.1) {
            let up = pattern_grid[u].iter().collect::<String>();
            let down = pattern_grid[d].iter().collect::<String>();

            if up == down {
                continue;
            }
            if one_char_apart(&up, &down) {
                smudge_count += 1;
                continue;
            }

            is_mirror = false;
            break;
        }

        if is_mirror && smudge_count == 1 {
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

        let mut is_mirror = true;
        let mut smudge_count = 0;

        for (left, right) in zip((left.0..left.1).rev(), right.0..right.1) {
            let mut left_str = String::new();
            let mut right_str = String::new();

            pattern_grid.iter().for_each(|r| {
                left_str.push(r[left]);
                right_str.push(r[right]);
            });

            if left_str == right_str {
                continue;
            }
            if one_char_apart(&left_str, &right_str) {
                smudge_count += 1;
                continue;
            }
            is_mirror = false;
            break;
        }

        if is_mirror && smudge_count == 1 {
            mirror_cols.push(col_idx);
        }

        col_idx += 1;
    }

    mirror_cols.iter().sum::<usize>()
}

fn solve(input: &str) -> usize {
    let mut total = 0;

    let pattern_grids = deser(input);

    let start = Instant::now();

    for pattern_grid in pattern_grids {
        let row = find_row_mirror(&pattern_grid);
        if row != 0 {
            total += row;
            continue;
        }
        total += find_column_mirror(&pattern_grid);
    }

    let end = Instant::now();
    println!("Processed in {:?}", end.duration_since(start));

    total
}

fn one_char_apart(str1: &str, str2: &str) -> bool {
    if str1.len() != str2.len() {
        return false;
    }
    let mut diff_count = 0;
    for (char1, char2) in str1.chars().zip(str2.chars()) {
        if char1 != char2 {
            diff_count += 1;
            if diff_count > 1 {
                return false;
            }
        }
    }
    diff_count == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(solve(TEST2), 405);
    }
}
