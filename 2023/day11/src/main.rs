use std::{collections::HashMap, time::Instant};

const IN: &str = include_str!("./input.txt");

#[derive(Debug)]
struct Galaxy {
    name: usize,
    row: usize,
    column: usize,
}

fn main() {
    let solve = part_2(IN, 1000000);
    println!("{solve}");
}

fn part_2(input: &str, multiplier: usize) -> usize {
    let start = Instant::now();

    let mut space: Vec<Vec<char>> = Vec::new();

    let mut extra_row_indexes: Vec<usize> = Vec::new();
    let mut extra_col_indexes: Vec<usize> = Vec::new();

    for (i, line) in input.lines().enumerate() {
        space.push(line.chars().collect());
        if !line.contains('#') {
            extra_row_indexes.push(i)
        }
    }

    // println!("height {:?}, length: {}", space.len(), space[0].len());

    let mut column = 0;

    while column < space[0].len() {
        if space.iter().all(|r| r[column] == '.') {
            extra_col_indexes.push(column);
        }
        column += 1;
    }

    // println!("extra rows at: {:?}", extra_row_indexes);
    // println!("extra cols at: {:?}", extra_col_indexes);

    // println!("height {:?}, length: {}", space.len(), space[0].len());

    let end = Instant::now();
    println!("Deserialized in {:?}", end.duration_since(start));

    let start = Instant::now();

    let mut map: HashMap<usize, Galaxy> = HashMap::new();
    let mut keys: Vec<usize> = Vec::new();

    let mut plus_rows = 0;
    let mut plus_cols = 0;

    for (row, r) in space.iter().enumerate() {
        // if its a row that needs extra add to the plus rows
        if extra_row_indexes.contains(&row) {
            plus_rows += multiplier - 1;
            continue;
        }

        for (col, &c) in r.iter().enumerate() {
            // println!("({row},{col})");
            if extra_col_indexes.iter().any(|i| i == &col) {
                plus_cols += multiplier - 1;
                // extra_col_indexes.remove(pos);
                continue;
            }

            if c == '#' {
                let n = map.len();
                let g = Galaxy {
                    name: n,
                    row: row + plus_rows,
                    column: col + plus_cols,
                };
                map.insert(n, g);
                keys.push(n);
            }
        }
        // reset plus cols for every row
        plus_cols = 0;
    }

    // println!("{:?}", map);

    // for (name, gal) in map.iter() {
    //     println!("{:?}", gal);
    // }

    // println!("{:?}", keys);

    let mut shortest_path_sum = 0;

    for (name, gal) in &map {
        // println!("name: {name}, gal: {:?}", gal);

        let cur_positon = keys.iter().position(|k| k == name).expect("will have");
        let _rem = keys.remove(cur_positon);

        for otr in keys.iter() {
            match map.get(otr) {
                Some(otr_gal) => {
                    // println!("Gal {:?} -> Other Gal {:?}", gal, otr_gal);
                    let shortest_path = usize::abs_diff(gal.column, otr_gal.column)
                        + usize::abs_diff(gal.row, otr_gal.row);
                    shortest_path_sum += shortest_path;
                }
                None => {
                    // already matched and removed
                    continue;
                }
            };
        }
    }

    let end = Instant::now();
    println!("Processed in {:?}", end.duration_since(start));

    shortest_path_sum
}

fn part_1(input: &str) -> usize {
    let start = Instant::now();

    let mut space: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        space.push(line.chars().collect());
        if !line.contains('#') {
            space.push(line.chars().collect());
        }
    }

    // println!("height {:?}, length: {}", space.len(), space[0].len());

    let mut column = 0;

    while column < space[0].len() {
        if space.iter().all(|r| r[column] == '.') {
            // println!("no galaxies in column {column}");
            space.iter_mut().for_each(|r| r.insert(column, '.'));
            column += 1;
        }
        column += 1;
    }

    // println!("height {:?}, length: {}", space.len(), space[0].len());

    let end = Instant::now();
    println!("Deserialized in {:?}", end.duration_since(start));

    let mut map: HashMap<usize, Galaxy> = HashMap::new();
    let mut keys: Vec<usize> = Vec::new();

    for (row, r) in space.iter().enumerate() {
        for (col, &c) in r.iter().enumerate() {
            if c == '#' {
                let n = map.len();
                let g = Galaxy {
                    name: n,
                    row,
                    column: col,
                };
                map.insert(n, g);
                keys.push(n);
            }
        }
    }

    // println!("{:?}", map);
    // println!("{:?}", keys);

    let start = Instant::now();

    let mut shortest_path_sum = 0;

    for (name, gal) in &map {
        // println!("name: {name}, gal: {:?}", gal);

        let cur_positon = keys.iter().position(|k| k == name).expect("will have");
        let _rem = keys.remove(cur_positon);

        for otr in keys.iter() {
            match map.get(otr) {
                Some(otr_gal) => {
                    // println!("Gal {:?} -> Other Gal {:?}", gal, otr_gal);
                    let shortest_path = usize::abs_diff(gal.column, otr_gal.column)
                        + usize::abs_diff(gal.row, otr_gal.row);
                    shortest_path_sum += shortest_path;
                }
                None => {
                    // already matched and removed
                    continue;
                }
            };
        }
    }

    let end = Instant::now();
    println!("Processed in {:?}", end.duration_since(start));

    shortest_path_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....\
";

        assert_eq!(part_1(input), 374)
    }

    #[test]
    fn test_part_2_2() {
        let input = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....\
";

        assert_eq!(part_2(input, 2), 374)
    }

    #[test]
    fn test_part_2_10() {
        let input = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....\
";

        assert_eq!(part_2(input, 10), 1030)
    }

    #[test]
    fn test_part_2_100() {
        let input = "\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....\
";

        assert_eq!(part_2(input, 100), 8410)
    }
}
