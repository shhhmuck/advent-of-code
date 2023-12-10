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

    let mut pipes: Vec<Pipe> = Vec::new();

    input.lines().enumerate().for_each(|(row, l)| {
        l.chars().enumerate().for_each(|(column, c)| {
            let kind = match c {
                '|' => PipeKind::Horizontal,
                '-' => PipeKind::Vertical,
                'L' => PipeKind::NorthEast,
                'J' => PipeKind::NorthWest,
                '7' => PipeKind::SouthWest,
                'F' => PipeKind::SouthEast,
                '.' => PipeKind::Ground,
                'S' => PipeKind::Start,
                _ => unreachable!("All characters in the input are handled"),
            };
            pipes.push(Pipe { row, column, kind })
        })
    });

    let end = Instant::now();
    println!("Deserialized in {:?}", end.duration_since(start));

    let start = pipes.iter().find(|&p| p.kind == PipeKind::Start);
    println!("{:?}", start);

    0
}
