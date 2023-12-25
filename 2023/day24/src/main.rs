const INPUT: &str = include_str!("./input.txt");

fn main() {
    part_1(INPUT);
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
        if let Some(time) = self.overlap_time(other) {
            let x_self = self.position.x + self.velocity.x * time;
            let y_self = self.position.y + self.velocity.y * time;
            // let z_self = self.position.z + self.velocity.z * time;
            let x_other = other.position.x + other.velocity.x * time;
            let y_other = other.position.y + other.velocity.y * time;
            // let z_other = other.position.z + other.velocity.z * time;

            let x = (x_self + x_other) / 2.0;
            let y = (y_self + y_other) / 2.0;
            // let z = (z_self + z_other) / 2.0;

            Some(Position::new(x, y, 0.0))
        } else {
            None
        }
    }

    fn overlap_time(&self, other: &Hailstone) -> Option<f64> {
        let div_x = other.velocity.x - self.velocity.x;
        let div_y = other.velocity.y - self.velocity.y;
        // let div_z = other.velocity.z - self.velocity.z;

        // Check if velocities are parallel or hailstones never intersect
        if div_x.abs() < f64::EPSILON || div_y.abs() < f64::EPSILON
        /*|| div_z.abs() < f64::EPSILON*/
        {
            return None;
        }

        let time_x = (self.position.x - other.position.x) / div_x;
        let time_y = (self.position.y - other.position.y) / div_y;
        // let time_z = (self.position.z - other.position.z) / div_z;

        // Select maximum non-negative time for intersection
        let max_time = [time_x, time_y /*time_z*/]
            .iter()
            .filter(|&t| *t >= 0.0)
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);

        if max_time.is_infinite() {
            None
        } else {
            Some(max_time)
        }
    }
}

fn part_1(input: &str) -> usize {
    let hailstones = input
        .lines()
        .map(Hailstone::from_line)
        .collect::<Vec<Hailstone>>();

    let mut overlapped_within_range = 0;

    for i in 0..hailstones.len() {
        for j in i + 1..hailstones.len() {
            if let Some(Position { x, y, z: _ }) = hailstones[i].overlap_position(&hailstones[j]) {
                if x > 7.0 && x < 27.0 && y > 7.0 && y < 27.0 {
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

    #[test]
    fn test_overlap_1() {
        let hailstone_1 = Hailstone::from_line("19, 13, 30 @ -2, 1, -2");
        let hailstone_2 = Hailstone::from_line("18, 19, 22 @ -1, -1, -2");

        let res = hailstone_1.overlap_position(&hailstone_2).unwrap();

        assert_eq!(res.x, 14.333);
        assert_eq!(res.y, 15.333);
    }

    #[test]
    fn test_overlap_2() {
        let hailstone_1 = Hailstone::from_line("19, 13, 30 @ -2, 1, -2");
        let hailstone_2 = Hailstone::from_line("20, 25, 34 @ -2, -2, -4");

        let res = hailstone_1.overlap_position(&hailstone_2).unwrap();

        assert_eq!(res.x, 11.667);
        assert_eq!(res.y, 16.667);
    }

    #[test]
    fn it_works() {
        assert_eq!(part_1(TEST), 2);
    }
}
