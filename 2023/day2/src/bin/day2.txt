const INPUT: &str = include_str!("../../data/input.txt");

#[derive(Debug, PartialEq, Eq)]
struct Game {
    game_id: u16,
    draws: Vec<Draw>,
}

impl Game {
    pub fn is_game_possible(&self, red_count: u8, blue_count: u8, green_count: u8) -> bool {
        self.draws.iter().all(|draw| {
            draw.red <= red_count && draw.blue <= blue_count && draw.green <= green_count
        })
    }

    pub fn find_game_power(&self) -> u64 {
        let min_draw = self.find_min_draw();
        // println!("{:#?}", min_draw);
        let power = min_draw.blue as u64 * min_draw.green as u64 * min_draw.red as u64;
        // println!("Game power: {}", power);
        power
    }

    fn find_min_draw(&self) -> Draw {
        let mut min_draw = Draw::default();
        self.draws.iter().for_each(|draw| {
            if draw.red > min_draw.red {
                min_draw.red = draw.red;
            }
            if draw.blue > min_draw.blue {
                min_draw.blue = draw.blue
            }
            if draw.green > min_draw.green {
                min_draw.green = draw.green
            }
        });
        min_draw
    }
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Draw {
    blue: u8,
    red: u8,
    green: u8,
}

impl Draw {
    pub fn new(blue: u8, red: u8, green: u8) -> Self {
        Self { blue, red, green }
    }
}

fn main() {
    // let solved = solve(INPUT);
    let solved = solve_part_2(INPUT);
    println!("{solved}");
}

fn solve_part_2(input: &str) -> u64 {
    let mut sum = 0;
    let games = parse_games(input);
    // println!("{:#?}", games);
    for game in games {
        sum += game.find_game_power();
    }
    sum
}

fn solve_part_1(input: &str) -> u16 {
    let mut sum = 0;
    let games = parse_games(input);
    for game in games {
        if game.is_game_possible(12, 14, 13) {
            sum += game.game_id;
        }
    }
    sum
}

fn parse_games(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| Game {
            game_id: get_game_id(line),
            draws: get_game_draws(line),
        })
        .collect()
}

// for a given game, get the draws
fn get_game_draws(line: &str) -> Vec<Draw> {
    let mut draws: Vec<Draw> = Vec::new();

    let split: Vec<&str> = line.split(':').collect();
    let game: &str = split[1];

    let draw_splits: Vec<&str> = game.split(';').collect();

    for draw_split in draw_splits {
        let mut draw = Draw::default();

        let cubes: Vec<&str> = draw_split.split(',').collect();

        for cube in cubes {
            let cube = cube.trim();
            let cube_split: Vec<&str> = cube.split(' ').collect();

            let count: u8 = cube_split[0].parse().unwrap();

            match cube_split[1] {
                "green" => {
                    draw.green += count;
                }
                "red" => {
                    draw.red += count;
                }
                "blue" => {
                    draw.blue += count;
                }
                _ => unreachable!(),
            };
        }
        draws.push(draw);
    }
    draws
}

fn get_game_id(line: &str) -> u16 {
    let line = line.replace("Game ", "");
    let game_id_str = line.split(':').next().unwrap();
    game_id_str.parse().expect("will be a number at this spot")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_game_power() {
        let game = Game {
            game_id: 1,
            draws: vec![Draw::new(5, 5, 5), Draw::new(2, 2, 2)],
        };
        let game2 = Game {
            game_id: 1,
            draws: vec![Draw::new(2, 2, 2), Draw::new(5, 5, 5)],
        };

        assert_eq!(game.find_game_power(), 125);
        assert_eq!(game2.find_game_power(), 125);
    }

    #[test]
    fn test_is_game_possible() {
        let game = Game {
            game_id: 1,
            draws: vec![Draw::new(5, 5, 5)],
        };

        assert!(!game.is_game_possible(1, 1, 1));
        assert!(!game.is_game_possible(4, 4, 4));
        assert!(game.is_game_possible(5, 5, 5));
        assert!(game.is_game_possible(12, 7, 7));
    }

    #[test]
    fn test_parse_games() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        let expected = vec![Game {
            game_id: 1,
            draws: vec![Draw::new(3, 4, 0), Draw::new(6, 1, 2), Draw::new(0, 0, 2)],
        }];

        assert_eq!(parse_games(input), expected)
    }

    #[test]
    fn test_get_game_id() {
        let test1 = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let test2 = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let test3 = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(get_game_id(test1), 1);
        assert_eq!(get_game_id(test2), 2);
        assert_eq!(get_game_id(test3), 5);
    }

    #[test]
    fn test_get_game_draws() {
        let test1 = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let test2 = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let test3 = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(
            get_game_draws(test1),
            vec![Draw::new(3, 4, 0), Draw::new(6, 1, 2), Draw::new(0, 0, 2)]
        );

        assert_eq!(
            get_game_draws(test2),
            vec![Draw::new(1, 0, 2), Draw::new(4, 1, 3), Draw::new(1, 0, 1)]
        );

        assert_eq!(
            get_game_draws(test3),
            vec![Draw::new(1, 6, 3), Draw::new(2, 1, 2)]
        );
    }
}
