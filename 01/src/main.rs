use std::fs::File;
use std::io::{self, BufRead};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "input.txt";
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // for #2
    let digits = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]; 
    let reversed_digits = ["orez", "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin"];

    let mut sum = 0;

    for line in reader.lines() {
        match line {
            Ok(text) => {
                // 1. find first and last digit 0-9
                // let first = text.chars().find(|c| c.is_digit(10)).unwrap();
                // let last = text.chars().rfind(|c| c.is_digit(10)).unwrap();
                
                // 2. find first and last digit 0-9 OR spelled out (one, two, three, etc.)
                let first = text.chars().enumerate().find_map(|(i, c)| { 
                    if c.is_digit(10) { return Some(c); };

                    // iterate over the list of digits
                    for j in 0..digits.len() {
                        // if the current digit starts with the current char
                        if digits[j].chars().nth(0).unwrap() == c {
                            // get a substring from the char position to the length of the current
                            // digit
                            if let Some(sub) = text.get(i..i + digits[j].len()) {
                                if digits[j] == sub {
                                    return Some(string_digit_to_char(sub))
                                }
                            }
                        };
                    }

                    None
                }).unwrap();

                let reversed: String = text.chars().rev().collect();
                let last = reversed.chars().enumerate().find_map(|(i, c)| { 
                    if c.is_digit(10) { return Some(c); };

                    // iterate over the list of digits
                    for j in 0..reversed_digits.len() {
                        // if the current digit starts with the current char
                        if reversed_digits[j].chars().nth(0).unwrap() == c {
                            // get a substring from the char position to the length of the current
                            // digit
                            if let Some(sub) = reversed.get(i..i + reversed_digits[j].len()) {
                                if reversed_digits[j] == sub {
                                    return Some(string_digit_to_char(sub))
                                }
                            }
                        };
                    }

                    None
                }).unwrap();

                let calibration_value = format!("{}{}", first, last).parse::<u32>().unwrap();

                sum = sum + calibration_value;
            },
            Err(err) => eprintln!("{}", err),
        }
    }

    println!("{}", sum);
    Ok(())
}

fn string_digit_to_char(digit: &str) -> char {
    match digit {
        "zero" | "orez" => '0',
        "one" | "eno" => '1',
        "two" | "owt" => '2',
        "three" | "eerht" => '3',
        "four" | "ruof" => '4',
        "five" | "evif" => '5',
        "six" | "xis" => '6',
        "seven" | "neves" => '7',
        "eight" | "thgie" => '8',
        "nine" | "enin" => '9',
        _ => 'b',
    }
}
