use std::time::Instant;

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

fn find_column_mirror(pattern_grid: &Vec<Vec<char>>) -> usize {
    let pattern_len = pattern_grid[0].len();
    let middle_index = pattern_len / 2;

    for row in pattern_grid {
        let mut col_idx = 1;
        while col_idx < row.len() {
            let (left_range, right_range) = if col_idx <= middle_index {
                ((0..col_idx).rev(), col_idx..col_idx + col_idx - 0)
            } else {
                (
                    0 + pattern_len - col_idx..col_idx,
                    col_idx..col_idx + pattern_len - col_idx,
                )
            };
            println!(
                "IDX:{} LEFT:{:?} RIGHT:{:?}",
                col_idx, left_range, right_range
            );

            col_idx += 1;
        }
        println!();
    }

    0
}

fn part_1(input: &str) -> usize {
    let pattern_grids = deser(input);
    for pattern_grid in pattern_grids {
        find_column_mirror(&pattern_grid);
        for pattern in pattern_grid {
            println!("{:?}", pattern);
        }
        // println!();
    }

    usize::MIN
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST), 405);
    }
}
