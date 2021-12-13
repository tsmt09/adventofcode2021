extern crate utilities;
use std::path::PathBuf;
use structopt::StructOpt;
use utilities::files::open_file;

#[derive(Debug, StructOpt)]
#[structopt(name = "day8", version = "1.0.0")]
struct Args {
    #[structopt(short = "f", long, parse(from_os_str))]
    file: PathBuf,
}

#[derive(Debug)]
struct Signal {
    encoding: Vec<String>,
    signal: Vec<String>,
}

impl Signal {
    fn new(_encoding: Vec<String>, _signal: Vec<String>) -> Signal {
        Signal {
            encoding: _encoding,
            signal: _signal,
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
        let encoding: Vec<String> = my_split[0].split(" ").map(|x| x.to_string()).collect();
        let signal: Vec<String> = my_split[1].split(" ").map(|x| x.to_string()).collect();
        my_signals.push(Signal::new(encoding, signal));
    }
    my_signals
}

fn main() {
    let args = Args::from_args();
    let vec_str = open_file(args.file);
    let signals = parse_signals(&vec_str);
    dbg!(signals);
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
    }
}
