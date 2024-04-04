use std::fs;

const FILENAME: &str = "input.txt";
const UNSET_DIGIT: u32 = 11;

fn main() {
    let raw_text: String = fs::read_to_string(FILENAME).expect("Failed to open file");

    let mut lines = raw_text.lines();
    let mut optional_line = lines.next();
    let mut accumulator = 0;
    while optional_line.is_some() {
        let line = optional_line.unwrap();
        println!("parsing {}", line);

        let mut first_digit = UNSET_DIGIT;
        let mut last_digit = UNSET_DIGIT;

        for character in line.chars() {
            if character.is_digit(10) {
                let digit = character.to_digit(10);
                if first_digit == UNSET_DIGIT {
                    first_digit = digit.expect("Not a digit");
                }

               last_digit = digit.expect("Not a digit"); 
            }
        }

        let new_number = first_digit * 10 + last_digit;

        println!("adding {} to accumulator", new_number);
        accumulator += new_number;

        optional_line = lines.next();
    }

    println!("total: {}", accumulator);
}
