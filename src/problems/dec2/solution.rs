use std::cmp::PartialEq;
use std::fs;

#[derive(PartialEq, Clone, Copy)]
enum Status {
    First,
    Second,
    Increasing,
    Decreasing,
}

fn is_line_valid(values: &Vec<i128>) -> bool {
    let mut valid = true;
    let mut current_status = Status::First;
    let mut prev_number = 0i128;
    for value in values {
        let numeric_value = *value;
        if current_status == Status::First {
            current_status = Status::Second;
            prev_number = numeric_value;
            continue;
        }
        if current_status == Status::Second {
            current_status = if numeric_value > prev_number { Status::Increasing } else { Status::Decreasing };
        }

        let difference = numeric_value - prev_number;
        valid = if current_status == Status::Increasing {
            difference > 0 && difference <= 3
        } else {
            difference < 0 && difference >= -3
        };

        if !valid {
            break;
        }
        prev_number = numeric_value;
    }
    valid
}

pub fn solution1(file: &str) -> i128 {
    let file = fs::read_to_string(file).expect("Could not read file");
    let total_count = file.lines().fold(0, |acc, line| {
        let vec = line.split_whitespace()
            .collect::<Vec<&str>>().iter()
            .map(|&x| x.parse::<i128>().unwrap())
            .collect::<Vec<i128>>();
        if is_line_valid(&vec) {
            return acc + 1;
        }
        acc
    });
    total_count
}

pub fn process_vectors_with_missing_elements(vec: &Vec<i128>) -> bool {
    for i in 0..vec.len() {
        let mut smaller_vec = vec![];
        for i2 in 0..vec.len() {
            if i == i2 { continue; }
            else { smaller_vec.push(vec[i2]); }
        }
        if is_line_valid(&smaller_vec) {
            return true
        }
    }
    false
}

// Brute force solution.
pub fn solution2(file: &str) -> i128 {
    let file = fs::read_to_string(file).expect("Could not read file");
    let mut counter = 0;
    for line in file.lines() {
        let vec = line.split_whitespace()
            .collect::<Vec<&str>>().iter()
            .map(|&x| x.parse::<i128>().unwrap())
            .collect::<Vec<i128>>();
        if is_line_valid(&vec) {
            counter += 1;
            continue;
        }
        if process_vectors_with_missing_elements(&vec) {
            counter += 1;
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use crate::problems::dec2::solution::solution1;
    use crate::problems::dec2::solution::solution2;
    #[test]
    fn test_solution1() {
        let file_path = "resources/dec2.txt";
        let response = solution1(file_path);
        let answer = 598;
        assert_eq!(response, answer);
    }

    #[test]
    fn test_solution2() {
        let file_path = "resources/dec2.txt";
        let response = solution2(file_path);
        let answer = 634;
        assert_eq!(response, answer);
    }
}