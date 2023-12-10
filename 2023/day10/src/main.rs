use std::time::Instant;

const INPUT: &str = include_str!("./input.txt");

#[derive(Debug, PartialEq)]
enum PipeKind {
    Vertical,   // |
    Horizontal, // -
    NorthEast,  // L
    NorthWest,  // J
    SouthWest,  // 7
    SouthEast,  // F
    Ground,     // .
    Start,      // S
}

impl PipeKind {
    fn from(c: char) -> Self {
        match c {
            '|' => PipeKind::Horizontal,
            '-' => PipeKind::Vertical,
            'L' => PipeKind::NorthEast,
            'J' => PipeKind::NorthWest,
            '7' => PipeKind::SouthWest,
            'F' => PipeKind::SouthEast,
            '.' => PipeKind::Ground,
            'S' => PipeKind::Start,
            _ => unreachable!("All characters in the input are handled"),
        }
    }
}

#[derive(Debug)]
struct Pipe {
    row: usize,
    column: usize,
    kind: PipeKind,
}

fn main() {
    part_1(INPUT);
}

fn part_1(input: &str) -> u64 {
    let start = Instant::now();

    let pipes: Vec<Vec<PipeKind>> = input
        .lines()
        .map(|l| l.chars().map(PipeKind::from).collect())
        .collect();

    let end = Instant::now();
    println!("Deserialized in {:?}", end.duration_since(start));

    println!("{:?}", pipes[4][77]);

    0
}
