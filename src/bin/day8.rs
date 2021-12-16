extern crate utilities;
use std::collections::HashSet;
use std::hash::Hash;
use std::path::PathBuf;
use structopt::StructOpt;
use utilities::files::open_file;

#[derive(Debug, StructOpt)]
#[structopt(name = "day8", version = "1.0.0")]
struct Args {
    #[structopt(short = "f", long, parse(from_os_str))]
    file: PathBuf,
}

#[derive(Debug, Clone)]
struct Signal {
    encoding: Vec<HashSet<char>>,
    segments: Vec<HashSet<char>>,
}

impl Signal {
    fn new(_encoding: Vec<HashSet<char>>, _segments: Vec<HashSet<char>>) -> Signal {
        Signal {
            encoding: _encoding,
            segments: _segments,
        }
    }
}

fn parse_signals(input: &Vec<String>) -> Vec<Signal> {
    let mut my_signals: Vec<Signal> = Vec::new();
    for line in input {
        let my_split: Vec<String> = line
            .split("|")
            .map(|x| x.to_string().trim().to_string())
            .collect();
        if my_split.len() != 2 {
            panic!("Error parsing line {}", line)
        }
        let encoding: Vec<HashSet<char>> = my_split[0]
            .split(" ")
            .map(|x| x.to_string().chars().collect())
            .collect();
        let segments: Vec<HashSet<char>> = my_split[1]
            .split(" ")
            .map(|x| x.to_string().chars().collect())
            .collect();
        my_signals.push(Signal::new(encoding, segments));
    }
    my_signals
}

fn assign1_count(signals: &Vec<Signal>) -> i32 {
    let mut count: i32 = 0;
    for signal in signals {
        for segment in &signal.segments {
            if [2, 3, 4, 7].contains(&segment.len()) {
                count += 1;
            }
        }
    }
    count
}

fn analyze_encoding_and_return_number(signal: Signal) -> i32 {
    // search for numbers in encodings
    let one: Vec<&HashSet<char>> = signal.encoding.iter().filter(|x| x.len() == 2).collect();
    let seven: Vec<&HashSet<char>> = signal.encoding.iter().filter(|x| x.len() == 2).collect();
    let four: Vec<&HashSet<char>> = signal.encoding.iter().filter(|x| x.len() == 2).collect();
    let eight: Vec<&HashSet<char>> = signal.encoding.iter().filter(|x| x.len() == 2).collect();
    let mut top: Vec<&HashSet<char>> = seven.clone();
    top.retain(|x| one.contains(x));
    // let mut nine_missing_bottom: HashSet<_> = four.union(&top).collect();
    dbg!(top);
    return 1;
}

fn main() {
    let args = Args::from_args();
    let vec_str = open_file(args.file);
    let signals = parse_signals(&vec_str);
    dbg!(&signals);
    println!("Total count for assignment1: {}", assign1_count(&signals));
    analyze_encoding_and_return_number(signals.iter().next().unwrap().clone());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signals_test() {
        let testvec: Vec<String> = vec![
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".to_string(),
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc".to_string(),
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg".to_string(),
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb".to_string(),
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea".to_string(),
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb".to_string(),
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe".to_string(),
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef".to_string(),
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb".to_string(),
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".to_string()
        ];
        dbg!(&testvec);
        let signals = parse_signals(&testvec);
        dbg!(&signals);
        assert_eq!(testvec.len(), signals.len());
        assert_eq!(assign1_count(&signals), 26);
    }
}
