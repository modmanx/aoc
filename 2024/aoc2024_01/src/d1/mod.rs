use std::{
    fs::File,
    io::{BufReader, Lines},
};

pub struct D1 {}

impl D1 {
    pub fn solve(lines: Lines<BufReader<File>>) {
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

        left.sort();
        right.sort();
        let mut diff = 0i32;
        for (x, y) in left.iter().zip(&right) {
            diff += (x - y).abs();
        }

        println!("{diff}");
    }
}

pub struct D1P2 {}

impl D1P2 {
    pub fn solve(lines: Lines<BufReader<File>>) {
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

        println!("{total}");
    }
}
