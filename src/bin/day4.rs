extern crate utilities;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "day3", version = "1.0.0")]
struct Args {
    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    ///#[structopt(short, parse(from_occurrences))]
    ///verbose: u8,
    /// Files to process
    #[structopt(short, long, parse(from_os_str))]
    file: PathBuf,
}

#[derive(Debug, Copy, Clone)]
struct Field {
    field: [[i32; 5]; 5],
    hits: [[bool; 5]; 5],
    sum: i32,
    filledlines: usize,
}

fn parse_board(data: Vec<String>) -> (Vec<i32>, Vec<Field>) {
    /* interpret line 0 as input numbers */
    let mut workiterator = data.iter();
    let input_vector: Vec<i32> = workiterator
        .next()
        .unwrap()
        .split(',')
        .map(|element| element.parse::<i32>().unwrap())
        .collect();
    let mut fields: Vec<Field> = Vec::new();
    let mut work_field: Field = Field {
        field: [[0; 5]; 5],
        hits: [[false; 5]; 5],
        sum: 0,
        filledlines: 0,
    };
    for line in workiterator {
        if line.is_empty() {
            /* parse new field */
            if work_field.filledlines == 5 {
                fields.push(work_field);
            }
            work_field = Field {
                field: [[0; 5]; 5],
                hits: [[false; 5]; 5],
                sum: 0,
                filledlines: 0,
            };
        } else {
            if work_field.filledlines >= 5 {
                panic!("input has wrong format in line: {}", line)
            }
            let numbers_line: Vec<i32> = line
                .split(' ')
                .filter(|element| !element.is_empty())
                .map(|element| element.trim().parse::<i32>().unwrap())
                .collect();
            if numbers_line.len() == 5 {
                work_field.sum += numbers_line[0]
                    + numbers_line[1]
                    + numbers_line[2]
                    + numbers_line[3]
                    + numbers_line[4];
                work_field.field[work_field.filledlines] = [
                    numbers_line[0],
                    numbers_line[1],
                    numbers_line[2],
                    numbers_line[3],
                    numbers_line[4],
                ];
                work_field.filledlines += 1;
            } else {
                panic!("wrong count of numbers in line: {}", line)
            }
        }
    }
    (input_vector, fields)
}

fn calc_and_print_hit(field: &Field, number: i32) -> i32 {
    let mut sum = 0;
    for index_x in 0..5 {
        for index_y in 0..5 {
            if field.hits[index_x][index_y] == false {
                sum += field.field[index_x][index_y];
            }
        }
    }
    println!(
        "Hit found! Sum: {}, Number: {}, Score: {}",
        sum,
        number,
        sum * number
    );
    return sum * number;
}

fn play(input: Vec<i32>, fields: &mut Vec<Field>) -> i32 {
    let mut input_iter = input.iter();
    let mut winners: Vec<Field> = Vec::new();
    let mut last_number = 0;
    loop {
        let number = match input_iter.next() {
            Some(n) => *n,
            None => {
                return calc_and_print_hit(&winners.last().unwrap(), last_number);
            }
        };
        let mut won_keys: Vec<usize> = Vec::new();
        for (key, field) in fields.iter_mut().enumerate() {
            /* set number to -1 in fields lines and check for hits */
            let mut found = false;
            for index_x in 0..5 {
                let mut hitcount = 0;
                for index_y in 0..5 {
                    if field.field[index_x][index_y] == number {
                        field.hits[index_x][index_y] = true;
                    }
                    if field.hits[index_x][index_y] == true {
                        hitcount += 1;
                    }
                }
                if hitcount == 5 {
                    winners.push(field.clone());
                    won_keys.push(key);
                    found = true;
                }
            }
            if found {
                continue;
            }
            /* set number to -1 in fields columns and check for hits */
            for index_y in 0..5 {
                let mut hitcount = 0;
                for index_x in 0..5 {
                    if field.hits[index_x][index_y] == true {
                        hitcount += 1;
                    }
                }
                if hitcount == 5 {
                    winners.push(field.clone());
                    won_keys.push(key);
                    continue;
                }
            }
        }
        won_keys.sort_by(|a, b| b.cmp(a));
        for key in won_keys {
            fields.remove(key);
        }
        if fields.is_empty() {
            return calc_and_print_hit(&winners.pop().unwrap(), number);
        }
        last_number = number;
    }
}

fn main() {
    let args = Args::from_args();
    let vec_str = utilities::files::open_file(args.file);
    let (input_vector, mut fields) = parse_board(vec_str);
    play(input_vector, &mut fields);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bingo_test() {
        let testvec: Vec<String> = vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1".to_string(),
            "".to_string(),
            "22 13 17 11  0".to_string(),
            "8  2 23  4 24".to_string(),
            "21  9 14 16  7".to_string(),
            "6 10  3 18  5".to_string(),
            "1 12 20 15 19".to_string(),
            "".to_string(),
            "3 15  0  2 22".to_string(),
            "9 18 13 17  5".to_string(),
            "19  8  7 25 23".to_string(),
            "20 11 10 24  4".to_string(),
            "14 21 16 12  6".to_string(),
            "".to_string(),
            "14 21 17 24  4".to_string(),
            "10 16 15  9 19".to_string(),
            "18  8 23 26 20".to_string(),
            "22 11 13  6  5".to_string(),
            "2  0 12  3  7".to_string(),
        ];
        let (input_vector, mut fields) = parse_board(testvec);
        assert_eq!(play(input_vector, &mut fields), 1924);
    }
}
