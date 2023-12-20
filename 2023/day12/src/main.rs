use std::{collections::HashMap, time::Instant};

const INPUT: &str = include_str!("./input.txt");
const TEST: &str = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1\
";

#[derive(Debug, Clone, Copy)]
enum Condition {
    Operational, // .
    Damaged,     // #
    Unknown,     // ?
}

impl Condition {
    fn from(c: char) -> Self {
        match c {
            '.' => Condition::Operational,
            '#' => Condition::Damaged,
            '?' => Condition::Unknown,
            _ => unreachable!("All chars are handled"),
        }
    }
}

#[derive(Debug)]
struct Record {
    conditions: Vec<Condition>,
    groups: Vec<usize>,
}

fn main() {
    part_1(TEST);
}

fn deser(input: &str) -> Vec<Record> {
    let s = Instant::now();
    let records = input
        .lines()
        .map(|l| {
            let (conditions, groups) = l.split_once(' ').unwrap();
            let conditions: Vec<Condition> = conditions.chars().map(Condition::from).collect();
            let groups: Vec<usize> = groups
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            Record { conditions, groups }
        })
        .collect();

    let e = Instant::now();
    println!("Deserialized in {:?}", e.duration_since(s));

    records
}

fn part_1(input: &str) -> usize {
    let records = deser(input);

    for record in records {
        println!("{:?} {:?}", record.conditions, record.groups)
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1\
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST), 21);
    }
}
