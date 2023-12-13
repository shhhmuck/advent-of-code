use std::{collections::HashMap, time::Instant};
use itertools::Itertools;

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

fn main() {
    part_1(TEST);
}

fn part_1(input: &str) -> usize {
    // let mut memo: HashMap<String, usize> = HashMap::new();
    let start = Instant::now();

    let mut total = 0;

    for line in input.lines() {
        let split = line.split_once(' ').expect("provided input");
        // let springs: Vec<Condition> = split.0.chars().map(Condition::from).collect();
        let springs: Vec<char> = split.0.chars().collect();
        let counts: Vec<usize> = split
            .1
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        println!("{:?}, {:?}", springs, counts);

        let mut arrangements = 0;
        
        for arrangement in springs.iter().permutations(springs.len()).unique() {
            println!("{:?}", arrangement);
            let mut iter = arrangement.into_iter();
            let mut groups = vec![];

            for &length in &counts {
                let mut group = vec![];
                for _ in 0..length {
                    if let Some(spring) = iter.next() {
                        println!("{spring}");
                        group.push(spring);
                    } else {
                        return arrangements; // Invalid arrangement
                    }
                }
                groups.push(group);
            }

            if groups.iter().all(|group| group.iter().all(|&spring| *spring == '#' || *spring == '?')) {
                arrangements += 1;
            }
        }

        total += arrangements;
    
    }

    let end = Instant::now();
    println!("Deserialized in {:?}", end.duration_since(start));


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
