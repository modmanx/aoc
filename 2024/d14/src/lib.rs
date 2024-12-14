use std::{collections::HashMap, thread::sleep, time::Duration};

use glam::IVec2;
use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::newline,
    multi::separated_list0,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug, Clone)]
struct ParsedLine {
    position: IVec2,
    velocity: IVec2,
}

fn parse_line(i: &str) -> IResult<&str, ParsedLine> {
    let (i, ((a, b), (c, d))) = separated_pair(
        preceded(
            tag("p="),
            separated_pair(
                nom::character::complete::i32,
                tag(","),
                nom::character::complete::i32,
            ),
        ),
        tag(" "),
        preceded(
            tag("v="),
            separated_pair(
                nom::character::complete::i32,
                tag(","),
                nom::character::complete::i32,
            ),
        ),
    )(i)?;

    Ok((
        i,
        ParsedLine {
            position: IVec2::new(a, b),
            velocity: IVec2::new(c, d),
        },
    ))
}

fn parse_input(i: &str) -> IResult<&str, Vec<ParsedLine>> {
    separated_list0(newline, parse_line)(i)
}

// fn print_map(map_width: &i32, map_height: &i32, p: &Vec<ParsedLine>) {
//     for yy in 0..*map_height {
//         for xx in 0..*map_width {
//             let el =
//                 p.iter().any(|pp| pp.position == IVec2::new(xx, yy));
//             if el {
//                 print!("o")
//             } else {
//                 print!(" ")
//             }
//         }
//         println!("");
//     }
// }

pub fn solve_p1(contents: String) -> miette::Result<String> {
    let (_, mut p) = parse_input(&contents)
        .map_err(|e| miette!("error parsing {}", e))?;

    let secs = 100;
    let map_width = p.iter().map(|x| x.position.x).max().unwrap() + 1;
    let map_height =
        p.iter().map(|x| x.position.y).max().unwrap() + 1;

    // let map_width = 11;
    // let map_height = 7;

    println!("map size {}x{}", map_width, map_height);

    p.iter_mut().for_each(|f| {
        f.position.x += secs * f.velocity.x;
        f.position.x %= map_width;

        f.position.x = match f.position.x {
            n if n < 0 => map_width - n.abs(),
            n => n,
        };

        f.position.y += secs * f.velocity.y;
        f.position.y %= map_height;

        f.position.y = match f.position.y {
            n if n < 0 => map_height - n.abs(),
            n => n,
        };
    });

    let map_quart = [
        IVec2::new(0, 0),
        IVec2::new(map_width / 2 - 1, map_height / 2 - 1),
    ];

    let map_quadrants = [
        map_quart,
        map_quart.map(|mut mq| {
            mq += IVec2::new(map_width / 2 + 1, 0);
            mq
        }),
        map_quart.map(|mut mq| {
            mq += IVec2::new(0, map_height / 2 + 1);
            mq
        }),
        map_quart.map(|mut mq| {
            mq += IVec2::new(map_width / 2 + 1, map_height / 2 + 1);
            mq
        }),
    ];

    let r2: Vec<i32> = map_quadrants
        .iter()
        .map(|q| {
            p.iter()
                .map(|xyxy| {
                    if xyxy.position.x >= q[0].x
                        && xyxy.position.x <= q[1].x
                        && xyxy.position.y >= q[0].y
                        && xyxy.position.y <= q[1].y
                    {
                        return 1;
                    }
                    0
                })
                .sum::<i32>()
        })
        .collect();

    let mut res = r2[0];
    (1..r2.len()).for_each(|x| {
        res *= r2[x];
    });

    Ok(format!("{}", res))
}

pub fn solve_p2(contents: String) -> miette::Result<String> {
    let (_, mut p) = parse_input(&contents)
        .map_err(|e| miette!("error parsing {}", e))?;

    let map_width = p.iter().map(|x| x.position.x).max().unwrap() + 1;
    let map_height =
        p.iter().map(|x| x.position.y).max().unwrap() + 1;

    println!("map size {}x{}", map_width, map_height);

    let mut max_row_store = 0i32;
    let mut max_col_store = 0i32;
    let mut ok_step = 0i32;
    for step in 0..1000000 {
        let mut per_row = HashMap::new();
        let mut per_col = HashMap::new();

        p.iter_mut().for_each(|f| {
            f.position.x += f.velocity.x;
            f.position.x %= map_width;

            f.position.x = match f.position.x {
                n if n < 0 => map_width - n.abs(),
                n => n,
            };

            f.position.y += f.velocity.y;
            f.position.y %= map_height;

            f.position.y = match f.position.y {
                n if n < 0 => map_height - n.abs(),
                n => n,
            };

            *per_row.entry(f.position.y).or_insert(0) += 1;
            *per_col.entry(f.position.x).or_insert(0) += 1;
        });

        let max_row = per_row.values().max().unwrap();
        let max_col = per_col.values().max().unwrap();

        if *max_row > max_row_store {
            println!("new max row: {max_row}");
            max_row_store = *max_row;
        }

        if *max_col > max_col_store {
            println!("new max col: {max_col}");
            max_col_store = *max_col;
        }

        if *max_row > 30 && *max_col > 30 {
            ok_step = step + 1;
            println!(
                "max row: {}  col: {}  step: {}",
                max_row,
                max_col,
                step + 1
            );
            break;
        }
    }

    Ok(format!("{}", ok_step))
}

#[cfg(test)]
mod tests {
    use crate::{solve_p1, solve_p2};

    #[test]
    fn p1_1() -> miette::Result<()> {
        let s = solve_p1(String::from(
            "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
        ));

        assert_eq!("12", s?);
        Ok(())
    }

    #[test]
    fn p2_1() -> miette::Result<()> {
        let s = solve_p2(String::from(
            "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
        ));

        assert_eq!("0", s?);
        Ok(())
    }
}
