use std::time::Instant;

const INPUT: &str = include_str!("./input.txt");
// const TEST: &str = "\
// ???.### 1,1,3
// .??..??...?##. 1,1,3
// ?#?#?#?#?#?#?#? 1,3,1,6
// ????.#...#... 4,1,1
// ????.######..#####. 1,6,5
// ?###???????? 3,2,1\
// ";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    springs: Vec<Condition>,
    groups: Vec<usize>,
}

impl Record {
    // for part 2
    fn from_line(l: &str) -> Self {
        let (springs, groups) = l.split_once(' ').unwrap();

        let springs = [springs, springs, springs, springs, springs].join("?");
        let springs = format!(".{}", springs.trim_end_matches('.'))
            .chars()
            .map(Condition::from)
            .collect();

        let groups = [groups, groups, groups, groups, groups].join(",");
        let groups: Vec<usize> = groups
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        Self { springs, groups }
    }
}

fn main() {
    let s = Instant::now();
    println!(
        "Part 1 = {} in {:?}",
        part_1(INPUT),
        Instant::now().duration_since(s)
    );
    let s = Instant::now();
    println!(
        "Part 2 = {} in {:?}",
        part_2(INPUT),
        Instant::now().duration_since(s)
    );
}

fn tabulation(record: &Record) -> usize {
    // table [g - group][i - spring possibilities]
    let mut table = vec![vec![0; record.springs.len() + 1]; record.groups.len() + 1];
    table[0][0] = 1;

    // calc init possibilities for first group springs
    for (i, &c) in record.springs.iter().enumerate() {
        if c == Condition::Damaged {
            break;
        }
        table[0][i + 1] = 1;
    }

    for (g, &count) in record.groups.iter().enumerate() {
        let mut consecutive_non_op = 0;

        for (i, &c) in record.springs.iter().enumerate() {
            if c == Condition::Operational {
                consecutive_non_op = 0;
            } else {
                // iterate consecutive non operational springs
                consecutive_non_op += 1;
            }
            if c != Condition::Damaged {
                // propogate possibilities forward if spring is not damaged
                table[g + 1][i + 1] += table[g + 1][i]
            }
            if consecutive_non_op >= count && record.springs[i - count] != Condition::Damaged {
                // if consecutive non op fits group size, propogate with previous dp count
                table[g + 1][i + 1] += table[g][i - count];
            }
        }
    }
    table[record.groups.len()][record.springs.len()]
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (springs, groups) = l.split_once(' ').unwrap();
            let springs = format!(".{}", springs.trim_end_matches('.'));
            let springs: Vec<Condition> = springs.chars().map(Condition::from).collect();
            let groups: Vec<usize> = groups
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            Record { springs, groups }
        })
        .collect::<Vec<Record>>()
        .iter()
        .map(tabulation)
        .sum::<usize>()
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(Record::from_line)
        .collect::<Vec<Record>>()
        .iter()
        .map(tabulation)
        .sum::<usize>()
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
    fn test_tabulation_1() {
        // ???.### 1,1,3
        let record = Record {
            springs: vec![
                Condition::Operational,
                Condition::Unknown,
                Condition::Unknown,
                Condition::Unknown,
                Condition::Operational,
                Condition::Damaged,
                Condition::Damaged,
                Condition::Damaged,
            ],
            groups: vec![1, 1, 3],
        };

        assert_eq!(tabulation(&record), 1);
    }

    #[test]
    fn test_tabulation_2() {
        // .??..??...?##. 1,1,3
        let record = Record {
            springs: vec![
                Condition::Operational,
                Condition::Operational,
                Condition::Unknown,
                Condition::Unknown,
                Condition::Operational,
                Condition::Operational,
                Condition::Unknown,
                Condition::Unknown,
                Condition::Operational,
                Condition::Operational,
                Condition::Operational,
                Condition::Unknown,
                Condition::Damaged,
                Condition::Damaged,
            ],
            groups: vec![1, 1, 3],
        };

        assert_eq!(tabulation(&record), 4);
    }

    #[test]
    fn test_tabulation_3() {
        // ?###???????? 3,2,1
        let record = Record {
            springs: vec![
                Condition::Operational,
                Condition::Unknown,
                Condition::Damaged,
                Condition::Damaged,
                Condition::Damaged,
                Condition::Unknown,
                Condition::Unknown,
                Condition::Unknown,
                Condition::Unknown,
                Condition::Unknown,
                Condition::Unknown,
                Condition::Unknown,
                Condition::Unknown,
            ],
            groups: vec![3, 2, 1],
        };

        assert_eq!(tabulation(&record), 10);
    }

    #[test]
    fn test_tabulation_4() {
        // ?#?#?#?#?#?#?#? 1,3,1,6
        let record = Record {
            springs: vec![
                Condition::Operational,
                Condition::Unknown,
                Condition::Damaged,
                Condition::Unknown,
                Condition::Damaged,
                Condition::Unknown,
                Condition::Damaged,
                Condition::Unknown,
                Condition::Damaged,
                Condition::Unknown,
                Condition::Damaged,
                Condition::Unknown,
                Condition::Damaged,
                Condition::Unknown,
                Condition::Damaged,
                Condition::Unknown,
            ],
            groups: vec![1, 3, 1, 6],
        };

        assert_eq!(tabulation(&record), 1);
    }

    #[test]
    fn test_part_1_tab() {
        assert_eq!(part_1(TEST), 21);
    }

    #[test]
    fn test_part_2_tab() {
        assert_eq!(part_2(TEST), 525152);
    }
}
