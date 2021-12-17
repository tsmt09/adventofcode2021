extern crate utilities;
use std::collections::HashSet;
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
    _string: String,
}

impl Signal {
    fn new(
        _encoding: Vec<HashSet<char>>,
        _segments: Vec<HashSet<char>>,
        _string: String,
    ) -> Signal {
        Signal {
            encoding: _encoding,
            segments: _segments,
            _string: _string,
        }
    }
}

fn parse_signals(input: &Vec<String>) -> Vec<Signal> {
    let mut my_signals: Vec<Signal> = Vec::new();
    for line in input {
        let my_split: Vec<String> = line
            .split("|")
            .map(|x| String::from(x).trim().to_string())
            .collect();
        if my_split.len() != 2 {
            panic!("Error parsing line {}", line)
        }
        let encoding: Vec<HashSet<char>> = my_split[0]
            .split(" ")
            .map(|x| String::from(x).chars().collect())
            .collect();
        let segments: Vec<HashSet<char>> = my_split[1]
            .split(" ")
            .map(|x| String::from(x).chars().collect())
            .collect();
        my_signals.push(Signal::new(encoding, segments, line.clone()));
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

fn get_hash_set_by_len(hashsets: Vec<HashSet<char>>, length: usize) -> HashSet<char> {
    return hashsets
        .into_iter()
        .filter(|hashset| hashset.len() == length)
        .collect::<Vec<HashSet<char>>>()
        .first()
        .unwrap()
        .clone();
}

fn get_hashset_by_len_and_difflen(
    hashsets: Vec<HashSet<char>>,
    select_len: usize,
    diffset: HashSet<char>,
    diff_len: usize,
) -> HashSet<char> {
    return hashsets
        .clone()
        .into_iter()
        .filter(|x| x.len() == select_len)
        .collect::<Vec<HashSet<char>>>()
        .iter()
        .map(|x| x.clone())
        .find(|x| (*x).difference(&diffset).collect::<HashSet<&char>>().len() == diff_len)
        .unwrap();
}

fn analyze_encoding_and_return_number(signal: Signal) -> i32 {
    // search for numbers in encodings
    let one: HashSet<char> = get_hash_set_by_len(signal.encoding.clone(), 2);
    let four: HashSet<char> = get_hash_set_by_len(signal.encoding.clone(), 4);
    let seven: HashSet<char> = get_hash_set_by_len(signal.encoding.clone(), 3);
    let eight: HashSet<char> = get_hash_set_by_len(signal.encoding.clone(), 7);
    let three = get_hashset_by_len_and_difflen(signal.encoding.clone(), 5, seven.clone(), 2);
    let two = get_hashset_by_len_and_difflen(signal.encoding.clone(), 5, four.clone(), 3);
    let six = get_hashset_by_len_and_difflen(signal.encoding.clone(), 6, one.clone(), 5);
    let five = get_hashset_by_len_and_difflen(signal.encoding.clone(), 5, six.clone(), 0);
    let nine = get_hashset_by_len_and_difflen(signal.encoding.clone(), 6, three.clone(), 1);
    let zero = get_hashset_by_len_and_difflen(signal.encoding.clone(), 6, five.clone(), 2);

    /* go through segment and translate numbers */
    let mut number_str: String = String::from("");
    for seg in signal.segments {
        match seg {
            tmp if tmp == zero => number_str.push('0'),
            tmp if tmp == one => number_str.push('1'),
            tmp if tmp == two => number_str.push('2'),
            tmp if tmp == three => number_str.push('3'),
            tmp if tmp == four => number_str.push('4'),
            tmp if tmp == five => number_str.push('5'),
            tmp if tmp == six => number_str.push('6'),
            tmp if tmp == seven => number_str.push('7'),
            tmp if tmp == eight => number_str.push('8'),
            tmp if tmp == nine => number_str.push('9'),
            _ => number_str.push('X'),
        }
    }
    return match number_str.parse() {
        Ok(n) => n,
        Err(n) => {
            panic!("Cannot parse {} with error {}", number_str, n);
        }
    };
}

fn assign2_count(signals: &Vec<Signal>) -> i32 {
    let mut count = 0i32;
    for signal in signals {
        count += analyze_encoding_and_return_number(signal.clone());
    }
    count
}

fn main() {
    let args = Args::from_args();
    let vec_str = open_file(args.file);
    let signals = parse_signals(&vec_str);
    println!("Total count for assignment1: {}", assign1_count(&signals));
    println!("Total count for assignment2: {}", assign2_count(&signals));
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
        assert_eq!(assign2_count(&signals), 61229);
    }
}
