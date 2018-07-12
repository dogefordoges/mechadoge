use std::fs::File;
use std::io::prelude::*;
mod processor;

fn read_to_lines(filename: &str) -> Vec<String> {
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut lines = Vec::<String>::new();

    contents.lines().for_each( |line| lines.push(line.to_string()));

    return lines;
}

fn main() {
    let input_lines: Vec<String> = read_to_lines("data/preprocessor_test_input.mdg");
    let (processed_code, _): (Vec<String>, processor::Context) = processor::preprocess_code(input_lines);
    for t in processed_code { println!("{}", t); }
}
