use std::collections::HashMap;

use miette::miette;
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, multi::separated_list1,
    IResult,
};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

fn parse_line(i: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tag(" "), map_res(digit1, |s: &str| s.parse::<u64>()))(i)
}

fn calc_steps(
    curr_step: u64,
    curr_nb: u64,
    steps: u64,
    zero_cache: &mut HashMap<(u64, u64), u64>,
) -> u64 {
    if curr_step == steps {
        // zero_cache.insert((curr_step, curr_nb), 1);
        return 1;
    }

    if let Some(res) = zero_cache.get(&(curr_step, curr_nb)) {
        return *res;
    }

    let digits = curr_nb.checked_ilog10().unwrap_or(0) + 1;
    if digits % 2 == 0 {
        let nstr = curr_nb.to_string();

        let res = calc_steps(
            curr_step + 1,
            nstr[nstr.len() / 2..].parse::<u64>().unwrap(),
            steps,
            zero_cache,
        ) + calc_steps(
            curr_step + 1,
            nstr[0..nstr.len() / 2].parse::<u64>().unwrap(),
            steps,
            zero_cache,
        );

        zero_cache.insert((curr_step, curr_nb), res);
        res
    } else if curr_nb == 0 {
        let res = calc_steps(curr_step + 1, 1, steps, zero_cache);

        zero_cache.insert((curr_step, curr_nb), res);
        res
    } else {
        let res = calc_steps(curr_step + 1, curr_nb * 2024, steps, zero_cache);

        zero_cache.insert((curr_step, curr_nb), res);
        res
    }
}

pub fn solve(contents: String, mut blinks_left: u64) -> miette::Result<String> {
    let (_, stones) = parse_line(&contents[..]).map_err(|e| miette!("parse failed {}", e))?;

    let res: Vec<u64> = stones
        .par_iter()
        .map(|x| {
            let mut zero_cache: HashMap<(u64, u64), u64> = HashMap::default();
            calc_steps(1, *x, blinks_left + 1, &mut zero_cache)
        })
        .collect();

    // let res: u64 = stones
    //     .par_iter()
    //     .map(|x| {
    //         let mut zero_cache: HashMap<(u64, u64), u64> = HashMap::default();
    //         calc_steps(1, *x, blinks_left + 1, &mut zero_cache)
    //     })
    //     .collect()
    //     .sum();

    Ok(format!("{}", res.iter().sum::<u64>()))

    // ... I did few steps too much
    // let (_, stones) = parse_line(&contents[..]).map_err(|e| miette!("parse failed {}", e))?;

    // let start_time = Instant::now();

    // let mut framebuffer: Vec<Vec<u64>> = vec![vec![], vec![]];
    // framebuffer[0].reserve(10000000);
    // framebuffer[1].reserve(10000000);
    // let mut fb_idx = 0;
    // let mut fb_idx2 = 1;

    // framebuffer[fb_idx] = stones.clone();
    // let mut zero_cnt = 0;

    // let mut zeros: Vec<u64> = vec![];
    // let mut blinks_add = 0;

    // loop {
    //     println!("starting par chunks");

    //     for idx in 0..framebuffer[fb_idx].len() {
    //         match framebuffer[fb_idx][idx] {
    //             0 => {
    //                 // framebuffer[fb_idx2].push(1);
    //                 zeros.push(blinks_left);
    //                 // zero_cnt += 1;
    //             }
    //             n if (n.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 => {
    //                 let nstr = n.to_string();
    //                 framebuffer[fb_idx2].push(nstr[nstr.len() / 2..].parse::<u64>().unwrap());
    //                 framebuffer[fb_idx2].push(nstr[0..nstr.len() / 2].parse::<u64>().unwrap());
    //             }
    //             n => framebuffer[fb_idx2].push(n * 2024),
    //         };
    //     }

    //     println!(
    //         "blinks: {:?} zerocnt: {:?} fb_len: {:?}  stones: {:?} needed: {}",
    //         blinks_add,
    //         zero_cnt,
    //         0,
    //         framebuffer[fb_idx].len(),
    //         (Instant::now() - start_time).as_secs()
    //     );

    //     blinks_left -= 1;
    //     blinks_add += 1;
    //     if blinks_left == 0 {
    //         break;
    //     }
    //     framebuffer[fb_idx].clear();
    //     if fb_idx == 0 {
    //         fb_idx = 1;
    //         fb_idx2 = 0;
    //     } else if fb_idx == 1 {
    //         fb_idx = 0;
    //         fb_idx2 = 1;
    //     }
    // }

    // let mut nums = framebuffer[fb_idx2].len() as u64;

    // let t2: u64 = zeros
    //     .iter()
    //     .map(|blinks_to_calc| {
    //         let mut zero_cache: HashMap<(u64, u64), u64> = HashMap::default();
    //         calc_steps(1, 0, *blinks_to_calc + 1, &mut zero_cache)
    //     })
    //     .sum();

    // nums += t2;

    // println!("after {} blinks    {:?}", blinks_orig, stones);
}

pub fn solve_p2(contents: String) -> miette::Result<String> {
    Ok(format!("{}", 0))
}

#[cfg(test)]
mod tests {
    use crate::{solve, solve_p2};
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

    #[test]
    fn p1_score2_00000() -> miette::Result<()> {
        let s = solve(String::from("0"), 20);

        assert_eq!("55312", s?);
        Ok(())
    }
}
