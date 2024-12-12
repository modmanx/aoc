use core::num;

use itertools::Itertools;
use nom::{bytes::complete::tag, character::complete::digit0, multi::separated_list0, IResult};
use rayon::prelude::*;

pub mod p1;

#[derive(Debug, Clone)]
struct LineMatch {
    result: i64,
    nums: Vec<i64>,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Op {
    Num(i64),
    Times,
    Add,
    Concat,
}

fn parse_line(i: &str) -> IResult<&str, Vec<&str>> {
    // let pp = terminated(take_until(tag("\r\n")), tag("\r\n"))(i);
    let (input, result) = digit0(i)?;
    let (input, _) = tag(": ")(input)?;
    let (input, nums) = separated_list0(tag(" "), digit0)(input)?;
    Ok((result, nums))
}

// fn parse(i: &str) -> IResult<&str, &str> {
//     println!("parse started: ");
//     let pp = dbg!(many0(parse_line)(i));

//     println!("pp: {:?}", pp);
//     Ok(("", ""))
// }

pub fn solve(contents: String) -> Result<i64, ()> {
    let lines: Vec<String> = contents.split("\n").into_iter().map(String::from).collect();

    let total_res: i64 = lines
        .par_iter()
        .map(|l| {
            let ll = &mut &l;
            let res = parse_line(ll);
            if let Ok(r) = res {
                let to_check = r.0.parse::<i64>().unwrap();
                let nums = r.1.into_iter().map(|x| x.parse::<i64>().unwrap()).collect();
                return LineMatch {
                    result: to_check,
                    nums,
                };
            }
            LineMatch {
                result: 0,
                nums: vec![],
            }
        })
        .filter(|x| !x.nums.is_empty())
        .filter_map(|line_match| {
            let v = vec![Op::Times, Op::Add];
            let nums: Vec<Op> = line_match.nums.clone().into_iter().map(Op::Num).collect();
            let comb_num = nums.len() - 1;
            let to_check = line_match.result.clone();
            let is_solvable = v
                .into_iter()
                .combinations_with_replacement(comb_num)
                .flat_map(|cc| cc.into_iter().permutations(comb_num).unique())
                .find_map(|x| {
                    let to_test: Vec<Op> = nums.clone().into_iter().interleave(x).collect();
                    let mut total = 0;
                    let mut curr_op = Op::Add;
                    for x in to_test {
                        match x {
                            Op::Num(n) => match curr_op {
                                Op::Num(_) | Op::Concat => {}
                                Op::Times => total *= n,
                                Op::Add => total += n,
                            },
                            Op::Times => curr_op = Op::Times,
                            Op::Add => curr_op = Op::Add,
                            Op::Concat => curr_op = Op::Add,
                        }
                        if total > to_check {
                            return None;
                        }
                    }
                    if total == to_check {
                        Some(total)
                    } else {
                        None
                    }
                });
            if is_solvable.is_some() {
                println!(
                    "{:?} is solvable, result: {}",
                    line_match,
                    is_solvable.unwrap()
                );
                Some(is_solvable.unwrap())
            } else {
                None
            }
        })
        .sum();

    Ok(total_res)
}

pub fn solve_concat(contents: String) -> Result<i64, ()> {
    let lines: Vec<String> = contents.split("\n").into_iter().map(String::from).collect();

    let total_res: i64 = lines
        // .par_iter()
        .into_iter()
        .map(|l| {
            let ll = &mut &l;
            let res = parse_line(ll);
            if let Ok(r) = res {
                let to_check = r.0.parse::<i64>().unwrap();
                let nums = r.1.into_iter().map(|x| x.parse::<i64>().unwrap()).collect();
                return LineMatch {
                    result: to_check,
                    nums,
                };
            }
            LineMatch {
                result: 0,
                nums: vec![],
            }
        })
        .filter(|x| !x.nums.is_empty())
        // .map(|x| {println!("before filtermap: {:?}", x); x})
        .filter_map(|line_match| {
            let v = vec![Op::Times, Op::Add, Op::Concat];
            let nums: Vec<Op> = line_match.nums.clone().into_iter().map(Op::Num).collect();
            // println!("nums: {:?}", nums);
            let comb_num = nums.len() - 1;
            let to_check = line_match.result;
            let is_solvable = v
                .into_iter()
                .combinations_with_replacement(comb_num)
                .flat_map(|cc| cc.into_iter().permutations(comb_num).unique())
                .find_map(|ops: Vec<Op>| {
                    let to_test: Vec<Op> = nums.clone().into_iter().interleave(ops).collect();
                    // println!("  to test: {:?}", to_test);

                    let res = to_test.iter().fold((0, Op::Add), |mut acc, x| {
                        match x {
                            Op::Num(n) => match acc.1 {
                                Op::Times => acc.0 *= n,
                                Op::Add => acc.0 += n,
                                Op::Concat => {
                                    acc.0 = format!("{}{}", acc.0, n).parse::<i64>().unwrap()
                                }
                                Op::Num(_) => {}
                            },
                            Op::Times => acc.1 = Op::Times,
                            Op::Add => acc.1 = Op::Add,
                            Op::Concat => acc.1 = Op::Concat,
                        };
                        acc
                    });

                    // for x in to_test {
                    //     match x {
                    //         Op::Num(n) => match curr_op {
                    //             Op::Num(_) => {}
                    //             Op::Times => total *= n,
                    //             Op::Add => total += n,
                    //             Op::Concat => {
                    //                 total = format!("{}{}", total, n).parse::<i64>().unwrap()
                    //             }
                    //         },
                    //         Op::Times => curr_op = Op::Times,
                    //         Op::Add => curr_op = Op::Add,
                    //         Op::Concat => curr_op = Op::Concat,
                    //     }
                    //     if total > to_check {
                    //         return None;
                    //     }
                    // }
                    if res.0 == to_check {
                        Some(res.0)
                    } else {
                        None
                    }
                });
            if is_solvable.is_some() {
                println!(
                    "  {:?} is solvable, result: {}",
                    line_match,
                    is_solvable.unwrap()
                );
                Some(is_solvable.unwrap())
            } else {
                None
            }
        })
        .sum();

    Ok(total_res)
}
