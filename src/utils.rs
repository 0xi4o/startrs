#![allow(dead_code)]

use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

pub fn read_lines_from_file(file_name: &str) -> Lines<BufReader<File>> {
    let file = File::open(file_name)
        .expect("error reading file. make sure the file exists and is readable.");
    let reader = BufReader::new(file);

    reader.lines()
}
