use std::{collections::HashMap, io::BufRead};

use lazy_static::lazy_static;

use crate::question::Question;

lazy_static! {
    static ref QUESTION: Question = Question::new(8);
}

#[allow(dead_code)]
struct Node {
    pub left: String,
    pub right: String,
}

impl Node {
    pub fn new(left: String, right: String) -> Node {
        Node { left, right }
    }
}

fn prepare() -> Option<(String, HashMap<String, Node>)> {
    // Create a buffered reader to read the file
    let mut reader = QUESTION.read_file();

    let mut instructions = String::new();
    if let Err(err) = reader.read_line(&mut instructions) {
        println!("{err:?}");
        return None;
    }

    let mut ignore_line = String::new();
    if let Err(err) = reader.read_line(&mut ignore_line) {
        println!("{err:?}");
        return None;
    }

    let mut map: HashMap<String, Node> = HashMap::new();

    for line in reader.lines() {
        match line {
            Ok(network) => {
                let value = &network[..3];
                let left = &network[7..10];
                let right = &network[12..15];
                let node = Node::new(left.to_string(), right.to_string());
                map.insert(value.to_string(), node);
            }
            Err(err) => {
                println!("{err:?}");
                return None;
            }
        }
    }
    Some((instructions, map))
}

pub fn part1() {
    let (instructions, map) = prepare().unwrap();

    let mut start = "AAA".to_string();
    let mut steps = 0;
    loop {
        for ch in instructions.chars().filter(|ch| ch.is_alphabetic()) {
            if start == "ZZZ" {
                break;
            }
            match map.get(&start) {
                Some(node) => {
                    if ch == 'L' {
                        start = node.left.clone();
                    } else if ch == 'R' {
                        start = node.right.clone();
                    }
                    steps += 1;
                }
                None => {
                    println!("No {start} in HashMap");
                    return;
                }
            }
        }
        if start == "ZZZ" {
            break;
        }
    }
    println!("Answer for day 8 part 1 is {steps}");
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u128, b: u128) -> u128 {
    a / gcd(a, b) * b
}

fn lcm_of_vec(numbers: &Vec<u128>) -> u128 {
    numbers.iter().fold(1, |lcm_acc, &num| lcm(lcm_acc, num))
}

pub fn part2() {
    let (instructions, map) = prepare().unwrap();

    let mut starts: Vec<String> = Vec::new();
    for key in map.keys() {
        if key.ends_with("A") {
            starts.push(key.clone());
        }
    }

    let mut all_steps: Vec<u128> = Vec::new();

    for mut start in starts {
        let mut steps = 0;
        loop {
            for ch in instructions.chars().filter(|ch| ch.is_alphabetic()) {
                if start.ends_with("Z") {
                    break;
                }
                match map.get(&start) {
                    Some(node) => {
                        if ch == 'L' {
                            start = node.left.clone();
                        } else if ch == 'R' {
                            start = node.right.clone();
                        }
                        steps += 1;
                    }
                    None => {
                        println!("No {start} in HashMap");
                        return;
                    }
                }
            }
            if start.ends_with("Z") {
                break;
            }
        }
        all_steps.push(steps);
    }

    println!("Answer for day 8 part 2 is {}", lcm_of_vec(&all_steps));
}
