use std::time::Instant;

use miette::miette;
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, multi::separated_list1,
    IResult,
};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

fn parse_line(i: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tag(" "), map_res(digit1, |s: &str| s.parse::<u64>()))(i)
}

// enum Ops {
//     ZeroToOne,
//     EvenDigitsTwoStones,

// }

pub fn solve(contents: String, mut blinks: u64) -> miette::Result<String> {
    println!("begin with stones:     {:?}", contents);
    let (_, mut stones) = parse_line(&contents[..]).map_err(|e| miette!("parse failed {}", e))?;
    stones.reserve(100000000);
    //                        2120125
    // let blinks_orig = blinks;
    let mut curr_idx = 0;
    let tstart: Instant = Instant::now();
    let mut to_sep: Vec<(usize, u64)> = vec![];
    loop {
        // let start_loop_instant = std::time::Instant::now();
        // let mut arm = 0;
        // idx, num
        match stones[curr_idx] {
            0 => {
                stones[curr_idx] = {
                    // arm = 0;
                    1
                }
            }
            n if (n.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 => {
                // arm = 1;
                stones.insert(curr_idx + 1, 0);
                to_sep.push((curr_idx, n));
                curr_idx += 1;
            }
            n => {
                // arm = 2;
                stones[curr_idx] = n * 2024;
            }
        }
        // let el = std::time::Instant::now() - start_loop_instant;
        // println!("arm: {arm} el: {}", el.as_secs_f32());
        // println!("{:?}", stones);
        curr_idx += 1;
        if curr_idx == stones.len() {
            // if to_sep.len() > 0 {
            //     println!("  tosep: {}", to_sep.len())
            // }
            let ttt: Vec<(usize, usize, u64, u64)> = to_sep
                .par_iter()
                .map(|(idx, y)| {
                    let mut n1 = 0u64;
                    let mut n2 = 0u64;

                    let mut v_copy = *y;
                    let mut digits: u32 = v_copy.checked_ilog10().unwrap_or(0) + 1;
                    let digits2 = digits / 2;
                    let mut times = 1;
                    while v_copy > 0 {
                        let n = v_copy % 10;
                        v_copy /= 10;
                        if digits > digits2 {
                            n2 += n * times;
                        } else {
                            n1 += n * times;
                        }
                        times *= 10;
                        digits -= 1;
                        if digits == digits2 {
                            times = 1;
                        }
                    }
                    (*idx, idx + 1, n1, n2)
                    // stones[curr_idx] = n1;
                    // curr_idx += 1;
                    // stones.insert(curr_idx, n2);
                })
                .collect();

            // println!("{:?}", to_sep);
            // println!("{:?}", ttt);

            ttt.into_iter().for_each(|x| {
                stones[x.0] = x.2;
                stones[x.1] = x.3;
            });

            to_sep = vec![];

            curr_idx = 0;
            blinks -= 1;
            println!(
                "blinks: {:?} curr_idx: {}   {:?}    needed: {}",
                blinks,
                curr_idx,
                stones.len(),
                (Instant::now() - tstart).as_secs_f32()
            );
        }
        if blinks == 0 {
            break;
        }
    }

    // println!("after {} blinks    {:?}", blinks_orig, stones);

    Ok(format!("{}", stones.len()))
}

pub fn solve_p2(contents: String) -> miette::Result<String> {
    Ok(format!("{}", 0))
}

#[cfg(test)]
mod tests {
    use crate::solve;
    // use miette::miette;

    #[test]
    fn p1_score2_22() -> miette::Result<()> {
        let s = solve(String::from("125 17"), 6);

        assert_eq!("22", s?);
        Ok(())
    }
    #[test]
    fn p1_score2_55312() -> miette::Result<()> {
        let s = solve(String::from("125 17"), 25);

        assert_eq!("55312", s?);
        Ok(())
    }
}
