use std::collections::HashMap;
use std::time::Instant;

const INPUT: &str = include_str!("./input.txt");
const TEST: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4\
";

fn main() {
    // let solved = part_1(INPUT);
    // println!("{solved}");
    let solved = part_2(INPUT);
    println!("{solved}");
}

#[derive(Debug)]
struct Category {
    destination: String,
    ranges: Vec<Range>,
}

#[derive(Debug)]
struct Range {
    source_range_start: u128,
    destination_range_start: u128,
    range_length: u128,
}

impl Range {
    fn from(source_range_start: u128, destination_range_start: u128, range_length: u128) -> Self {
        Self {
            source_range_start,
            destination_range_start,
            range_length,
        }
    }

    fn check_source_overlap(&self, val: u128) -> bool {
        val >= self.source_range_start && val <= self.source_range_end()
    }

    fn source_range_end(&self) -> u128 {
        self.source_range_start + self.range_length - 1
    }

    fn destination_range_end(&self) -> u128 {
        self.destination_range_start + self.range_length - 1
    }
}

fn part_2(input: &str) -> u128 {
    let start = Instant::now();

    let seed_pairs: Vec<&str> = input[input.find(':').expect("first line has this char") + 1
        ..input.find('\n').expect("will have line break")]
        .split_ascii_whitespace()
        .collect();

    // println!("{:?}", seed_pairs);
    let mut seed_ranges: Vec<(u128, u128)> = Vec::new();

    for (idx, &s) in seed_pairs.iter().enumerate().step_by(2) {
        // println!("{s}");
        if idx < seed_pairs.len() - 1 {
            let seed: u128 = s.parse().unwrap();
            let peek = seed_pairs[idx + 1];
            let range: u128 = peek.parse().unwrap();
            seed_ranges.push((seed, range));
        }
    }
    // println!("seed ranges {:?}", seed_ranges);

    let mut map: HashMap<&str, Category> = HashMap::new();
    let mut sources: Vec<&str> = Vec::new();

    let mut current_source = "";
    for line in input.lines().skip(1).filter(|&l| !l.is_empty()) {
        // println!("{line}");
        if line.contains("map") {
            let mut parts = line.split('-');
            let source = parts.next().unwrap_or("");
            let _ = parts.next();
            let destination = parts.next().unwrap_or("").replace(" map:", "");

            current_source = source;
            sources.push(source);

            let category = Category {
                destination,
                ranges: Vec::new(),
            };
            map.insert(current_source, category);
        } else {
            let mut parts = line.split_ascii_whitespace();

            let destination_range_start: u128 = parts.next().unwrap().parse().unwrap();
            let source_range_start: u128 = parts.next().unwrap().parse().unwrap();
            let range_length: u128 = parts.next().unwrap().parse().unwrap();

            let category = map.get_mut(current_source).unwrap();
            category.ranges.push(Range::from(
                source_range_start,
                destination_range_start,
                range_length,
            ))
        }
    }
    // println!("{:#?}",  map);

    let end = Instant::now();
    let elapsed = end.duration_since(start);
    println!("Parsing time: {:?}", elapsed);

    let start = Instant::now();

    let mut source = "seed";
    let mut lowest_location: u128 = u128::MAX;
    let mut total_iteration_count = 0;

    for (start, range) in seed_ranges {
        for seed in start..start + range - 1 {
            total_iteration_count += 1;
            let mut mapping = seed; // will get converted from seed -> location
            while map.contains_key(source) {
                let category = map.get(source).unwrap();
                // println!("seed: {seed} cat: {:?}", category);
                for range in &category.ranges {
                    if range.check_source_overlap(mapping) {
                        // println!("source overlap found in range {:?}", range);
                        // seed is within one of the ranges, so map it to destination
                        // println!("current mapping {}", mapping);
                        // println!("destination/source offset: {}", range.destination_range_start as i128 - range.source_range_start as i128);
                        mapping =
                            mapping + range.destination_range_start - range.source_range_start;
                        // println!("{} -> {} calibration: {}", source, category.destination, mapping);
                        break;
                    }
                }
                source = &category.destination;
            }
            if lowest_location > mapping {
                lowest_location = mapping;
            }
            source = "seed";
        }
    }
    let end = Instant::now();
    let elapsed = end.duration_since(start);
    println!("Processed {} seeds in {:?}", total_iteration_count, elapsed);

    lowest_location
}

fn part_1(input: &str) -> u128 {
    let seeds: Vec<u128> = input[input.find(':').expect("first line has this char") + 1
        ..input.find('\n').expect("will have line break")]
        .split_ascii_whitespace()
        .map(|n| n.parse().expect("all nums"))
        .collect();

    let mut map: HashMap<&str, Category> = HashMap::new();
    let mut sources: Vec<&str> = Vec::new();

    let mut current_source = "";
    for line in input.lines().skip(1).filter(|&l| !l.is_empty()) {
        // println!("{line}");
        if line.contains("map") {
            let mut parts = line.split('-');
            let source = parts.next().unwrap_or("");
            let _ = parts.next();
            let destination = parts.next().unwrap_or("").replace(" map:", "");

            current_source = source;
            sources.push(source);

            let category = Category {
                destination,
                ranges: Vec::new(),
            };
            map.insert(current_source, category);
        } else {
            let mut parts = line.split_ascii_whitespace();
            let destination_range_start: u128 = parts.next().unwrap().parse().unwrap();
            let source_range_start: u128 = parts.next().unwrap().parse().unwrap();
            let range_length: u128 = parts.next().unwrap().parse().unwrap();

            let category = map.get_mut(current_source).unwrap();
            category.ranges.push(Range::from(
                source_range_start,
                destination_range_start,
                range_length,
            ))
        }
    }

    let mut lowest_location: u128 = u128::MAX;
    let mut source = "seed";

    for seed in seeds {
        let mut seed_mapping = seed;
        while map.contains_key(source) {
            let category = map.get(source).unwrap();
            for range in &category.ranges {
                if seed_mapping >= range.source_range_start
                    && seed_mapping <= range.source_range_start + range.range_length
                {
                    seed_mapping =
                        seed_mapping + range.destination_range_start - range.source_range_start;
                    break;
                }
            }
            source = &category.destination;
        }

        if lowest_location > seed_mapping {
            lowest_location = seed_mapping;
        }

        source = "seed";
    }

    lowest_location
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4\
";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(TEST), 35);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(TEST), 46);
    }
}
