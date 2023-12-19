use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::f32::INFINITY;
use std::time::Instant;

const INPUT: &str = include_str!("./input.txt");
const TEST: &str = "\
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

type Point = (usize, usize);
type Direction = (isize, isize);

#[derive(Clone, Copy, PartialEq, Eq)]
struct State {
    point: Point,
    cost: usize,
    moves: usize,
    direction: Option<Direction>,
}

impl State {
    fn new(point: Point, cost: usize, moves: usize, direction: Option<Direction>) -> Self {
        Self {
            point,
            cost,
            moves,
            direction,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_lowest_heat_path(
    graph: &[Vec<usize>],
    start: Point,
    end: Point,
    dir_limit: usize,
) -> Option<usize> {
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut costs = HashMap::new();

    heap.push(State::new(start, 0, 0, None));

    while let Some(state) = heap.pop() {
        // if we've reached the end point we can return the cost
        if state.point == end {
            return Some(state.cost);
        }

        // if we've explored this state skip it
        if visited.contains(&(state.point, state.direction, state.moves)) {
            continue;
        }

        // add this state to visited so we can ignore next time
        visited.insert((state.point, state.direction, state.moves));

        costs.insert(state.point, (state.cost, state.direction, state.moves));

        for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)].iter() {
            // ignore exploration of opposite direction as we can only go straight left or right
            if state.direction == Some((-dir.0, -dir.1)) {
                continue;
            }

            let next_point = (
                state.point.0 as isize + dir.0,
                state.point.1 as isize + dir.1,
            );

            // if we are moving the same direction iterate the moves
            let next_moves = if state.direction == Some(*dir) {
                state.moves + 1
            } else {
                1
            };

            // let mut consecutive_direction_count = 0;
            if next_point.0 >= 0
                && next_point.0 < graph.len() as isize
                && next_point.1 >= 0
                && next_point.1 < graph[0].len() as isize
                // && consecutive_direction_count < dir_limit as isize
            {
                let next_point_usize = (next_point.0 as usize, next_point.1 as usize);
                let next_cost = state.cost + graph[next_point_usize.0][next_point_usize.1];

                if !visited.contains(&(next_point_usize, Some(*dir), next_moves))
                    && (costs
                        .get(&next_point_usize)
                        .map_or(true, |&(c, _, _)| next_cost < c))
                {
                    heap.push(State::new(
                        next_point_usize,
                        next_cost,
                        next_moves,
                        Some(*dir),
                    ));
                }
                // next_point = (next_point.0 + dir.0, next_point.1 + dir.1);
                // consecutive_direction_count += 1;
            }
            // consecutive_direction_count = 0;
        }
    }
    None
}

fn main() {
    println!("{}", part_1(TEST));
}

fn part_1(input: &str) -> usize {
    let graph = deser(input);
    let s = Instant::now();
    let ans =
        get_lowest_heat_path(&graph, (0, 0), (graph.len() - 1, graph[0].len() - 1), 3).unwrap();
    let e = Instant::now();
    println!("Processed in {:?}", e.duration_since(s));
    ans
}

fn deser(input: &str) -> Vec<Vec<usize>> {
    let s = Instant::now();
    let grid = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).expect("input is numbers") as usize)
                .collect()
        })
        .collect();
    let e = Instant::now();
    println!("Deserialized in {:?}", e.duration_since(s));
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST), 102)
    }
}

// TODO: this does not work
// fn part_1(input: &str) -> usize {
//     let grid = deser(input);
//     // for row in &grid {
//     //     println!("{row:?}");
//     // }
//     let row_count = grid.len();
//     let col_count = grid[0].len();
//     let mut dp = vec![vec![usize::MAX / 2; col_count]; row_count];
//     dp[0][0] = grid[0][0];
//     for (r, row) in grid.iter().enumerate().take(row_count) {
//         for (c, col) in row.iter().enumerate().take(col_count) {
//             for m in 0..=3 {
//                 if r >= m {
//                     dp[r][c] = min(dp[r][c], dp[r - m][c] + *col);
//                 }
//                 if c >= m {
//                     dp[r][c] = min(dp[r][c], dp[r][c - m] + *col);
//                 }
//             }
//             // if r > 0 {
//             //     dp[r][c] = min(dp[r][c], dp[r - 1][c] + *col);
//             // }
//             // if c > 0 {
//             //     dp[r][c] = min(dp[r][c], dp[r][c - 1] + *col);
//             // }
//             // if r > 2 {
//             //     dp[r][c] = min(dp[r][c], dp[r - 3][c] + *col);

//             // }
//             // if c > 2 {
//             //     dp[r][c] = min(dp[r][c], dp[r][c - 3] + *col);
//             // }
//         }
//     }
//     dp[row_count - 1][col_count - 1]
// }
