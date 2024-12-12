use anyhow::{Ok, Result};
use aoc2024_01::{
    d1::D1,
    d2::{D2P1, D2P2},
    days::{
        d3::{D3P1, D3P2},
        d4::{D4P1, D4P2},
        d5::{solvep2, D5P1},
    },
    to_lines,
};
use std::{env, str::FromStr};
use strum_macros::EnumString;

#[derive(Debug, PartialEq, EnumString)]
#[strum(ascii_case_insensitive)]
enum DayPart {
    None,
    D1,
    D1P1,
    D1P2,

    D2P1,
    D2P2,

    D3P1,
    D3P2,

    D4P1,
    D4P2,

    D5P1,
    D5P2,
}

fn main() -> Result<()> {
    let mut fname = String::from("in-sample.txt");
    let mut which: std::result::Result<DayPart, strum::ParseError> =
        Result::Err(strum::ParseError::VariantNotFound);

    if let Some(arg1) = env::args().nth(1) {
        which = DayPart::from_str(arg1.to_lowercase().as_str());
    }
    if let Some(arg1) = env::args().nth(2) {
        fname = arg1;
    }

    match which {
        Result::Ok(DayPart::D1) | Result::Ok(DayPart::D1P1) => D1::solve(to_lines(fname)),
        Result::Ok(DayPart::D2P1) => D2P1::solve(to_lines(fname)),
        Result::Ok(DayPart::D2P2) => D2P2 {}.solve(to_lines(fname)),
        Result::Ok(DayPart::D3P1) => D3P1 {}.solve(to_lines(fname)),
        Result::Ok(DayPart::D3P2) => D3P2 {}.solve(to_lines(fname)),

        Result::Ok(DayPart::D4P1) => D4P1 {}.solve(to_lines(fname)),
        Result::Ok(DayPart::D4P2) => D4P2 {}.solve(to_lines(fname)),

        Result::Ok(DayPart::D5P1) => D5P1 {}.solve(to_lines(fname))?,
        Result::Ok(DayPart::D5P2) => solvep2(to_lines(fname))?,

        Result::Ok(DayPart::None) => println!("none"),
        Result::Ok(DayPart::D1P2) => println!("none d1p2"),

        Err(_) => println!("error"),
    }

    Ok(())
}
