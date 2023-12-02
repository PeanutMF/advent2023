use std::fs;
use std::io::{prelude::*, Result, BufReader, Lines};

const STR_DIGIT : [(u32, &str); 9] = [  (1, "one"), 
                                        (2, "two"), 
                                        (3, "three"), 
                                        (4, "four"), 
                                        (5, "five"), 
                                        (6, "six"), 
                                        (7, "seven"), 
                                        (8, "eight"), 
                                        (9, "nine")]; 


pub fn part1(file_path : &str) {
    let total : u32 = count_lines(get_input_reader(file_path).unwrap(), false);
    println!("Day 1 part 1 total is {total}!");
}

pub fn part2(file_path : &str) {
    let total : u32 = count_lines(get_input_reader(file_path).unwrap(), true);
    println!("Day 1 part 2 total is {total}!");
}

fn get_input_reader(file_path : &str) -> Result<Lines<BufReader<fs::File>>> {
    let file = fs::File::open(file_path)?;
    return Ok(BufReader::new(file).lines());
}

fn count_lines(input_reader : Lines<BufReader<fs::File>>, is_part2 : bool) -> u32 {
    let mut total = 0;
    for line in input_reader {
        if let Ok(val) = line {
                total += if !is_part2 { count_line_part1(val) } else { count_line_part2(val) };
        }
    }

    return total;
}

fn count_line_part1(line : String) -> u32 {

    let mut found_first : bool = false;
    let mut first : u32 = 0;
    let mut last : u32 = 0;

    for character in line.chars() {
        if character.is_digit(10) {
            let digit = character.to_digit(10).expect("Converting character is not a digit!");
            if !found_first {
                found_first = true;
                first = digit;
            }
            last = digit;
        }
    }
    let total = (first * 10) + last;
    return total;

}

fn count_line_part2(line : String) -> u32 {

    let mut found_first : bool = false;
    let mut first : u32 = 0;
    let mut last : u32 = 0;

    for index in 0..line.len() {
        let slice = &line[index..line.len()];
        let mut digit : u32 = 0;
        let mut found : bool = false;
        
        for pair in STR_DIGIT {
            if slice.starts_with(pair.1) {
                digit = pair.0;
                found = true;
            }
        }

        if !found {
            let character = slice.chars().nth(0).unwrap();

            if character.is_digit(10) {
                digit = character.to_digit(10).expect("Converting character is not a digit!");
                found = true;
            }
        }

        if found {
            if !found_first {
                found_first = true;
                first = digit;
            }

            last = digit;
        }
    }

    let total = (first * 10) + last;
    return total;
}
