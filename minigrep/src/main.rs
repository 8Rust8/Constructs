#![allow(dead_code, unused)]
use std::env;
use std::fs;

fn main() {
    let vars: Vec<String> = env::args().collect();

    let (query, filepath) = parse_config(&vars);
    let content = fs::read_to_string(filepath).expect("Unable to read the content of file");
    println!("With text:\n{content}");
}

pub fn parse_config(args: &[String]) -> (&str, &str) {
    if args.len() < 3 {
        panic!("We need minimum two arguments, first for item to search , second the file path");
    }
    let query = &args[1];
    let filepath = &args[2];
    (query, filepath)
}
