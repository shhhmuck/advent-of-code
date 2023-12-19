use std::time::Instant;

use geo::{polygon, Area, Coord, Polygon};

const INPUT: &str = include_str!("./input.txt");
const TEST: &str = "\
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

#[derive(Debug, Clone, PartialEq)]
enum Thing {
    Trench, // #
    Ground, // .
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => unreachable!("all dir chars are handled"),
        }
    }
}

#[derive(Debug)]
struct Plan {
    dir: Direction,
    size: usize,
    color: String,
}

impl Plan {
    fn from_line(l: &str) -> Self {
        let mut split = l.split_ascii_whitespace();
        let dir = Direction::from_char(split.next().unwrap().chars().next().unwrap());
        let size = split.next().unwrap().parse::<usize>().unwrap();
        let color = split.next().unwrap().replace(['(', ')'], "");
        Plan { dir, size, color }
    }

    fn offset(&self, position: &mut (f64, f64)) {
        match self.dir {
            Direction::Up => position.0 -= self.size as f64,
            Direction::Down => position.0 += self.size as f64,
            Direction::Left => position.1 -= self.size as f64,
            Direction::Right => position.1 += self.size as f64,
        }
    }
}

fn main() {
    // println!("{}", part_1(INPUT));
    println!("{}", part_2(INPUT));
}

fn deser_part_1(input: &str) -> Vec<Plan> {
    let s = Instant::now();
    let plans = input.lines().map(Plan::from_line).collect();
    println!("Deserialized in {:?}", Instant::now().duration_since(s));
    plans
}

fn deser_part_2(input: &str) -> Vec<Plan> {
    let s = Instant::now();
    let plans = input.lines().map(|l| {
        let mut split = l.split_ascii_whitespace();
        split.next();
        split.next();

        let color = split.next().unwrap().replace(['(', ')', '#'], "");

        let size = usize::from_str_radix(&color[0..5], 16).unwrap();
        let dir = match &color.chars().nth(5).unwrap() {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,  
            '3' => Direction::Up,
            _=> unreachable!("no other numbers"),
        };


        Plan { dir, size, color }
    }).collect();
    println!("Deserialized in {:?}", Instant::now().duration_since(s));
    plans
}

fn part_2(input:&str) -> usize {
    let plans = deser_part_2(input);

    let s = Instant::now();

    let mut trench_coords = vec![Coord::from((0_f64,0_f64))];
    
    let mut position = (0_f64, 0_f64);
    let mut trench_size = 0;

    for plan in plans {
        plan.offset(&mut position);
        trench_size += plan.size;
        trench_coords.push(Coord::from(position));
    }

    let p = Polygon::new(trench_coords.into(), vec![]);
    let solve = p.unsigned_area() as usize + trench_size / 2 + 1;

    println!("Processed in {:?}", Instant::now().duration_since(s));

    solve
}

fn part_1(input: &str) -> usize {
    let plans = deser_part_1(input);

    let s = Instant::now();

    let mut trench_coords = vec![Coord::from((0_f64,0_f64))];
    
    let mut position = (0_f64, 0_f64);
    let mut trench_size = 0;

    for plan in plans {
        plan.offset(&mut position);
        trench_size += plan.size;
        trench_coords.push(Coord::from(position));
    }

    let p = Polygon::new(trench_coords.into(), vec![]);
    let solve = p.unsigned_area() as usize + trench_size / 2 + 1;

    println!("Processed in {:?}", Instant::now().duration_since(s));

    solve
}

// fn part_1_old(input: &str) -> usize {
//     let plans = deser_part_1(input);

//     let mut row_check = Vec::new();
//     let mut col_check = Vec::new();

//     let mut rows = 1;
//     let mut cols = 1;

//     // TODO: actual input makes coords hit negative, how do we handle this?
//     // get the max size of the rows/cols
//     for Plan { dir, size, .. } in &plans {
//         match dir {
//             Direction::Up => {
//                 rows -= size;
//                 row_check.push(rows);
//             }
//             Direction::Down => {
//                 rows += size;
//                 row_check.push(rows);
//             }
//             Direction::Right => {
//                 cols += size;
//                 col_check.push(cols);
//             }
//             Direction::Left => {
//                 cols -= size;
//                 col_check.push(cols);
//             }
//         }
//     }

//     rows = *row_check.iter().max().unwrap();
//     cols = *col_check.iter().max().unwrap();

//     drop(row_check);
//     drop(col_check);

//     // init the grid with rows and cols of ground level
//     let mut grid: Vec<Vec<Thing>> = Vec::with_capacity(rows);
//     for _ in 0..rows {
//         let row: Vec<Thing> = vec![Thing::Ground; cols];
//         grid.push(row);
//     }

//     // dig main trench
//     let mut trench_coords: Vec<(usize, usize)> = Vec::new();
//     let mut digger_pos = (0_usize, 0_usize);

//     grid[0][0] = Thing::Trench;
//     trench_coords.push((0, 0));

//     for Plan { dir, size, .. } in &plans {
//         match dir {
//             Direction::Up => {
//                 for r in (digger_pos.0 - *size..=digger_pos.0 - 1).rev() {
//                     grid[r][digger_pos.1] = Thing::Trench;
//                     trench_coords.push((r, digger_pos.1));
//                 }
//                 digger_pos.0 -= size;
//             }
//             Direction::Down => {
//                 for r in digger_pos.0 + 1..=digger_pos.0 + *size {
//                     grid[r][digger_pos.1] = Thing::Trench;
//                     trench_coords.push((r, digger_pos.1));
//                 }
//                 digger_pos.0 += size;
//             }
//             Direction::Left => {
//                 for c in (digger_pos.1 - *size..=digger_pos.1 - 1).rev() {
//                     grid[digger_pos.0][c] = Thing::Trench;
//                     trench_coords.push((digger_pos.0, c));
//                 }
//                 digger_pos.1 -= size;
//             }
//             Direction::Right => {
//                 for c in digger_pos.1 + 1..=digger_pos.1 + *size {
//                     grid[digger_pos.0][c] = Thing::Trench;
//                     trench_coords.push((digger_pos.0, c));
//                 }
//                 digger_pos.1 += size;
//             }
//         }
//     }

//     // dig space between trench
//     let mut row_idx = 0;
//     while row_idx < grid.len() {
//         let mut toggle = false;
//         let row = &mut grid[row_idx];
//         let mut col_idx = 0;
//         while col_idx < row.len() {
//             if row[col_idx] == Thing::Trench {
//                 toggle = !toggle;
//                 let mut next = col_idx + 1;
//                 while next < row.len() && row[next] == Thing::Trench {
//                     next += 1;
//                     continue;
//                 }
//                 col_idx = next - 1;
//             } else if toggle {
//                 row[col_idx] = Thing::Trench;
//             }
//             col_idx += 1;
//         }
//         row_idx += 1;
//     }

//     let mut count = 0;

//     for row in &grid {
//         for thing in row {
//             if *thing == Thing::Trench {
//                 count += 1;
//             }
//         }
//     }
//     println!();

//     count
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST), 62)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST), 952408144115)
    }
}
