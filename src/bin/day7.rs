extern crate utilities;
use statistical;
use std::path::PathBuf;
use structopt::StructOpt;
use utilities::files::open_file;
use utilities::stringparsers::parse_string_as_vec_int;

#[derive(Debug)]
struct CrabsInSubmarines {
    crabs: Vec<i128>,
    min: i128,
    max: i128,
}

impl CrabsInSubmarines {
    fn new() -> CrabsInSubmarines {
        CrabsInSubmarines {
            crabs: Vec::new(),
            min: 0,
            max: 0,
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "day3", version = "1.0.0")]
struct Args {
    #[structopt(short = "f", long, parse(from_os_str))]
    file: PathBuf,
}

fn parse_crabs(input: Vec<String>) -> CrabsInSubmarines {
    let mut crabs = CrabsInSubmarines::new();
    if !input.is_empty() {
        crabs.crabs = parse_string_as_vec_int(&input[0], ',');
        crabs.min = match crabs.crabs.iter().min() {
            Some(min) => *min,
            None => 0i128,
        };
        crabs.max = match crabs.crabs.iter().max() {
            Some(max) => *max,
            None => 0i128,
        };
    }
    crabs
}

fn find_horizontal_with_lowest_fuel(crabs: CrabsInSubmarines) -> i128 {
    let median: i128 = statistical::median(&crabs.crabs);
    /* calc each distance to median and add to fuel */
    let mut fuels: Vec<i128> = Vec::new();
    for field in crabs.min..crabs.max {
        let mut fuel = 0i128;
        for crab in &crabs.crabs {
            let distance = (field - crab).abs();
            for index in 1..=distance {
                fuel += index;
            }
        }
        fuels.push(fuel);
    }
    dbg!(&fuels);
    println!(
        "min fuel used to get all crabs to a position: {}",
        fuels.iter().min().unwrap()
    );
    println!(
        "min fuel used to get all crabs to median {}: {}",
        median,
        fuels.iter().nth(median as usize).unwrap()
    );
    *fuels.iter().min().unwrap()
}

fn main() {
    let args = Args::from_args();
    let vec_str = open_file(args.file);
    let crabs = parse_crabs(vec_str);
    find_horizontal_with_lowest_fuel(crabs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crabs_test() {
        let testvec: Vec<String> = vec!["16,1,2,0,4,2,7,1,2,14".to_string()];
        let crabs = parse_crabs(testvec);
        assert_eq!(crabs.crabs.len(), 10);
        assert_eq!(crabs.min, 0);
        assert_eq!(crabs.max, 16);
        assert_eq!(find_horizontal_with_lowest_fuel(crabs), 168);
    }
}
