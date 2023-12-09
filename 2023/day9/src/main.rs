use std::time::Instant;

const INPUT: &str = include_str!("./input.txt");
const TEST: &str = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45\
";

fn main() {
    let solved = part_2(INPUT);
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
        // println!("Current sequence: {:?}", sequence);

        let mut firsts: Vec<i64> = Vec::new();
        let mut subbed = sequence.clone();

        firsts.push(subbed[0]);

        while subbed.iter().any(|&n| n != 0) {
            subbed = subbed.iter().enumerate().fold(Vec::new(), |mut a, (i, n)| {
                if i + 1 < subbed.len() {
                    a.push(subbed[i + 1] - n);
                }
                a
            });
            firsts.push(subbed[0]);
            // println!("current subbed {:?}", subbed);
            // println!("current firsts {:?}", firsts);
        }

        let mut idx = firsts.len() - 1;
        let mut prev: i64 = firsts[idx];

        for i in (0..firsts.len()).rev() {
            // println!("idx:{i} prev:{prev}");
            // current firsts [3, 0, 2, 0]    0 + what = 2 (2)  2 + what = 0 ? (-2) -2 + what = 3? (5)
            if i > 0 {
                // println!("{}-{}", firsts[i - 1], prev);
                prev = firsts[i - 1] - prev;
            }
        }

        sequence.insert(0, prev);
    }

    // println!("Final sequences {:?}\n", sequences);

    let solved = sequences.iter().fold(0, |a, s| a + s[0]);

    let end = Instant::now();
    println!("Processed in: {:?}", end.duration_since(start));

    solved
}

/*
0 1 3 6 10 15 21
 0  2 3 4  5  6
   1 1 1  1  1
    0  0   0  0
*/

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
