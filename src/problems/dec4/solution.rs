use std::fs;

// TODO - Solution part 2.

struct ChristmasData {
    vec: Vec<char>,
    x_dim: usize,
    y_dim: usize,
}

fn create_christmas_data(file: &str) -> ChristmasData {
    let mut vec = vec![];
    let file = fs::read_to_string(file).expect("Could not read file");
    let mut y_count = 0;
    let mut x_count = 0;
    for line in file.lines() {
        x_count = line.len();
        for char in line.chars() {
            vec.push(char);
        }
        y_count += 1;
    }

    ChristmasData{vec, x_dim: x_count, y_dim: y_count }
}

fn xy_index(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

fn valid_sequence(first: char, second: char, third: char, fourth: char) -> bool {
    first == 'X' && second == 'M' && third == 'A' && fourth == 'S'
}

fn check_mas(christmas_data: &ChristmasData,
             start_x: i64, start_y: i64,
             x_inc: i64, y_inc: i64) -> i32 {
    let is_valid_increment =
        start_x + (x_inc * 3) >= 0
        && start_x + (x_inc * 3) < christmas_data.x_dim as i64
        && start_y + (y_inc * 3) >= 0
        && start_y + (y_inc * 3) < christmas_data.y_dim as i64;
    if is_valid_increment  {
        let m = christmas_data.vec[xy_index(
            (start_x + x_inc) as usize,
            (start_y + y_inc) as usize, christmas_data.x_dim)];
        let a = christmas_data.vec[xy_index(
            (start_x + x_inc * 2) as usize,
            (start_y + y_inc * 2) as usize, christmas_data.x_dim)];
        let s = christmas_data.vec[xy_index(
            (start_x + x_inc * 3) as usize,
            (start_y + y_inc * 3) as usize, christmas_data.x_dim)];
        return if valid_sequence('X', m, a, s) {1} else {0};
    }
    0
}

pub fn solution(file: &str) -> i32 {
    let christmas_data = create_christmas_data(file);
    let mut valid_count = 0;
    for y in 0..christmas_data.y_dim {
        for x in 0..christmas_data.x_dim {
            let index = xy_index(x, y, christmas_data.x_dim);
            let index_val = christmas_data.vec[index];
            if index_val == 'X' {
                // Normal cases
                valid_count += check_mas(&christmas_data, x as i64, y as i64, 1, 0);
                valid_count += check_mas(&christmas_data, x as i64, y as i64, -1, 0);
                valid_count += check_mas(&christmas_data, x as i64, y as i64, 0, 1);
                valid_count += check_mas(&christmas_data, x as i64, y as i64, 0, -1);
                // Diagonal cases
                valid_count += check_mas(&christmas_data, x as i64, y as i64, 1, 1);
                valid_count += check_mas(&christmas_data, x as i64, y as i64, 1, -1);
                valid_count += check_mas(&christmas_data, x as i64, y as i64, -1, 1);
                valid_count += check_mas(&christmas_data, x as i64, y as i64, -1, -1);
            }
        }
    }
    valid_count
}

#[cfg(test)]
mod tests {
    use crate::problems::dec4::solution::solution;
    #[test]
    fn test_solution1() {
        let file_path = "resources/dec4.txt";
        let response = solution(file_path);
        let answer = 2458;
        assert_eq!(response, answer);
    }
}
