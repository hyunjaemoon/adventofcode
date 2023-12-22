use std::collections::HashMap;
use std::io::BufRead;

use lazy_static::lazy_static;

use crate::question::Question;

lazy_static! {
    static ref QUESTION: Question = Question::new(1);
}

fn find_calibration_value(word: String) -> u32 {
    let mut first_digit: Option<char> = None;
    let mut last_digit: Option<char> = None;
    for ch in word.chars() {
        if ch.is_digit(10) {
            first_digit = Some(ch);
            break;
        }
    }
    for ch in word.chars().rev() {
        if ch.is_digit(10) {
            last_digit = Some(ch);
            break;
        }
    }
    match (first_digit, last_digit) {
        (Some(first_num), Some(last_num)) => {
            u32::from_str_radix(format!("{first_num}{last_num}").as_str(), 10).unwrap()
        }
        _ => 0,
    }
}

fn find_calibration_value_with_string(word: String) -> u32 {
    let mut first_digit: Option<char> = None;
    let mut last_digit: Option<char> = None;
    let word_map: HashMap<&str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);
    for (idx, ch) in word.char_indices() {
        if ch.is_digit(10) {
            first_digit = Some(ch);
            break;
        }
        for (key, value) in &word_map {
            if word[idx..].starts_with(key) {
                first_digit = Some(value.clone());
                break;
            }
        }
        if first_digit.is_some() {
            break;
        }
    }
    for (idx, ch) in word.char_indices().rev() {
        if ch.is_digit(10) {
            last_digit = Some(ch);
            break;
        }
        for (key, value) in &word_map {
            if word[..(idx + 1)].ends_with(key) {
                last_digit = Some(value.clone());
                break;
            }
        }
        if last_digit.is_some() {
            break;
        }
    }
    match (first_digit, last_digit) {
        (Some(first_num), Some(last_num)) => {
            u32::from_str_radix(format!("{first_num}{last_num}").as_str(), 10).unwrap()
        }
        _ => 0,
    }
}

pub fn part1() {
    // Create a buffered reader to read the file
    let reader = QUESTION.read_file();

    // Read the file line by line
    let mut answer = 0;
    for line in reader.lines() {
        match line {
            Ok(str) => {
                answer += find_calibration_value(str);
            }
            Err(err) => println!("{err:?}"),
        }
    }
    QUESTION.print_answer(1, answer);
}

pub fn part2() {
    // Create a buffered reader to read the file
    let reader = QUESTION.read_file();

    // Read the file line by line
    let mut answer = 0;
    for line in reader.lines() {
        match line {
            Ok(str) => {
                answer += find_calibration_value_with_string(str);
            }
            Err(err) => println!("{err:?}"),
        }
    }
    QUESTION.print_answer(2, answer);
}
