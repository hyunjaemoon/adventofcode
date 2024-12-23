use std::{fs::File, io::BufReader, path::PathBuf};

pub struct Question {
    day: u32,
}

impl Question {
    pub fn new(day: u32) -> Question {
        Question { day }
    }

    pub fn read_file(&self) -> BufReader<File> {
        // Open the file for reading
        let mut file_dir = PathBuf::from("src");
        file_dir.push(format!("day{}", self.day));
        file_dir.push("input.txt");
        BufReader::new(File::open(file_dir).expect("Failed to open file"))
    }

    pub fn print_answer(&self, part: u32, answer: u32) {
        println!("Answer for day {} part {} is {}", self.day, part, answer);
    }
}
