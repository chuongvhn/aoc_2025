use std::collections::VecDeque;
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

pub fn split_number_into_digits(input: u64) -> VecDeque<u64>{
    let mut rets : VecDeque<u64> = VecDeque::new();
    let mut tmp = input;
    let mut calculate = tmp / 10;
    let mut digit = tmp % 10;
    rets.push_front(digit);
    
    while calculate != 0 {
        tmp = calculate;
        calculate = tmp / 10;
        digit = tmp % 10;
        rets.push_front(digit);
    }

    return rets;
}
