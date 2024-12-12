use anyhow::Ok;

use std::{
    fs::File,
    io::{BufReader, Lines},
};

use winnow::{
    ascii::digit1,
    combinator::{alt, separated},
    PResult, Parser,
};

use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct AocError;

#[derive(Debug, Clone)]
enum NumberParser {
    Pair(i32, i32),
    Multi(Vec<i32>),
    None,
}

fn parse_pair(input: &mut &str) -> PResult<NumberParser> {
    let o = (digit1, "|", digit1).parse_next(input)?;
    std::result::Result::Ok(NumberParser::Pair(
        o.0.parse::<i32>().unwrap(),
        o.2.parse::<i32>().unwrap(),
    ))
}

fn parse_num_list(input: &mut &str) -> PResult<NumberParser> {
    let o: Vec<&str> = separated(1.., digit1, ",").parse_next(input)?;
    let nums = o.into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
    std::result::Result::Ok(NumberParser::Multi(nums))
}

fn parse_empty(input: &mut &str) -> PResult<NumberParser> {
    "".parse_next(input)?;
    std::result::Result::Ok(NumberParser::None)
}

fn parse_line(input: &mut &str) -> PResult<NumberParser> {
    alt((parse_pair, parse_num_list, parse_empty)).parse_next(input)
}

pub struct D5P1 {}

impl D5P1 {
    pub fn solve(&self, lines: Lines<BufReader<File>>) -> anyhow::Result<()> {
        // let lines_data: Vec<String> = lines.map(|x| x.unwrap()).collect();
        let mut nbs: Vec<NumberParser> = vec![];
        for l in lines.into_iter() {
            let line = l.unwrap();
            let parse_res = parse_line(&mut &line[..]);
            if let std::result::Result::Ok(pr) = parse_res {
                nbs.push(pr);
            }
        }

        let mut middles: Vec<i32> = vec![];
        for only_multi in nbs.iter() {
            match only_multi {
                NumberParser::Pair(_, _) => {}
                NumberParser::Multi(vec) => {
                    let mut is_ok = true;
                    for vec_idx in 0..vec.len() {
                        let check_num = vec[vec_idx];
                        for yy in vec_idx + 1..vec.len() {
                            let second_num = vec[yy];
                            println!("checking number pair {} {}", check_num, second_num);
                            let has_match = nbs.iter().any(|x| match x {
                                NumberParser::Pair(a, b) => *a == check_num && *b == second_num,
                                NumberParser::Multi(_) => false,
                                NumberParser::None => false,
                            });
                            if has_match {
                                println!("  has match");
                            } else {
                                println!("  no match");
                                is_ok = false;
                                break;
                            }
                        }
                        println!(" ")
                    }

                    if is_ok {
                        middles.push(vec[(vec.len() as f32 / 2f32).floor() as usize]);
                    }

                    println!("----------")
                }
                NumberParser::None => {}
            }
        }
        let sum: i32 = middles.iter().sum();
        println!("middles sum: {}", sum);

        Ok(())
    }
}

pub fn solvep2(lines: Lines<BufReader<File>>) -> anyhow::Result<()> {
    // let lines_data: Vec<String> = lines.map(|x| x.unwrap()).collect();
    // let mut nbs: Vec<NumberParser> = vec![];
    let mut nbs_pairs: Vec<(i32, i32)> = vec![];
    let mut nbs_lists: Vec<Vec<i32>> = vec![];
    for l in lines.into_iter() {
        let line = l.unwrap();
        let parse_res = parse_line(&mut &line[..]);
        if let std::result::Result::Ok(pr) = parse_res {
            match pr {
                NumberParser::Pair(x, y) => nbs_pairs.push((x, y)),
                NumberParser::Multi(vvv) => nbs_lists.push(vvv.clone()),
                NumberParser::None => {}
            }
            // nbs.push(pr);
        }
    }

    // let mut middles: Vec<i32> = vec![];

    let middles: Vec<i32> = nbs_lists
        .par_iter()
        .map(|vecc| {
            let mut is_ok = is_comb_ok(vecc, &nbs_pairs);
            let mut vecc2 = vecc.clone();
            if is_ok {
                return 0;
            }

            let mut jj = 0;
            println!("started fixing {:?}", vecc2);
            loop {
                let n1 = vecc2[jj];
                let n2 = vecc2[jj + 1];
                let mmm = nbs_pairs
                    .iter()
                    .find(|itm| itm.0 == n1 && itm.1 == n2 || itm.0 == n2 && itm.1 == n1);

                if let Some(pair) = mmm {
                    if n1 != pair.0 {
                        vecc2.swap(jj, jj + 1);
                    }
                }

                jj += 1;

                if jj > vecc2.len() - 2 {
                    jj = 0;
                }

                is_ok = is_comb_ok(&vecc2, &nbs_pairs);

                if is_ok {
                    return vecc2[(vecc2.len() as f32 / 2f32).floor() as usize];
                    // break
                }
            }
        })
        .collect();

    // for only_multi in nbs.par_iter() {

    // }
    let sum: i32 = middles.iter().sum();
    println!("middles sum: {}", sum);

    Ok(())
}

fn is_comb_ok(list: &[i32], nbs: &[(i32, i32)]) -> bool {
    for xx in 0..list.len() {
        let check_num = list[xx];
        for yy in xx + 1..list.len() {
            let second_num = list[yy];
            // println!("checking number pair {} {}", check_num, second_num);
            if !nbs.iter().any(|x| x.0 == check_num && x.1 == second_num) {
                return false;
            }
        }
        // println!("")
    }
    false
}
