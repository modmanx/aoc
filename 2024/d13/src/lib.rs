use miette::miette;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    multi::separated_list0,
    IResult,
};

use mathru::{
    algebra::linear::{
        matrix::{General, LUDec, LUDecomposition, Solve},
        vector::Vector,
    },
    matrix, vector,
};

use itertools::Itertools;

#[derive(Debug)]
enum ParsedLine {
    // ButtonA(u64, u64),
    // ButtonB(u64, u64),
    // Prize(u64, u64),
    Mover(u64, u64, u64, u64, u64, u64),

    Skip,
}

fn parsebuttonquizz(i: &str) -> IResult<&str, ParsedLine> {
    let (i, _) = tag("Button A: X+")(i)?;
    let (i, d1) = digit1(i)?;
    let (i, _) = tag(", Y+")(i)?;
    let (i, d2) = digit1(i)?;

    let (i, _) = tag("\n")(i)?;

    let (i, _) = tag("Button B: X+")(i)?;
    let (i, d3) = digit1(i)?;
    let (i, _) = tag(", Y+")(i)?;
    let (i, d4) = digit1(i)?;

    let (i, _) = tag("\n")(i)?;

    let (i, _) = tag("Prize: X=")(i)?;
    let (i, d5) = digit1(i)?;
    let (i, _) = tag(", Y=")(i)?;
    let (i, d6) = digit1(i)?;

    Ok((
        i,
        ParsedLine::Mover(
            d1.parse::<u64>().unwrap(),
            d2.parse::<u64>().unwrap(),
            d3.parse::<u64>().unwrap(),
            d4.parse::<u64>().unwrap(),
            d5.parse::<u64>().unwrap(),
            d6.parse::<u64>().unwrap(),
        ),
    ))
}

fn parseskip(i: &str) -> IResult<&str, ParsedLine> {
    let (_, _) = tag("\n")(i)?;
    Ok((i, ParsedLine::Skip))
}

fn parse_line(i: &str) -> IResult<&str, ParsedLine> {
    alt((parsebuttonquizz, parseskip))(i)
}

fn parse_input(i: &str) -> IResult<&str, Vec<ParsedLine>> {
    separated_list0(newline, parse_line)(i)
}

pub fn solve_p1_v1(contents: String) -> miette::Result<String> {
    let (_, parsed) = parse_input(&contents[..])
        .map_err(|err| miette!("could not parse ... {}", err))?;

    println!("parsed: {:?}", parsed);

    let res = parsed
        .iter()
        .map(|f| {
            if let ParsedLine::Mover(btn_a_x, btn_a_y, btn_b_x, btn_b_y, prize_x, prize_y) = f {
                println!("solving for {:?}", f);

                let mut cheapest = 999999999u64;
                let mut found = false;

                let min_x = btn_a_x.min(btn_b_x);
                let min_y = btn_a_y.min(btn_b_y);
                let max_presses_x: u64 = prize_x / min_x;
                let max_presses_y: u64 = prize_y / min_y;

                let max_presses = max_presses_x.max(max_presses_y);

                for btn_a_press in 0..max_presses {
                    let mut curr_pos_x = btn_a_x * btn_a_press;
                    let mut curr_pos_y = btn_a_y * btn_a_press;
                    let mut btn_b_press = 1;
                    loop {
                        curr_pos_x += btn_b_x;
                        curr_pos_y += btn_b_y;
                        if curr_pos_x == *prize_x && curr_pos_y == *prize_y {
                            println!("  found prize press A {btn_a_press} press B {btn_b_press}");
                            let price_for_moves = btn_a_press * 3 + btn_b_press;
                            if price_for_moves < cheapest {
                                cheapest = price_for_moves;
                                found = true;
                            }
                        }
                        if curr_pos_x > *prize_x || curr_pos_y > *prize_y {
                            break;
                        }
                        btn_b_press += 1;
                    }
                }
                if found {
                    return cheapest;
                } else {
                    return 0u64;
                }
            }
            0
        })
        .sum::<u64>();

    Ok(format!("{}", res))
}

pub fn solve_p1(contents: String) -> miette::Result<String> {
    let (_, parsed) = parse_input(&contents[..])
        .map_err(|err| miette!("could not parse ... {}", err))?;

    // println!("parsed: {:?}", parsed);

    let res = parsed
        .iter()
        .map(|f| {
            if let ParsedLine::Mover(
                btn_a_x,
                btn_a_y,
                btn_b_x,
                btn_b_y,
                mut prize_x,
                mut prize_y,
            ) = f
            {

                println!("solving {:?}", f);
                let a: General<f64> = matrix![  *btn_a_x as f64, *btn_b_x as f64 ;
                *btn_a_y as f64, *btn_b_y as f64
                ];

                let b: Vector<f64> = vector![prize_x as f64; prize_y as f64];

                // Decompose a into a lower and upper matrix
                let lu_dec: LUDec<f64> = a.dec_lu().unwrap();

                // Solve the system of linear equations with the decompose matrix
                let _x1: Vector<f64> = lu_dec.solve(&b).unwrap();

                let mut s1 = _x1[0];
                let mut s2 = _x1[1];

                let s1_diff = (s1 - s1.round()).abs();
                let s2_diff = (s2 - s2.round()).abs();

                if s1_diff > 0.0000001 || s2_diff > 0.0000001 {
                    return 0;
                }

                s1 = s1.round();
                s2 = s2.round();

                let multiplied = s1 * *btn_a_x as f64 + s2 * *btn_b_x as f64;

                if multiplied == prize_x as f64 {
                    println!("  solve ok - {}   {}     {}  {}  {}  {}",   s1, s2, s1_diff,   s1_diff == 0., s2_diff, s2_diff == 0. );
                    return s1 as u64 * 3 + s2 as u64;
                }
            }
            0
        })
        .sum::<u64>();

    Ok(format!("{}", res))
}

pub fn solve_p2(contents: String) -> miette::Result<String> {
    let (_, parsed) = parse_input(&contents[..])
        .map_err(|err| miette!("could not parse ... {}", err))?;

    // println!("parsed: {:?}", parsed);

    let res = parsed
        .iter()
        .map(|f| {
            if let ParsedLine::Mover(
                btn_a_x,
                btn_a_y,
                btn_b_x,
                btn_b_y,
                mut prize_x,
                mut prize_y,
            ) = f
            {

                prize_x += 10000000000000;
                prize_y += 10000000000000;

                println!("solving {:?}", f);
                let a: General<f64> = matrix![  *btn_a_x as f64, *btn_b_x as f64 ;
                *btn_a_y as f64, *btn_b_y as f64
                ];

                let b: Vector<f64> = vector![prize_x as f64; prize_y as f64];

                // Decompose a into a lower and upper matrix
                let lu_dec: LUDec<f64> = a.dec_lu().unwrap();

                // Solve the system of linear equations with the decompose matrix
                let _x1: Vector<f64> = lu_dec.solve(&b).unwrap();

                let mut s1 = _x1[0];
                let mut s2 = _x1[1];

                let s1_diff = (s1 - s1.round()).abs();
                let s2_diff = (s2 - s2.round()).abs();

                //  0.0000152587890625
                if s1_diff > 0.0001 || s2_diff > 0.0001 {
                    return 0;
                }

                s1 = s1.round();
                s2 = s2.round();

                let multiplied = s1 * *btn_a_x as f64 + s2 * *btn_b_x as f64;

                if multiplied == prize_x as f64 {
                    println!("  solve ok - {}   {}     {}  {}  {}  {}",   s1, s2, s1_diff,   s1_diff == 0., s2_diff, s2_diff == 0. );
                    return s1 as u64 * 3 + s2 as u64;
                }
            }
            0
        })
        .sum::<u64>();

    Ok(format!("{}", res))
}

#[cfg(test)]
mod tests {
    use crate::{solve_p1, solve_p2};

    #[test]
    fn p1_1() -> miette::Result<()> {
        let s = solve_p1(String::from(
            "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
        ));

        assert_eq!("480", s?);
        Ok(())
    }

    #[test]
    fn p2_1() -> miette::Result<()> {
        let s = solve_p2(String::from(
            "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
        ));

        assert_eq!("480", s?);
        Ok(())
    }
}
