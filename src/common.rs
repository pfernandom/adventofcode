use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::vec::Vec;

pub fn read_file_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename);

    let f = match file {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };

    BufReader::new(&f)
        .lines()
        .filter_map(Result::ok)
        .collect()
}