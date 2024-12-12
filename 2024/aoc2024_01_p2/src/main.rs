use anyhow::{Context, Ok, Result};
use std::{
    fs::File,
    io::{self, stdin, BufRead, BufReader, Read},
    path::Path,
};

fn main() -> Result<()> {
    // let mut buffer = String::new();
    let filename = String::from("in.txt");
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for line in lines {
        let lll = line.unwrap();
        // println!("{}  {}", lll, lll.len());
        if lll.is_empty() {
            break;
        }
        let mut ls = lll.split("   ");
        left.push(ls.next().unwrap().parse::<i32>().unwrap());
        right.push(ls.next().unwrap().parse::<i32>().unwrap());
    }

    let mut total = 0i32;
    for x in left {
        let bbb = right.iter().filter(|y| *y == &x).count() as i32;
        total += x * bbb;
    }

    // println!("{:?}", left);
    // println!("{:?}", right);

    println!("{total}");

    Ok(())
}
