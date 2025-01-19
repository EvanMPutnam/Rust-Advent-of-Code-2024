use std::collections::HashMap;
use std::fs;

pub fn solution1(file: &str) -> i128 {
    let file = fs::read_to_string(file).expect("Could not read file");
    let mut left_array = vec![];
    let mut right_array = vec![];
    for line in file.lines() {
        let mut values = line.split_whitespace();
        let first = values.next().unwrap().parse::<i128>().unwrap();
        let second = values.next().unwrap().parse::<i128>().unwrap();
        left_array.push(first);
        right_array.push(second);
    }

    left_array.sort();
    right_array.sort();

    if left_array.len() != right_array.len() {
        panic!("Left and right hand arrays are not equal!");
    }

    let mut running_total = 0;
    for i in 0..left_array.len() {
        running_total += (left_array[i] - right_array[i]).abs();
    }

    running_total
}

pub fn solution2(file: &str) -> i128 {
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
    use crate::problems::dec1::solution::solution1;
    use crate::problems::dec1::solution::solution2;

    #[test]
    fn test_solution1() {
        let file_path = "resources/dec1.txt";
        let response = solution1(file_path);
        let answer = 1666427;
        assert_eq!(response, answer);
    }

    #[test]
    fn test_solution2() {
        let file_path = "resources/dec1.txt";
        let response = solution2(file_path);
        let answer = 24316233;
        assert_eq!(response, answer);
    }
}