use std::env;
use rosalind::{FileReader, 
                solution::*,
            * };

fn main() {
    let args : Vec<String> = env::args().collect();
    let path = FileReader::Path::get(&args).unwrap();
    let content = FileReader::Path::read(&path).unwrap();

    BA10::K::run(content);
    // stronghold::EDIT::run(&content);
}