use std::{collections::HashMap, io::BufRead};

use lazy_static::lazy_static;

use crate::question::Question;

lazy_static! {
    static ref QUESTION: Question = Question::new(4);
}

fn calculate_points(winning_numbers: Vec<u32>, given_numbers: Vec<u32>) -> u32 {
    let mut count = 0;
    for number in given_numbers {
        if winning_numbers.contains(&number) {
            count += 1;
        }
    }
    return count;
}

fn line_parsing(line_string: String) -> (Vec<u32>, Vec<u32>) {
    let parts: Vec<&str> = line_string.split('|').collect();
    let first_half: Vec<u32> = parts[0]
        .split_whitespace()
        .map(|num| num.parse::<u32>().expect("Invalid number"))
        .collect();

    let last_half: Vec<u32> = parts[1]
        .split_whitespace()
        .map(|num| num.parse::<u32>().expect("Invalid number"))
        .collect();

    (first_half, last_half)
}

pub fn part1() {
    let reader = QUESTION.read_file();
    let mut total_points: u32 = 0;
    for line in reader.lines() {
        let line_string = line.unwrap()[10..].to_string();
        let (first_half, last_half) = line_parsing(line_string);
        let count = calculate_points(first_half, last_half);
        if count != 0 {
            total_points += 2_u32.pow(count);
        }
    }
    QUESTION.print_answer(1, total_points);
}

pub fn part2() {
    let reader = QUESTION.read_file();
    let mut total_counts: u32 = 0;
    let mut card_numbers: HashMap<u32, u32> = HashMap::new();
    for line in reader.lines() {
        let card_number = line.as_ref().unwrap()[5..8]
            .to_string()
            .trim()
            .parse::<u32>()
            .expect("Invalid number");
        match card_numbers.contains_key(&card_number) {
            true => card_numbers.insert(card_number, card_numbers.get(&card_number).unwrap() + 1),
            false => card_numbers.insert(card_number, 1),
        };
        let line_string = line.unwrap()[10..].to_string();
        let (first_half, last_half) = line_parsing(line_string);
        let count = calculate_points(first_half, last_half);
        if count != 0 {
            for i in 0..count {
                *card_numbers.entry(card_number + i + 1).or_insert(0) +=
                    card_numbers.get(&card_number).unwrap().clone();
            }
        }
        total_counts += card_numbers.get(&card_number).unwrap();
    }
    QUESTION.print_answer(1, total_counts);
}
