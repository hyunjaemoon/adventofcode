use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

fn read_file() -> BufReader<File> {
    // Open the file for reading
    let mut file_dir = PathBuf::new();
    file_dir.push("src");
    file_dir.push("day3");
    file_dir.push("input.txt");
    let file = File::open(file_dir).expect("Failed to open file");
    BufReader::new(file)
}

pub fn part1() {
    // Create a buffered reader to read the file
    let reader = read_file();

    for line in reader.lines() {
        println!("{line:?}");
    }
}
