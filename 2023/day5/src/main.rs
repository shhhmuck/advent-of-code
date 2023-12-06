use std::collections::HashMap;

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
    let solved = part_1(INPUT);
    println!("{solved}");
}

#[derive(Debug)]
struct Category {
    destination: String,
    ranges: Vec<Range>,
}

#[derive(Debug)]
struct Range {
    source_range_start: u64,
    destination_range_start: u64,
    range_length: u64,
}

impl Range {
    fn from(source_range_start: u64, destination_range_start: u64, range_length: u64) -> Self {
        Self {
            source_range_start,
            destination_range_start,
            range_length,
        }
    }
}

fn part_1(input: &str) -> u64 {
    let seeds: Vec<u64> = input[input.find(":").expect("first line has this char") + 1
        ..input.find("\n").expect("will have line break")]
        .split_ascii_whitespace()
        .map(|n| n.parse().expect("all nums"))
        .collect();

    let mut map: HashMap<&str, Category> = HashMap::new();
    let mut sources: Vec<&str> = Vec::new();

    let mut current_source = "";
    for line in input.lines().skip(1).filter(|&l| !l.is_empty()) {
        // println!("{line}");
        if line.contains("map") {
            let mut parts = line.split("-");
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
            let destination_range_start: u64 = parts.next().unwrap().parse().unwrap();
            let source_range_start: u64 = parts.next().unwrap().parse().unwrap();
            let range_length: u64 = parts.next().unwrap().parse().unwrap();

            let category = map.get_mut(current_source).unwrap();
            category.ranges.push(Range::from(
                source_range_start,
                destination_range_start,
                range_length,
            ))
        }
    }

    let mut lowest_location: u64 = u64::MAX;
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
