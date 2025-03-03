use std::fs;

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

    ChristmasData {
        vec,
        x_dim: x_count,
        y_dim: y_count,
    }
}

fn xy_index(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

fn valid_sequence(first: char, second: char, third: char, fourth: char) -> bool {
    first == 'X' && second == 'M' && third == 'A' && fourth == 'S'
}

fn check_mas(
    christmas_data: &ChristmasData,
    start_x: i64,
    start_y: i64,
    x_inc: i64,
    y_inc: i64,
) -> i32 {
    let is_valid_increment = start_x + (x_inc * 3) >= 0
        && start_x + (x_inc * 3) < christmas_data.x_dim as i64
        && start_y + (y_inc * 3) >= 0
        && start_y + (y_inc * 3) < christmas_data.y_dim as i64;
    if is_valid_increment {
        let m = christmas_data.vec[xy_index(
            (start_x + x_inc) as usize,
            (start_y + y_inc) as usize,
            christmas_data.x_dim,
        )];
        let a = christmas_data.vec[xy_index(
            (start_x + x_inc * 2) as usize,
            (start_y + y_inc * 2) as usize,
            christmas_data.x_dim,
        )];
        let s = christmas_data.vec[xy_index(
            (start_x + x_inc * 3) as usize,
            (start_y + y_inc * 3) as usize,
            christmas_data.x_dim,
        )];
        return if valid_sequence('X', m, a, s) { 1 } else { 0 };
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

fn check_backwards(christmas_data: &ChristmasData, start_x: i64, start_y: i64) -> i32 {
    let s_left = christmas_data.vec[xy_index(
        (start_x - 2) as usize,
        (start_y) as usize,
        christmas_data.x_dim,
    )] == 'S';
    let a_middle_left = christmas_data.vec[xy_index(
        (start_x - 1) as usize,
        (start_y + 1) as usize,
        christmas_data.x_dim,
    )] == 'A';
    let m_below = christmas_data.vec[xy_index(
        (start_x) as usize,
        (start_y + 2) as usize,
        christmas_data.x_dim,
    )] == 'M';
    let s_below_left = christmas_data.vec[xy_index(
        (start_x - 2) as usize,
        (start_y + 2) as usize,
        christmas_data.x_dim,
    )] == 'S';
    if s_left && a_middle_left && m_below && s_below_left {
        return 1;
    }
    0
}

fn check_above(christmas_data: &ChristmasData, start_x: i64, start_y: i64) -> i32 {
    let s_above = christmas_data.vec[xy_index(
        (start_x) as usize,
        (start_y - 2) as usize,
        christmas_data.x_dim,
    )] == 'S';
    let s2_above = christmas_data.vec[xy_index(
        (start_x + 2) as usize,
        (start_y - 2) as usize,
        christmas_data.x_dim,
    )] == 'S';
    let a_middle = christmas_data.vec[xy_index(
        (start_x + 1) as usize,
        (start_y - 1) as usize,
        christmas_data.x_dim,
    )] == 'A';
    let m_next = christmas_data.vec[xy_index(
        (start_x + 2) as usize,
        (start_y) as usize,
        christmas_data.x_dim,
    )] == 'M';
    if s_above && s2_above && a_middle && m_next {
        1
    } else {
        0
    }
}

fn check_forwards(christmas_data: &ChristmasData, start_x: i64, start_y: i64) -> i32 {
    let mut count: i32 = 0;
    let two_below = christmas_data.vec[xy_index(
        (start_x) as usize,
        (start_y + 2) as usize,
        christmas_data.x_dim,
    )];
    if two_below == 'M' {
        let s_next = christmas_data.vec[xy_index(
            (start_x + 2) as usize,
            (start_y) as usize,
            christmas_data.x_dim,
        )] == 'S';
        let a_middle = christmas_data.vec[xy_index(
            (start_x + 1) as usize,
            (start_y + 1) as usize,
            christmas_data.x_dim,
        )] == 'A';
        let s_below = christmas_data.vec[xy_index(
            (start_x + 2) as usize,
            (start_y + 2) as usize,
            christmas_data.x_dim,
        )] == 'S';
        count += if s_next && a_middle && s_below { 1 } else { 0 }
    } else if two_below == 'S' {
        let m_next = christmas_data.vec[xy_index(
            (start_x + 2) as usize,
            (start_y) as usize,
            christmas_data.x_dim,
        )] == 'M';
        let a_middle = christmas_data.vec[xy_index(
            (start_x + 1) as usize,
            (start_y + 1) as usize,
            christmas_data.x_dim,
        )] == 'A';
        let s_below = christmas_data.vec[xy_index(
            (start_x + 2) as usize,
            (start_y + 2) as usize,
            christmas_data.x_dim,
        )] == 'S';
        count += if m_next && a_middle && s_below { 1 } else { 0 }
    }
    count
}

// Could definitely clean this up for not going to worry about it for now...
fn check_mas2(christmas_data: &ChristmasData, start_x: i64, start_y: i64) -> i32 {
    let mut count = 0;
    if start_x - 2 >= 0 && start_y + 2 < christmas_data.y_dim as i64 {
        count += check_backwards(&christmas_data, start_x, start_y);
    }

    if !(start_x + 2 < christmas_data.x_dim as i64) {
        return count;
    }

    if start_y + 2 < christmas_data.y_dim as i64 {
        count += check_forwards(&christmas_data, start_x, start_y);
    }

    if start_y - 2 >= 0i64 {
        count += check_above(&christmas_data, start_x, start_y)
    }
    count
}

pub fn solution2(file: &str) -> i32 {
    let christmas_data = create_christmas_data(file);
    let mut valid_count = 0;
    for y in 0..christmas_data.y_dim {
        for x in 0..christmas_data.x_dim {
            let index = xy_index(x, y, christmas_data.x_dim);
            let index_val = christmas_data.vec[index];
            if index_val == 'M' {
                valid_count += check_mas2(&christmas_data, x as i64, y as i64);
            }
        }
    }
    valid_count
}

#[cfg(test)]
mod tests {
    use crate::problems::dec4::solution::solution;
    use crate::problems::dec4::solution::solution2;
    #[test]
    fn test_solution1() {
        let file_path = "resources/dec4.txt";
        let response = solution(file_path);
        let answer = 2458;
        assert_eq!(response, answer);
    }

    #[test]
    fn test_solution2() {
        let file_path = "resources/dec4.txt";
        let response = solution2(file_path);
        let answer = 1945;
        assert_eq!(response, answer);
    }
}
