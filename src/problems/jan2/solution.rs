use std::collections::HashMap;
use std::fs;

pub fn solution(file: &str) -> i128 {
    let file = fs::read_to_string(file).expect("Could not read file");

    let mut left_vec = vec![];
    let mut right_map = HashMap::new();

    for line in file.lines() {
        let mut values = line.split_whitespace();
        let first = values.next().unwrap().parse::<i128>().unwrap();
        let second = values.next().unwrap().parse::<i128>().unwrap();

        left_vec.push(first);
        if !right_map.contains_key(&second) {
            right_map.insert(second, 1i128);
        } else {
            let current_value = right_map.get(&second).unwrap();
            right_map.insert(second, current_value + 1);
        }
    }

    let mut running_total = 0i128;
    for num in left_vec {
        if right_map.contains_key(&num) {
            let number_of_occurrences = right_map.get(&num).unwrap();
            running_total += number_of_occurrences * num;
        }
    }
    running_total
}

#[cfg(test)]
mod tests {
    use crate::problems::jan2::solution::solution;

    #[test]
    fn test_solution() {
        let file_path = "resources/jan2.txt";
        let response = solution(file_path);
        let answer = 24316233;
        assert_eq!(response, answer);
    }
}