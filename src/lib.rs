use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn read_file(path: &str)->Vec<String> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Vec::new();
        }
    };
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    return lines;
}
