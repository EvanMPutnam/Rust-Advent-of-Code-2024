use std::fs;
use regex::Regex;

fn process_mult_instruction(value: &str) -> i128 {
    let pieces = value.split(',').collect::<Vec<&str>>();
    let left_value = *pieces.get(0).unwrap()
        .split('(')
        .collect::<Vec<&str>>()
        .get(1).unwrap();
    let right_value = *pieces.get(1).unwrap()
        .split(')')
        .collect::<Vec<&str>>()
        .get(0).unwrap();
    left_value.parse::<i128>().unwrap() * right_value.parse::<i128>().unwrap()
}

pub fn solution1(file: &str) -> i128 {
    // Just get the multiplication regex.
    let regex_str = r"mul\(\d{1,3},\d{1,3}\)"; // mul(xxx,xxx)
    let file = fs::read_to_string(file).expect("Could not read file");
    let regex = Regex::new(regex_str).unwrap();
    let mut result = 0i128;
    for reg_match in regex.find_iter(&file) {
        let mul_str = reg_match.as_str();
        result += process_mult_instruction(mul_str);
    }
    result
}

pub fn solution2(file: &str) -> i128 {
    // This is not super sustainable but since its only 3 elements in the grammar I
    // really don't feel like building out anything more complex.
    let regex_str = r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)"; // mul(xxx,xxx) | do() | don't()
    let file = fs::read_to_string(file).expect("Could not read file");
    let regex = Regex::new(regex_str).unwrap();
    let mut result = 0i128;
    let mut enabled = true;
    for reg_match in regex.find_iter(&file) {
        let mul_str = reg_match.as_str();
        if mul_str == "do()" {
            enabled = true;
            continue;
        }
        if mul_str == "don't()" {
            enabled = false;
            continue;
        }
        if enabled {
            result += process_mult_instruction(mul_str);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::problems::dec3::solution::solution1;
    use crate::problems::dec3::solution::solution2;

    #[test]
    fn test_solution1() {
        let file_path = "resources/dec3.txt";
        let response = solution1(file_path);
        let answer = 170778545;
        assert_eq!(response, answer);
    }

    #[test]
    fn test_solution2() {
        let file_path = "resources/dec3.txt";
        let response = solution2(file_path);
        let answer = 82868252;
        assert_eq!(response, answer);
    }
}