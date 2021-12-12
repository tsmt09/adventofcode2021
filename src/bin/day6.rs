extern crate utilities;
use std::path::PathBuf;
use structopt::StructOpt;
use std::thread;

#[derive(Debug, Clone, Copy)]
struct Lanternfish {
    birth_timer: u8
}

impl Lanternfish {
    fn new(bt: u8) -> Lanternfish {
        Lanternfish {
            birth_timer: bt 
        }
    }
    fn evolve(&mut self) -> Vec<Lanternfish> {
        let mut children: Vec<Lanternfish> = Vec::new();
        if self.birth_timer == 0 {
            self.birth_timer = 6;
            children.push(Lanternfish::new(8));
        } else {
            self.birth_timer -= 1;
        }
        children
    }
}

fn parse_startfishes(input: Vec<String>) -> Vec<Lanternfish> {
    let mut startfishes: Vec<Lanternfish> = Vec::new();
    if !input.is_empty() {
        let lifetimes: Vec<u8> = input[0]
                        .split(',')
                        .map(|e| e.parse::<u8>().unwrap())
                        .collect();
        for lt in lifetimes {
            startfishes.push(Lanternfish::new(lt));
        }
    }
    startfishes
}

fn simulate_lifetime(fishes: Vec<Lanternfish>, years: u32, threads: usize, chunksize: usize) -> Vec<Lanternfish> {
    let mut fishes_work = fishes.clone();
    for day in 0..years {
        println!("Day {}, Fishes: {}", day, fishes_work.len());
        /* just working storage to save the during chunk computation */
        let mut fish_work_store: Vec<Lanternfish> = Vec::new();
        while !fishes_work.is_empty() {
            let mut t_vec: Vec<thread::JoinHandle<Vec<Lanternfish>>> = Vec::new();
            /* spawn N threads working on chunksize X */
            for _ in 0..threads {
                let mut split: Vec<Lanternfish>;
                if fishes_work.len() >= chunksize.try_into().unwrap() {
                    split = fishes_work.split_off((fishes_work.len() as usize) - chunksize);
                } else {
                    split = fishes_work.split_off(0);
                }
                /* computation in closure */
                let t_handle = std::thread::spawn(move || -> Vec<Lanternfish> {
                    let mut newfishes: Vec<Lanternfish> = Vec::new();
                    for fish in split.iter_mut() {
                        let mut born = fish.evolve();
                        if !born.is_empty() {
                            newfishes.append(&mut born);
                        }
                    }
                    if !newfishes.is_empty() {
                        split.append(&mut newfishes);
                    }
                    split
                });
                t_vec.push(t_handle);
            }
            for t in t_vec {
                let mut result = t.join().unwrap();
                fish_work_store.append(&mut result);
            }
        }
        fishes_work.append(&mut fish_work_store);
    }
    fishes_work
}

#[derive(Debug, StructOpt)]
#[structopt(name = "day3", version = "1.0.0")]
struct Args {
    #[structopt(short = "f", long, parse(from_os_str))]
    file: PathBuf,
    #[structopt(short = "t", long)]
    threads: Option<usize>,
    #[structopt(short = "c", long)]
    chunksize: Option<usize>
}

fn main() {
    let args = Args::from_args();

    let threads = match args.threads {
        Some(n) => n,
        None => 2
    };
    let chunksize = match args.chunksize {
        Some(n) => n,
        None => 1000000
    };

    let vec_str = utilities::files::open_file(args.file);
    let mut fishes = parse_startfishes(vec_str);
    fishes = simulate_lifetime(fishes, 256, threads, chunksize);
    println!("Result: {}", fishes.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day6_test() {
        let testvec: Vec<String> = vec![
            "3,4,3,1,2".to_string(),
        ];
        let mut fishes = parse_startfishes(testvec);
        assert_eq!(5, fishes.len());
        fishes = simulate_lifetime(fishes, 18, 1, 1);
        assert_eq!(26, fishes.len());
        fishes = simulate_lifetime(fishes, 80-18, 1, 1);
        assert_eq!(5934, fishes.len());
        fishes = simulate_lifetime(fishes, 265-80, 8, 2000000);
        assert_eq!(26984457539, fishes.len());
    }
}
