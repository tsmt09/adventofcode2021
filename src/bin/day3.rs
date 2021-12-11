use std::io;
use std::io::BufRead;

struct Rates {
    gamma: u32,
    epsilon: u32,
    oxygen: u32,
    co2: u32
}

fn calculate_oxygen(data: Vec<String>) -> (u32, u32) {
    let mut index = 0;
    let limit = data[0].chars().count();
    let mut o2filter: Vec<String> = data.clone();
    let mut oxygenfilter: Vec<String> = data.clone();
    loop {
        /* calc ones and zeros */
        let mut oxy_ones = 0;
        let mut oxy_zeros = 0;
        for line in oxygenfilter.clone() {
            let getchar: char = line.chars().nth(index).unwrap();
            match getchar {
                '0' => { oxy_zeros = oxy_zeros + 1; }
                '1' => { oxy_ones = oxy_ones + 1; }
                _ => panic!("Unknown character in line: {}", line)
            }
        }
        let mut o2_ones = 0;
        let mut o2_zeros = 0;
        for line in o2filter.clone() {
            let getchar: char = line.chars().nth(index).unwrap();
            match getchar {
                '0' => { o2_zeros = o2_zeros + 1; }
                '1' => { o2_ones = o2_ones + 1; }
                _ => panic!("Unknown character in line: {}", line)
            }
        }
        /* decide what is to be filtered */
        let mut oxyfilter_char: char = '1';
        let mut o2filter_char: char = '0';
        if oxy_zeros > oxy_ones {
            oxyfilter_char = '0';
        }
        if o2_ones < o2_zeros {
            o2filter_char = '1';
        }
        /* first filter */
        if oxygenfilter.len() > 1 {
            oxygenfilter.retain(|line| {
                let getchar: char = line.chars().nth(index).unwrap();
                if getchar == oxyfilter_char {
                    return true;
                }
                return false;
            });
        }
        if o2filter.len() > 1 {
            o2filter.retain(|line| {
                let getchar: char = line.chars().nth(index).unwrap();
                if getchar == o2filter_char {
                    return true;
                }
                return false;
            });
        }
        if (oxygenfilter.len() == 1) && (o2filter.len() == 1) {
            let oxygen = u32::from_str_radix(oxygenfilter[0].as_str(), 2).unwrap();
            let o2 = u32::from_str_radix(o2filter[0].as_str(), 2).unwrap();
            return (oxygen, o2);
        } else {
            index += 1;
        }
        if index == limit {
            panic!("filtering out of bounds {}", index)
        }
    }
}

fn calculate_rates(data: Vec<String>) -> Rates {
    let line_length: usize = data[0].chars().count();
    let mut zero_count = vec![0u32; line_length];
    let mut one_count = vec![0u32; line_length];
    /* iterate data and add to vectors */
    for line in data.clone() {
        for index in 0..line_length {
            let getchar: char = line.chars().nth(index).unwrap();
            match getchar {
                '0' => { zero_count[index] = zero_count[index] + 1; }
                '1' => { one_count[index] = one_count[index] + 1; }
                _ => panic!("Unknown character in line: {}", line)
            }
        }
    }
    /* calculate rates */
    let mut gamma_rate = 0b0;
    let mut epsilon_rate = 0b0;
    for index in 0..line_length {
        gamma_rate <<= 1;
        epsilon_rate <<= 1;
        if zero_count[index] < one_count[index] {
            gamma_rate |= 0b01;
        } else {
            epsilon_rate |= 0b01;
        }
    }
    /* calc oxygen and co2 */
    let (oxygen, co2) = calculate_oxygen(data.clone());
    Rates { 
        gamma: gamma_rate,
        epsilon: epsilon_rate,
        oxygen: oxygen,
        co2: co2
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut vec_str: Vec<String> = Vec::new();
    /* parse line into commands */
    for line in lines {
        match line {
            Ok(str_line) => {
                vec_str.push(str_line)
            }
            Err(err) => {
                println!("error reading stdin: {}", err)
            }
        }
    }
    let rates = calculate_rates(vec_str);
    println!("The Rates are:\ngamma: {:b}\nepsilon: {:b}\nepsilon*gamma: {}",
            rates.gamma, rates.epsilon, rates.epsilon*rates.gamma);
    println!("The Ratings are:\nco2: {:b}\noxygen: {:b}\nco2*oxygen: {}",
            rates.co2, rates.oxygen, rates.co2*rates.oxygen);
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gamma_test() {
        let testvec: Vec<String> = vec![
            "00100".to_string(),
            "11110".to_string(),
            "10110".to_string(),
            "10111".to_string(),
            "10101".to_string(),
            "01111".to_string(),
            "00111".to_string(),
            "11100".to_string(),
            "10000".to_string(),
            "11001".to_string(),
            "00010".to_string(),
            "01010".to_string(),
        ];
        let rates = calculate_rates(testvec);
        assert_eq!(22, rates.gamma);
        assert_eq!(9, rates.epsilon);
        assert_eq!(10, rates.co2);
        assert_eq!(23, rates.oxygen);
        assert_eq!(230, rates.co2*rates.oxygen);
        assert_eq!(198, rates.gamma*rates.epsilon);
    }
}