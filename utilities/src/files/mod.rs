use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn open_file(filename: std::path::PathBuf) -> Vec<String> {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(_) => panic!("no such file"),
    };
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("could not parse line"))
        .collect()
}
