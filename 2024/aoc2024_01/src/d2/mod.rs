use std::{
    borrow::BorrowMut,
    fs::File,
    io::{BufReader, Lines},
};

pub struct D2P1 {}

impl D2P1 {
    pub fn solve(lines: Lines<BufReader<File>>) {
        let mut okcount = 0;
        for line in lines {
            let lll = line.unwrap();
            // println!("{}  {}", lll, lll.len());
            if lll.is_empty() {
                break;
            }
            let ls: Vec<i32> = lll.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
            let mut ok = true;
            let mut growing = false;
            let mut falling = false;
            println!("checking {:?}", ls);
            for (key, val) in ls.iter().enumerate() {
                if key > 0 {
                    let prev_diff = val - ls[key - 1];
                    if prev_diff == 0 {
                        ok = false;
                        println!("   - not ok ... diff = 0");
                        break;
                    }
                    if prev_diff < 0 {
                        falling = true;
                    }
                    if prev_diff > 0 {
                        growing = true;
                    }
                    if falling && growing {
                        ok = false;
                        println!("   - not ok ... falling and growing");
                        break;
                    }
                    if prev_diff.abs() > 3 {
                        ok = false;
                        println!("   - not ok ... diff too big");
                        break;
                    }
                }
            }

            if ok {
                okcount += 1;
                println!("ok!")
            }
        }
        println!("ok: {:?}", okcount);
    }
}

pub struct D2P2 {}

impl D2P2 {
    fn check_line(&self, line: Vec<i32>) -> bool {
        let mut falling = false;
        let mut growing = false;
        let mut valid = true;
        for ii in 0..line.len() - 1 {
            if line[ii + 1] > line[ii] {
                growing = true;
            }
            if line[ii + 1] < line[ii] {
                falling = true;
            }
            if growing && falling {
                print!("  - falling growing");
                valid = false;
                break;
            }

            let diff = line[ii] - line[ii + 1];
            let diff_abs = diff.abs();

            if diff_abs > 3 || diff_abs == 0 {
                print!("  - diff > 3");
                valid = false;
                break;
            }
        }

        valid
    }

    pub fn solve(&self, lines: Lines<BufReader<File>>) {
        let mut okcount = 0;
        for line in lines {
            let lll = line.unwrap();
            if lll.is_empty() {
                break;
            }
            let mut ls: Vec<i32> = lll.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
            println!("checking {:?}", ls);
            // let mut is_valid = false;

            let mut valid;
            valid = self.check_line(ls.clone());
            if !valid {
                for ii in 0..ls.len() {
                    let mut lsclone = ls.clone();
                    lsclone.remove(ii);
                    valid = self.check_line(lsclone);
                    if valid {
                        break;
                    }
                }
            }
            if valid {
                okcount += 1;
            }
            println!(" ");
        }
        println!("ok: {:?}", okcount);
    }
}
