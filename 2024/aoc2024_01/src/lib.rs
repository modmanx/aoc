use std::{
    fs::File,
    io::{self, BufRead},
};

pub mod d1;
pub mod d2;
pub mod days;

/// returns iterator
pub fn to_lines(fname: String) -> std::io::Lines<std::io::BufReader<std::fs::File>> {
    let file = File::open(fname).unwrap();
    io::BufReader::new(file).lines()
}
