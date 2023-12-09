use std::time::Instant;

const INPUT: &str = include_str!("./input.txt");
const TEST: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45\
";

fn main() {
    let solved = part_2(TEST);
    println!("{solved}");
}

fn part_1(input: &str) -> i64 {
    let start = Instant::now();

    let mut sequences: Vec<Vec<i64>> = input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|i| i.parse().expect("will be number"))
                .collect()
        })
        .collect();

    let end = Instant::now();
    println!("Deserialized in: {:?}\n", end.duration_since(start));

    // println!("All sequences: {:?}\n", sequences);

    let start = Instant::now();

    for sequence in &mut sequences {
        // println!("Current sequence: {:?}", sequence);
        let mut lasts: Vec<i64> = Vec::new();
        let mut subbed = sequence.clone();

        while subbed.iter().any(|&n| n != 0) {
            // println!("current subbed {:?}", subbed);
            subbed = subbed.iter().enumerate().fold(Vec::new(), |mut a, (i, n)| {
                if i + 1 < subbed.len() {
                    a.push(subbed[i + 1] - n);
                }
                a
            });
            lasts.push(subbed[subbed.len() - 1]);
            // println!("current lasts {:?}", lasts);
        }
        sequence.push(sequence[sequence.len() - 1] + lasts.iter().fold(0, |a, c| a + c));
    }

    // println!("Final sequences {:?}\n", sequences);

    let solved = sequences.iter().fold(0, |a, s| a + s[s.len() - 1]);

    let end = Instant::now();
    println!("Processed in: {:?}", end.duration_since(start));

    solved
}

fn part_2(input: &str) -> i64 {
    let start = Instant::now();

    let mut sequences: Vec<Vec<i64>> = input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|i| i.parse().expect("will be number"))
                .collect()
        })
        .collect();

    let end = Instant::now();
    println!("Deserialized in: {:?}\n", end.duration_since(start));

    // println!("All sequences: {:?}\n", sequences);

    let start = Instant::now();

    for sequence in &mut sequences {
        println!("Current sequence: {:?}", sequence);
        let mut firsts: Vec<i64> = Vec::new();
        let mut subbed = sequence.clone();

        while subbed.iter().any(|&n| n != 0) {
            println!("current subbed {:?}", subbed);
            subbed = subbed.iter().enumerate().fold(Vec::new(), |mut a, (i, n)| {
                if i + 1 < subbed.len() {
                    a.push(subbed[i + 1] - n);
                }
                a
            });
            firsts.push(subbed[0]);
            println!("current firsts {:?}", firsts);
        }
        sequence.insert(0,sequence[0] - firsts.iter().fold(0, |a, c| a + c));
    }

    println!("Final sequences {:?}\n", sequences);

    let solved = sequences.iter().fold(0, |a, s| a + s[0]);

    let end = Instant::now();
    println!("Processed in: {:?}", end.duration_since(start));

    solved
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST), 114);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST), 2)
    }
}
