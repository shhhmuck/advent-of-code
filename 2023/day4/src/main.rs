use std::collections::HashMap;

const INPUT: &str = include_str!("./input.txt");

const PART_1_TEST_INPUT: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\
";

const PART_2_TEST_INPUT: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\
";

fn main() {
    // let solved_1 = part_1(INPUT);
    // println!("{solved_1}");
    let solved_2 = part_2(PART_2_TEST_INPUT);
    println!("{solved_2}");
}

fn part_1(input: &str) -> u32 {
    let mut sum = 0;
    let lines: Vec<&str> = input.lines().collect();
    for line in lines {
        let mut line_worth: u32 = 0;

        let line: String = line.chars().collect();
        let line = line[(line.find(':').expect("line will have :") + 1)..].to_string();

        let split: Vec<&str> = line.split('|').collect();

        let winners: Vec<&str> = split[0].split_ascii_whitespace().collect();
        let numbers: Vec<&str> = split[1].split_ascii_whitespace().collect();

        for number in numbers {
            if winners.contains(&number) {
                if line_worth == 0 {
                    line_worth += 1;
                } else {
                    line_worth += line_worth;
                }
            }
        }
        sum += line_worth;
    }

    sum
}

fn part_2(input: &str) -> u32 {
    let mut card_map: HashMap<usize, u32> = HashMap::new();
    
    let lines: Vec<&str> = input.lines().collect();
    let mut idx = 0;

    while idx < lines.len() {
        let card = card_map.entry(idx).or_insert(1);
        
        let line = lines[idx];

        let mut wins: u32 = 0;

        let line: String = line.chars().collect();
        let line = line[(line.find(':').expect("line will have :") + 1)..].to_string();

        let split: Vec<&str> = line.split('|').collect();

        let winners: Vec<&str> = split[0].split_ascii_whitespace().collect();
        let numbers: Vec<&str> = split[1].split_ascii_whitespace().collect();

        for number in numbers {
            if winners.contains(&number) {
                wins += 1;
            }
        }
        let wins: usize = wins.try_into().expect("will fit into usize");

        if wins != 0 {
            for i in idx + 1..=idx + wins + 1 {
                let i = i as usize;
                let entry = card_map.entry(i).or_default();
                *entry += 1;
            }
        }

        println!("index {} :: wins {} :: map {:?}", idx, wins, card_map);

        idx += 1;
    }

    0
}

// fn part_2(input: &str) -> u32 {
//     let mut total_cards = 0;
//     let mut card_map: HashMap<usize, u32> = HashMap::new();
    
//     let lines: Vec<&str> = input.lines().collect();
//     let mut idx = lines.len() - 1;

//     while idx >= 0 {

        
//         let line = lines[idx];

//         let mut wins: u32 = 0;
//         let mut card_count = 1;

//         let line: String = line.chars().collect();
//         let line = line[(line.find(':').expect("line will have :") + 1)..].to_string();

//         let split: Vec<&str> = line.split('|').collect();

//         let winners: Vec<&str> = split[0].split_ascii_whitespace().collect();
//         let numbers: Vec<&str> = split[1].split_ascii_whitespace().collect();

//         for number in numbers {
//             if winners.contains(&number) {
//                 wins += 1;
//             }
//         }
//         let wins: usize = wins.try_into().expect("will fit into usize");

//         if wins != 0 {
//             for i in idx + 1..=idx + wins + 1 {
//                 let i = i as usize;
//                 card_count += card_map
//                     .get(&i)
//                     .expect("Cards will never make you copy a card past the end of the table.")
//             }
//         }

//         println!("index {} :: count {}", idx, card_count);

//         card_map.insert(idx, card_count);
//         total_cards += card_count;

//         println!("{:?}", card_map);

//         if idx == 0 {
//             break;
//         }
//         idx -= 1;
//     }

//     total_cards
// }


