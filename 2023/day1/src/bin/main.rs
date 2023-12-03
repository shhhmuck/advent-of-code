const INPUT: &str = include_str!("../../data/input.txt");
const RADIX: u32 = 10;
const DIGITS_AS_STR: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn main() {
    let sum_of_calibrations = solve(INPUT);
    println!("{}", sum_of_calibrations);
}

fn solve(input: &str) -> u32 {
    input.lines().fold(0, |sum, line| {
        let chars: Vec<char> = line.chars().collect();
        sum + get_calibration_value(&chars)
    })
}

fn get_calibration_value(chars: &[char]) -> u32 {
    let mut first_num = None;
    let mut second_num = None;

    let mut start_ptr = 0;
    let mut end_ptr = chars.len() - 1;

    let mut forward_string = String::new();
    let mut backward_string = String::new();

    while !chars[start_ptr].is_numeric() {
        forward_string.push(chars[start_ptr]);

        for (digit_as_str, digit) in DIGITS_AS_STR {
            if forward_string.contains(digit_as_str) {
                first_num = Some(digit);
                break;
            }
        }
        if first_num.is_some() {
            break;
        }
        start_ptr += 1;
    }

    while !chars[end_ptr].is_numeric() {
        backward_string.push(chars[end_ptr]);

        for (digit_as_str, digit) in DIGITS_AS_STR {
            if backward_string
                .chars()
                .rev()
                .collect::<String>()
                .contains(digit_as_str)
            {
                second_num = Some(digit);
                break;
            }
        }
        if second_num.is_some() {
            break;
        }
        end_ptr -= 1;
    }

    if first_num.is_none() {
        first_num = Some(chars[start_ptr].to_digit(RADIX).unwrap());
    }

    if second_num.is_none() {
        second_num = Some(chars[end_ptr].to_digit(RADIX).unwrap());
    }

    first_num.unwrap() * 10 + second_num.unwrap()
}

#[cfg(test)]
mod test_get_calibration_value {
    use super::*;
    #[test]
    fn test_numberic_only() {
        let test_input: &str = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        let expected_calibration_values = [12, 38, 15, 77];

        for (index, line) in test_input.lines().enumerate() {
            let chars: Vec<char> = line.chars().collect();
            assert_eq!(
                get_calibration_value(&chars),
                expected_calibration_values[index]
            )
        }
    }
}
