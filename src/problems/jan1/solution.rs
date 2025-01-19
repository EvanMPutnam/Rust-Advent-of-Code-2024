use std::fs;

pub fn solution(file: &str) -> i128 {
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

#[cfg(test)]
mod tests {
    use crate::problems::jan1::solution::solution;

    #[test]
    fn test_solution() {
        let file_path = "resources/jan1.txt";
        let response = solution(file_path);
        let answer = 1666427;
        assert_eq!(response, answer);
    }
}