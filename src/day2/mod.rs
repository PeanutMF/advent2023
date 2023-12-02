use std::fs::{self, File};
use std::io::{prelude::*, Result, BufReader, Lines};
use std::str;
use regex::Regex;

const MAX_MAP : [(&str, u32); 3] = [  ("red", 12), 
                                      ("green", 13), 
                                      ("blue", 14)];

pub fn part1(file_path : &str) {
    let total : u32 = check_games(get_input_reader(file_path).expect("Failed to open input file!"), false);

    println!("Day 2 part 1 total is {total}!");
}

pub fn part2(file_path : &str) {
    let total : u32 = check_games(get_input_reader(file_path).expect("Failed to open input file!"), true);

    println!("Day 2 part 2 total is {total}!");
}

fn get_input_reader(file_path : &str) -> Result<Lines<BufReader<File>>> {
    let file = fs::File::open(file_path)?;
    return Ok(BufReader::new(file).lines());
}

fn check_games(input_reader : Lines<BufReader<File>>, is_part2 : bool) -> u32 {
    let mut total = 0;
    for line_str in input_reader {
        if let Ok(val) = line_str {
            total += if !is_part2 {check_game_part1(val) } else {check_game_part2(val)};
        }
    }

    return total;
}

fn check_game_part1(line : String) -> u32 {
    let pull_regex : Regex = Regex::new(r"(\d+)\s([\w]+)").unwrap();
    let game_capture: (&str, [&str; 1]) = Regex::new(r"Game\s(\d+):\s").unwrap().captures(&line).unwrap().extract();
    let game_id = game_capture.1[0].parse::<u32>().expect("Failed to parse matched number for game id");
    let slice = &line[game_capture.0.len()..line.len()];

    for pull in slice.split([',', ';']) {
        let pull_captures: (&str, [&str; 2]) = pull_regex.captures(pull).unwrap().extract();
        let pull_colour = pull_captures.1[1];
        let pull_value = pull_captures.1[0].parse::<u32>().expect("Failed to parse matched number for cube count");

        match pull_colour {
            "red"   =>  if pull_value > MAX_MAP[0].1 { return 0; }
            "green" =>  if pull_value > MAX_MAP[1].1 { return 0;  }
            "blue"  =>  if pull_value > MAX_MAP[2].1 { return 0;  }
            _ => { eprintln!("Found unknown colour!"); return 0}
        }
    }

    return game_id;
}

fn check_game_part2(line : String) -> u32 {
    let pull_regex : Regex = Regex::new(r"(\d+)\s([\w]+)").unwrap();
    let game_capture: (&str, [&str; 1]) = Regex::new(r"Game\s(\d+):\s").unwrap().captures(&line).unwrap().extract();
    let slice = &line[game_capture.0.len()..line.len()];

    let mut red_max = 0;
    let mut green_max = 0;
    let mut blue_max = 0;

    for pull in slice.split([',', ';']) {
        let pull_captures: (&str, [&str; 2]) = pull_regex.captures(pull).unwrap().extract();
        let pull_colour = pull_captures.1[1];
        let pull_value = pull_captures.1[0].parse::<u32>().expect("Failed to parse matched number for cube count");

        match pull_colour {
            "red"   =>  if pull_value > red_max { red_max = pull_value; }
            "green" =>  if pull_value > green_max { green_max = pull_value;  }
            "blue"  =>  if pull_value > blue_max { blue_max = pull_value;  }
            _ => { eprintln!("Found unknown colour!"); return 0}
        }
    }

    return red_max * green_max * blue_max;
}