use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn answer() {
    // Fetch input22.txt file into Vec<u32>
    let path = Path::new("src/day22/input22.txt");
    let file = File::open(&path).expect("Unable to open file");
    let reader = io::BufReader::new(file);
    let input: Vec<u32> = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    /*
    In particular, each buyer's secret number evolves into the next secret number in the sequence via the following process:
    - Calculate the result of multiplying the secret number by 64. Then, mix this result into the secret number. Finally, prune the secret number.
    - Calculate the result of dividing the secret number by 32. Round the result down to the nearest integer. Then, mix this result into the secret number. Finally, prune the secret number.
    - Calculate the result of multiplying the secret number by 2048. Then, mix this result into the secret number. Finally, prune the secret number.
    
    Each step of the above process involves mixing and pruning:
    - To mix a value into the secret number, calculate the bitwise XOR of the given value and the secret number. Then, the secret number becomes the result of that operation. (If the secret number is 42 and you were to mix 15 into the secret number, the secret number would become 37.)
    - To prune the secret number, calculate the value of the secret number modulo 16777216. Then, the secret number becomes the result of that operation. (If the secret number is 100000000 and you were to prune the secret number, the secret number would become 16113920.)
    */

    todo!("Part 1: {input:?}");
}
