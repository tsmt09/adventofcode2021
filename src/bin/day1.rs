use std::io;
use std::io::BufRead;
use std::io::Lines;
use std::io::StdinLock;

fn get_vector_from_lines(lines: Lines<StdinLock<>>) -> Vec<i32> {
    let mut int_vec: Vec<i32> = Vec::new();
    for line in lines {
        match line {
            Ok(str_line) => {
                int_vec.push(str_line.parse().unwrap());
            }
            Err(err) => {
                println!("error: {}", err)
            }
        }
    }
    int_vec
}

fn measure_increases(vec: Vec<i32>) -> i32 {
    let mut last_value: i32 = -1;
    let mut increased_number: i32 = -1;
    for element in vec {
        if element > last_value {
            increased_number += 1;
        }
        last_value = element;
    }
    increased_number
}

fn three_measurement(vec: Vec<i32>) -> i32 {
    let mut new_vec: Vec<i32> = Vec::new();
    for vec_index in 0..vec.len() {
        let mut sum = 0;
        sum += vec[vec_index];
        if (vec_index + 1) < (vec.len() - 1) {
            sum += vec[vec_index + 1];
        }
        if (vec_index + 2) < (vec.len() - 1) {
            sum += vec[vec_index + 2];
        }
        new_vec.push(sum);
    }
    measure_increases(new_vec)
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let vector = get_vector_from_lines(lines);
    let vector_tree = vector.clone();
    let increases = measure_increases(vector);
    println!("Increased number count: {}", increases);
    let three_increases = three_measurement(vector_tree);
    println!("Three increases number count: {}", three_increases);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measure_increases() {
        let testvec = vec![199i32, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(7, measure_increases(testvec));
    }
    #[test]
    fn test_three_measurement() {
        let testvec = vec![199i32, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(5, three_measurement(testvec));
    }
}