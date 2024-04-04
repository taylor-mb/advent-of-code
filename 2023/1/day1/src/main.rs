// Puzzle link: https://adventofcode.com/2023/day/1

use std::fs;

const FILENAME: &str = "input.txt";
const UNSET: u32 = std::u32::MAX;
const NUMBERS:  [&str;19] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn read_text_from_file() -> String {
    fs::read_to_string(FILENAME).expect("Failed to open file")
}

fn string_to_number(string: &str) -> u32 {
    match string {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("Unrecognized number {}", string)
    }
}

fn calculate_line_value(line: &str) -> u32 {
        println!("parsing {}", line);

        let mut first_digit = UNSET;
        let mut last_digit = UNSET;

        let mut first_index = UNSET;
        let mut last_index = UNSET;

        for candidate in NUMBERS {
            let search_result: Vec<_> = line.match_indices(candidate).collect();
            if !search_result.is_empty() {
                // Find the index of the first occurrence of the number
                let mut index = search_result.first().unwrap().0 as u32;

                if (first_index == UNSET) || (index < first_index) {
                    first_index = index;

                    let parse_result = candidate.parse::<u32>();
                    if parse_result.is_ok() {
                        first_digit = parse_result.unwrap();
                    } else {
                        first_digit = string_to_number(candidate);
                    }
                }

                // Find the index of the last occurrence of the number
                index = search_result.last().unwrap().0 as u32;
                if (last_digit == UNSET) || (index > last_index) {
                    last_index = index;

                    let parse_result = candidate.parse::<u32>();
                    if parse_result.is_ok() {
                        last_digit = parse_result.unwrap();
                    } else {
                        last_digit = string_to_number(candidate);
                    }
                }
            }
        }
        first_digit * 10 + last_digit
}

fn main() {
    let raw_text = read_text_from_file();
    let mut lines = raw_text.lines();

    let mut accumulator = 0;

    let mut optional_line = lines.next();
    while optional_line.is_some() {
        let line = optional_line.unwrap();
        let line_value = calculate_line_value(line);

        println!("Adding {} to accumulator", line_value);
        accumulator += line_value;

        optional_line = lines.next();
    }

    println!("Total: {}", accumulator);
}
