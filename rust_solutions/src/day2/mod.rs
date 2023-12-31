use std::{collections::HashMap, io::BufRead};

use lazy_static::lazy_static;

use crate::question::Question;

lazy_static! {
    static ref QUESTION: Question = Question::new(2);
}

fn parse_line(line: String) -> Vec<HashMap<String, u32>> {
    match line.find(':') {
        Some(idx) => {
            let mut result: Vec<HashMap<String, u32>> = Vec::new();
            for set in line[idx + 2..].split("; ") {
                let mut map = HashMap::new();
                for kind in set.split(", ") {
                    let mut value = kind.split(" ");
                    let num = value.next().unwrap();
                    let color = value.next().unwrap();
                    map.insert(color.to_string(), u32::from_str_radix(num, 10).unwrap());
                }
                result.push(map);
            }
            result
        }
        None => vec![],
    }
}

pub fn part1() {
    // Create a buffered reader to read the file
    let reader = QUESTION.read_file();

    // Max value of each colors
    let standard = HashMap::from([
        ("red".to_string(), 12),
        ("green".to_string(), 13),
        ("blue".to_string(), 14),
    ]);

    // Read the file line by line
    let mut game_idx = 1;
    let mut result = 0;
    for line in reader.lines() {
        match line {
            Ok(str) => {
                let mut success = true;
                for map in parse_line(str) {
                    for (color, value) in &map {
                        if !standard.get(color).is_some_and(|num| num >= value) {
                            success = false;
                        }
                    }
                }
                if success {
                    result += game_idx;
                }
            }
            Err(err) => println!("{err:?}"),
        }
        game_idx += 1;
    }
    QUESTION.print_answer(1, result);
}

pub fn part2() {
    // Create a buffered reader to read the file
    let reader = QUESTION.read_file();

    // Read the file line by line
    let mut result = 0;
    for line in reader.lines() {
        let mut max_values: HashMap<String, u32> = HashMap::from([
            ("red".to_string(), 0),
            ("green".to_string(), 0),
            ("blue".to_string(), 0),
        ]);
        match line {
            Ok(str) => {
                for map in parse_line(str) {
                    for (color, value) in &map {
                        max_values.insert(
                            color.to_string(),
                            std::cmp::max(*value, *max_values.get(color).unwrap()),
                        );
                    }
                }
            }
            Err(err) => println!("{err:?}"),
        }
        result += {
            let mut sub_result = 1;
            for value in max_values.values() {
                sub_result *= value;
            }
            sub_result
        }
    }
    QUESTION.print_answer(2, result);
}
