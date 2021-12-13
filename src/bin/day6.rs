extern crate utilities;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, Clone, Copy)]
struct LanternfishCrowd {
    days_left_count: [u128; 9],
}

impl LanternfishCrowd {
    fn new(dlc: [u128; 9]) -> LanternfishCrowd {
        LanternfishCrowd {
            days_left_count: dlc,
        }
    }
    fn spend_day(&mut self) {
        let copy = self.days_left_count.clone();
        self.days_left_count[8] = copy[0];
        self.days_left_count[7] = copy[8];
        self.days_left_count[6] = copy[7] + copy[0];
        self.days_left_count[5] = copy[6];
        self.days_left_count[4] = copy[5];
        self.days_left_count[3] = copy[4];
        self.days_left_count[2] = copy[3];
        self.days_left_count[1] = copy[2];
        self.days_left_count[0] = copy[1];
    }
    fn allover_count(&self) -> u128 {
        return self.days_left_count[0]
            + self.days_left_count[1]
            + self.days_left_count[2]
            + self.days_left_count[3]
            + self.days_left_count[4]
            + self.days_left_count[5]
            + self.days_left_count[6]
            + self.days_left_count[7]
            + self.days_left_count[8];
    }
}

fn parse_startfishes(input: Vec<String>) -> LanternfishCrowd {
    let mut crowd = LanternfishCrowd::new([0u128, 0, 0, 0, 0, 0, 0, 0, 0]);
    if !input.is_empty() {
        let lifetimes: Vec<u8> = input[0]
            .split(',')
            .map(|e| e.parse::<u8>().unwrap())
            .collect();
        for lt in lifetimes {
            match lt {
                0 => crowd.days_left_count[0] += 1,
                1 => crowd.days_left_count[1] += 1,
                2 => crowd.days_left_count[2] += 1,
                3 => crowd.days_left_count[3] += 1,
                4 => crowd.days_left_count[4] += 1,
                5 => crowd.days_left_count[5] += 1,
                6 => crowd.days_left_count[6] += 1,
                7 => crowd.days_left_count[7] += 1,
                _ => panic!("unknown start number {}", lt),
            }
        }
    }
    crowd
}

fn simulate_lifetime(crowd: LanternfishCrowd, days: u128) -> LanternfishCrowd {
    let mut work_crowd = crowd;
    for day in 0..days {
        println!(
            "Day {} - Fishes {} - crowd: {:?}",
            day,
            work_crowd.allover_count(),
            work_crowd.days_left_count
        );
        work_crowd.spend_day();
    }
    work_crowd
}

#[derive(Debug, StructOpt)]
#[structopt(name = "day3", version = "1.0.0")]
struct Args {
    #[structopt(short = "f", long, parse(from_os_str))]
    file: PathBuf,
}

fn main() {
    let args = Args::from_args();
    let vec_str = utilities::files::open_file(args.file);
    let mut crowd = parse_startfishes(vec_str);
    crowd = simulate_lifetime(crowd, 256);
    println!("Result: {}", crowd.allover_count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day6_test() {
        let testvec: Vec<String> = vec!["3,4,3,1,2".to_string()];
        let mut crowd = parse_startfishes(testvec);
        crowd = simulate_lifetime(crowd, 18);
        assert_eq!(26, crowd.allover_count());
        crowd = simulate_lifetime(crowd, 80 - 18);
        assert_eq!(5934, crowd.allover_count());
        crowd = simulate_lifetime(crowd, 256 - 80);
        assert_eq!(26984457539, crowd.allover_count());
    }
}
