// Puzzle link: https://adventofcode.com/2023/day/1

use std::fs;

const FILENAME: &str = "input.txt";

const RED_MAX: u8 = 12;
const GREEN_MAX: u8 = 13;
const BLUE_MAX: u8 = 14;

fn read_text_from_file() -> String {
    fs::read_to_string(FILENAME).expect("Failed to open file")
}

fn check_line_validity(line: &str) -> bool {
    return true;
}

fn main() {
    let text_as_string = read_text_from_file();
    let mut lines = text_as_string.lines();
    let mut optional_line = lines.next();

    let mut valid: bool = true;
    while valid && optional_line.is_some() {
        valid = check_line_validity(optional_line.unwrap());

        optional_line = lines.next();
    }
}
