use std::time::Instant;

const INPUT: &str = include_str!("./input.txt");

fn main() {
    let s = Instant::now();
    println!(
        "Part 1: {} in {:?}",
        part_1(INPUT, (200000000000000.0, 400000000000000.0)),
        Instant::now().duration_since(s)
    );
}

#[derive(Debug)]
struct Position {
    x: f64,
    y: f64,
    z: f64,
}

impl Position {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Position { x, y, z }
    }
}

#[derive(Debug)]
struct Velocity {
    x: f64,
    y: f64,
    z: f64,
}

impl Velocity {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Velocity { x, y, z }
    }
}

#[derive(Debug)]
struct Hailstone {
    position: Position,
    velocity: Velocity,
}

impl Hailstone {
    fn from_line(line: &str) -> Self {
        let (positions, velocities) = line.split_once(" @ ").unwrap();
        let positions: Vec<f64> = positions
            .split(", ")
            .map(|s| s.parse::<f64>().unwrap())
            .collect();
        let velocities: Vec<f64> = velocities
            .split(", ")
            .map(|s| s.parse::<f64>().unwrap())
            .collect();

        Self {
            position: Position::new(positions[0], positions[1], positions[2]),
            velocity: Velocity::new(velocities[0], velocities[1], velocities[2]),
        }
    }

    fn overlap_position(&self, other: &Hailstone) -> Option<Position> {
        let det = (self.velocity.x * other.velocity.y) - (self.velocity.y * other.velocity.x);
        if det == 0.0 {
            return None;
        }
        let time: f64 = ((other.position.x - self.position.x) * other.velocity.y
            - (other.position.y - self.position.y) * other.velocity.x)
            as f64
            / det;
        let time2: f64 = ((self.position.x - other.position.x) * self.velocity.y
            - (self.position.y - other.position.y) * self.velocity.x)
            as f64
            / det;
        if time < 0.0 {
            return None;
        }
        if time * time2 > 0.0 {
            return None;
        }
        let x = self.position.x as f64 + self.velocity.x as f64 * time;
        let y = self.position.y as f64 + self.velocity.y as f64 * time;

        Some(Position { x, y, z: 0.0 })
    }
}

fn part_1(input: &str, range: (f64, f64)) -> usize {
    let hailstones = input
        .lines()
        .map(Hailstone::from_line)
        .collect::<Vec<Hailstone>>();

    get_overlapped_within_range(&hailstones, range)
}

fn get_overlapped_within_range(hailstones: &[Hailstone], allowed_range: (f64, f64)) -> usize {
    let mut overlapped_within_range = 0;
    let (min, max) = allowed_range;

    for i in 0..hailstones.len() {
        for j in i + 1..hailstones.len() {
            if let Some(Position { x, y, z: _ }) = hailstones[i].overlap_position(&hailstones[j]) {
                if x > min && x < max && y > min && y < max {
                    overlapped_within_range += 1;
                }
            }
        }
    }

    overlapped_within_range
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
19, 13, 30 @ -2, 1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @ 1, -5, -3";

    // round the values from overlap function to 3 decimal places to match example tests rounding
    fn round_to_precision(x: f64, precision: u32) -> f64 {
        let multiplier = 10f64.powi(precision as i32);
        (x * multiplier).round() / multiplier
    }

    #[test]
    fn test_overlap_1() {
        let hailstone_1 = Hailstone::from_line("19, 13, 30 @ -2, 1, -2");
        let hailstone_2 = Hailstone::from_line("18, 19, 22 @ -1, -1, -2");

        let res = hailstone_1.overlap_position(&hailstone_2).unwrap();

        assert_eq!(round_to_precision(res.x, 3), 14.333);
        assert_eq!(round_to_precision(res.y, 3), 15.333);
    }

    #[test]
    fn test_overlap_2() {
        let hailstone_1 = Hailstone::from_line("19, 13, 30 @ -2, 1, -2");
        let hailstone_2 = Hailstone::from_line("20, 25, 34 @ -2, -2, -4");

        let res = hailstone_1.overlap_position(&hailstone_2).unwrap();

        assert_eq!(round_to_precision(res.x, 3), 11.667);
        assert_eq!(round_to_precision(res.y, 3), 16.667);
    }

    #[test]
    fn it_works() {
        assert_eq!(part_1(TEST, (7.0, 27.0)), 2);
    }
}
