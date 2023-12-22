use std::io::BufRead;

use lazy_static::lazy_static;

use crate::question::Question;

lazy_static! {
    static ref QUESTION: Question = Question::new(3);
}

fn preprocess() -> Vec<Vec<char>> {
    let reader = QUESTION.read_file();
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let mut row: Vec<char> = Vec::new();
        row.push('.');
        match line {
            Ok(sentence) => {
                for letter in sentence.chars() {
                    row.push(letter)
                }
            }
            Err(err) => {
                println!("{err:?}");
                return Vec::new();
            }
        }
        row.push('.');
        matrix.push(row);
    }
    let dummy_row = vec!['.'; matrix[0].len()];
    matrix.insert(0, dummy_row.clone());
    matrix.push(dummy_row);
    matrix
}

pub fn part1() {
    let matrix = preprocess();
    let mut i = 0;
    let mut answer = 0;
    while i < matrix.len() {
        let row = &matrix[i];
        let mut j = 0;
        while j < row.len() {
            let c = row[j];
            if c == '.' {
                j += 1;
                continue;
            }
            if c.is_numeric() {
                let mut number_str = c.to_string();
                let init_j = j;
                j += 1;
                while j < row.len() {
                    if row[j].is_numeric() {
                        number_str += &row[j].to_string();
                    } else {
                        break;
                    }
                    j += 1;
                }
                if (row[init_j - 1] != '.' && !row[init_j - 1].is_numeric())
                    || (row[j] != '.' && !row[j].is_numeric())
                {
                    answer += number_str.parse::<u32>().unwrap();
                } else {
                    for k in init_j - 1..j + 1 {
                        let check_up = matrix[i - 1][k];
                        let check_down = matrix[i + 1][k];
                        if (check_up != '.' && !check_up.is_numeric())
                            || (check_down != '.' && !check_down.is_numeric())
                        {
                            answer += number_str.parse::<u32>().unwrap();
                            break;
                        }
                    }
                }
            } else {
                j += 1;
            }
        }
        i += 1;
    }
    QUESTION.print_answer(1, answer);
}

pub fn part2() {
    let matrix = preprocess();
    let mut i = 0;
    let mut answer = 0;
    while i < matrix.len() {
        let row = &matrix[i];
        let mut j = 0;
        while j < row.len() {
            if row[j] == '*' {
                let mut set = Vec::from([
                    (i - 1, j - 1),
                    (i - 1, j),
                    (i - 1, j + 1),
                    (i, j - 1),
                    (i, j + 1),
                    (i + 1, j - 1),
                    (i + 1, j),
                    (i + 1, j + 1),
                ]);
                let mut adjacent_vec: Vec<u32> = Vec::new();
                while let Some((a, b)) = set.pop() {
                    if matrix[a][b].is_numeric() {
                        let mut number_str = matrix[a][b].to_string();
                        // go to left
                        let mut b_left = b - 1;
                        while b_left > 0 {
                            if matrix[a][b_left].is_numeric() {
                                number_str = matrix[a][b_left].to_string() + &number_str;
                            } else {
                                break;
                            }
                            if let Some(index) = set.iter().position(|x| *x == (a, b_left)) {
                                set.remove(index);
                            }
                            b_left -= 1;
                        }
                        // go to right
                        let mut b_right = b + 1;
                        while b_right < row.len() {
                            if matrix[a][b_right].is_numeric() {
                                number_str += &matrix[a][b_right].to_string();
                            } else {
                                break;
                            }
                            if let Some(index) = set.iter().position(|x| *x == (a, b_right)) {
                                set.remove(index);
                            }
                            b_right += 1;
                        }
                        adjacent_vec.push(number_str.parse::<u32>().unwrap());
                    }
                }
                if adjacent_vec.len() == 2 {
                    answer += adjacent_vec[0] * adjacent_vec[1];
                }
            }
            j += 1;
        }
        i += 1;
    }
    QUESTION.print_answer(2, answer);
}
