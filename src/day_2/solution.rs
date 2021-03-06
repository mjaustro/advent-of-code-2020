use crate::common;

use std::io::Error;

pub fn part_1() -> Result<i32, Error>  {
    let lines = common::read_input_for_day(2);

    let mut valid_passwords = 0;
    for i in 0..lines.len() {
        // Parse line
        let line: Vec<&str> = lines[i].split(": ").collect();
        let policy: Vec<&str> = line[0].split(" ").collect();
        let lengths: Vec<&str> = policy[0].split("-").collect();
        let min_length = lengths[0].parse::<i32>().unwrap();
        let max_length = lengths[1].parse::<i32>().unwrap();
        let letter = policy[1].parse::<char>().unwrap();
        let password = line[1];

        let mut letter_count = 0;
        for p in password.chars() {
            if p == letter {
                letter_count += 1;
            }
        }
        if min_length <= letter_count && letter_count <= max_length {
            valid_passwords += 1;
        }
    }
    
    return Ok(valid_passwords);
}

pub fn part_2() -> Result<i32, Error>  {
    let lines = common::read_input_for_day(2);
    
    let mut valid_passwords = 0;
    for i in 0..lines.len() {
        // Parse line
        let line: Vec<&str> = lines[i].split(": ").collect();
        let policy: Vec<&str> = line[0].split(" ").collect();
        let lengths: Vec<&str> = policy[0].split("-").collect();
        let position_1 = lengths[0].parse::<usize>().unwrap();
        let position_2 = lengths[1].parse::<usize>().unwrap();
        let letter = policy[1].parse::<char>().unwrap();
        let password = line[1];
        
        let letter_1 = password.chars().nth(position_1 - 1).unwrap();
        let letter_2 = password.chars().nth(position_2 - 1).unwrap();
        if letter_1 == letter || letter_2 == letter {
            if !(letter_1 == letter && letter_2 == letter) {
                valid_passwords += 1;
            }
        }

    }
    
    return Ok(valid_passwords);
}
