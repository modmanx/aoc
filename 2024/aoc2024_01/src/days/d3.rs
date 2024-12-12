use std::{
    fs::File,
    io::{BufReader, Lines},
};

use winnow::Parser;
use winnow::{ascii::digit0, token::any, PResult};

pub struct D3P1 {}

impl D3P1 {
    pub fn solve(&self, lines: Lines<BufReader<File>>) {
        fn parser22<'s>(input: &mut &'s str) -> PResult<&'s str> {
            digit0.parse_next(input)
        }

        fn parsertakeone(input: &mut &str) -> PResult<char> {
            any.parse_next(input)
        }

        fn parser(input: &mut &str) {
            let mut final_result = 0i32;
            loop {
                let mm = ("mul(", parser22, ",", parser22, ")").parse_next(input);
                if let Ok(mmmatch) = mm {
                    let n1 = mmmatch.1.parse::<i32>().unwrap();
                    let n2 = mmmatch.3.parse::<i32>().unwrap();
                    final_result += n1 * n2;
                    println!("found next match {} {}", n1, n2);
                } else {
                    let mmm = parsertakeone.parse_next(input);
                    if mmm.is_ok() {
                        // println!("ok char");
                    } else {
                        println!("hmmm");
                        break;
                    }
                }
            }

            println!("final = {final_result}");
        }

        let linestr = lines.into_iter().nth(0).unwrap().unwrap().clone();
        parser(&mut &linestr[..]);
    }
}

pub struct D3P2 {}

impl D3P2 {
    pub fn solve(&self, lines: Lines<BufReader<File>>) {
        fn parser22<'s>(input: &mut &'s str) -> PResult<&'s str> {
            digit0.parse_next(input)
        }

        fn parsertakeone(input: &mut &str) -> PResult<char> {
            any.parse_next(input)
        }

        fn parserdo<'s>(input: &mut &'s str) -> PResult<&'s str> {
            "do()".parse_next(input)
        }

        fn parserdont<'s>(input: &mut &'s str) -> PResult<&'s str> {
            "don't()".parse_next(input)
        }

        fn parserlong<'s>(
            input: &mut &'s str,
        ) -> PResult<(&'s str, &'s str, &'s str, &'s str, &'s str)> {
            ("mul(", parser22, ",", parser22, ")").parse_next(input)
        }

        fn parser(input: &mut &str) {
            let mut final_result = 0i32;
            let mut mul_enable = true;
            loop {
                let mm = parserlong.parse_next(input);
                if let Ok(mmmatch) = mm {
                    let n1 = mmmatch.1.parse::<i32>().unwrap();
                    let n2 = mmmatch.3.parse::<i32>().unwrap();
                    if mul_enable {
                        final_result += n1 * n2;
                    }

                    // println!("found next match {} {}", n1, n2);
                    continue;
                }

                let mmdo = parserdo.parse_next(input);
                if mmdo.is_ok() {
                    mul_enable = true;
                    continue;
                }

                let mmdont = parserdont.parse_next(input);
                if mmdont.is_ok() {
                    mul_enable = false;
                    continue;
                }

                let mmm = parsertakeone.parse_next(input);
                if mmm.is_ok() {
                    // println!("ok char");
                } else {
                    println!("hmmm");
                    break;
                }
            }

            println!("final = {final_result}");
        }

        let linestr = lines.into_iter().nth(0).unwrap().unwrap().clone();
        parser(&mut &linestr[..]);
    }
}
