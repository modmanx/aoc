use glam::IVec2;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit0, digit1},
    multi::{many1, separated_list0},
    sequence::{separated_pair, terminated},
    IResult, Parser,
};
use pathfinding::prelude::dijkstra;

use std::{f32::INFINITY, sync::Arc};

use miette::miette;

fn parse(s: &str) -> IResult<&str, Vec<(i32, i32)>> {
    let r = separated_list0(tag("\n"), separated_pair(digit0, tag(","), digit0))(s)?;
    let res =
        r.1.into_iter()
            .map(|i| (i.0.parse::<i32>().unwrap(), i.1.parse::<i32>().unwrap()))
            .collect();
    Ok((r.0, res))
}

pub fn solve_p1(contents: String, steps: usize) -> miette::Result<String> {
    println!("start solve p1");

    let (_, corrupted_all) =
        parse(&contents[..]).map_err(|e| miette!("could not parse, err: {}", e))?;

    let w = corrupted_all.iter().map(|x| x.0).max().unwrap() + 1;
    let h = corrupted_all.iter().map(|x| x.1).max().unwrap() + 1;

    // print_map(w, h, &corrupted_all);

    let goal = (w - 1, h - 1);

    // let mut ci = 0;
    let mut corrupted = vec![];

    for ii in 0..steps {
        corrupted.push(corrupted_all[ii]);
    }

    let result = dijkstra(
        &(0, 0),
        |&(x, y)| {
            let mut res = vec![];
            if x > 0 && !corrupted.contains(&(x - 1, y)) {
                res.push((x - 1, y));
            }
            if x < w - 1 && !corrupted.contains(&(x + 1, y)) {
                res.push((x + 1, y));
            }
            if y > 0 && !corrupted.contains(&(x, y - 1)) {
                res.push((x, y - 1));
            }
            if y < h - 1 && !corrupted.contains(&(x, y + 1)) {
                res.push((x, y + 1));
            }
            // println!("currently at {:?}    opts: {:?}", (x, y), &res);

            // corrupted.push(corrupted_all[ci]);
            // ci += 1;

            res.into_iter().map(|p| (p, 1))
        },
        |&p| p == goal,
    );

    if let Some(res) = result {
        Ok(res.1.to_string())
    } else {
        Ok(String::from("0"))
    }
}

pub fn solve_p2(contents: String) -> miette::Result<String> {
    println!("start solve p2");

    let (_, corrupted_all) =
        parse(&contents[..]).map_err(|e| miette!("could not parse, err: {}", e))?;

    let w = corrupted_all.iter().map(|x| x.0).max().unwrap() + 1;
    let h = corrupted_all.iter().map(|x| x.1).max().unwrap() + 1;

    // print_map(w, h, &corrupted_all);

    let goal = (w - 1, h - 1);

    // let mut ci = 0;
    let mut corrupted = vec![];
    let mut is_ok_max = 0;

    for ii in 0..corrupted_all.len() {
        println!("current step {}", ii);
        corrupted.push(corrupted_all[ii]);

        let result = dijkstra(
            &(0, 0),
            |&(x, y)| {
                let mut res = vec![];
                if x > 0 && !corrupted.contains(&(x - 1, y)) {
                    res.push((x - 1, y));
                }
                if x < w - 1 && !corrupted.contains(&(x + 1, y)) {
                    res.push((x + 1, y));
                }
                if y > 0 && !corrupted.contains(&(x, y - 1)) {
                    res.push((x, y - 1));
                }
                if y < h - 1 && !corrupted.contains(&(x, y + 1)) {
                    res.push((x, y + 1));
                }
                // println!("currently at {:?}    opts: {:?}", (x, y), &res);

                // corrupted.push(corrupted_all[ci]);
                // ci += 1;

                res.into_iter().map(|p| (p, 1))
            },
            |&p| p == goal,
        );

        if result.is_some() {
            is_ok_max = ii;
        } else {
            break;
        }
    }

    Ok(format!(
        "{},{}",
        corrupted_all[is_ok_max + 1].0,
        corrupted_all[is_ok_max + 1].1
    ))
}

fn print_map(w: i32, h: i32, corrupted: &Vec<(i32, i32)>) {
    for yy in 0..h {
        for xx in 0..w {
            if corrupted.contains(&(xx, yy)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{solve_p1, solve_p2};

    #[test]
    fn p1_1() -> miette::Result<()> {
        let res = solve_p1(
            String::from(
                "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0",
            ),
            12,
        )?;

        assert_eq!(res, "22");

        Ok(())
    }

    #[test]
    fn p2_1() -> miette::Result<()> {
        let res = solve_p2(String::from(
            "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0",
        ))?;

        assert_eq!(res, "6,1");

        Ok(())
    }
}
