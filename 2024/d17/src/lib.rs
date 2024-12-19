use std::{
    collections::{HashMap, HashSet, VecDeque},
    thread::sleep,
    time::Duration,
};

use glam::IVec2;
use miette::miette;
use nom::{
    bytes::streaming::tag,
    character::complete::{self, anychar},
    multi::{many1, separated_list0, separated_list1},
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RegistryState {
    a: u64,
    b: u64,
    c: u64,
}

fn parse_registry(i: &str) -> IResult<&str, RegistryState> {
    let (i, nb_a) = preceded(tag("Register A: "), complete::u64)(i)?;
    let (i, _) = tag("\n")(i)?;

    let (i, nb_b) = preceded(tag("Register B: "), complete::u64)(i)?;
    let (i, _) = tag("\n")(i)?;

    let (i, nb_c) = preceded(tag("Register C: "), complete::u64)(i)?;
    Ok((
        i,
        RegistryState {
            a: nb_a,
            b: nb_b,
            c: nb_c,
        },
    ))
}

fn parse_instructions(i: &str) -> IResult<&str, Vec<u64>> {
    let (i, _) = tag("Program: ")(i)?;
    let (i, ret) = many1(anychar)(i)?; // could not solve with separated_list0, complete::u64 ...
    let nums: Vec<u64> = ret
        .iter()
        .filter(|x| **x != ',')
        .map(|x| x.to_digit(10).unwrap() as u64)
        .collect();
    Ok((i, nums))
}

fn parse_input(i: &str) -> IResult<&str, (RegistryState, Vec<u64>)> {
    let r = separated_pair(
        parse_registry,
        tag("\n\n"),
        parse_instructions,
    )(i)?;
    Ok((r.0, r.1)) // second flatten removes None options
}

fn get_combo_value(reg: &RegistryState, num: &u64) -> u64 {
    match num {
        0..=3 => *num,
        4 => reg.a,
        5 => reg.b,
        6 => reg.c,
        _ => {
            println!("ERRRORORORRRR 7");
            panic!("bye bye 777");
            // *num
        }
    }
}

fn get_and_increase_instr(instr: &Vec<u64>, idx: &mut usize) -> u64 {
    let ret = instr[*idx];
    *idx += 1;
    ret
}

fn run_program(
    instructions: &Vec<u64>,
    registry: &mut RegistryState,
) -> Vec<u64> {
    let mut instr_idx = 0;
    let mut output = vec![];
    loop {
        let instr =
            get_and_increase_instr(&instructions, &mut instr_idx);
        if instr == 0 {
            let literal_op =
                get_and_increase_instr(&instructions, &mut instr_idx);
            registry.a /= 2u64
                .pow(get_combo_value(&registry, &literal_op) as u32);
        } else if instr == 1 {
            let literal_op =
                get_and_increase_instr(&instructions, &mut instr_idx);
            registry.b ^= literal_op; // get_literal(&registry, &literal_op);
        } else if instr == 2 {
            let literal_op =
                get_and_increase_instr(&instructions, &mut instr_idx);
            registry.b = get_combo_value(&registry, &literal_op) % 8;
        } else if instr == 3 {
            if registry.a != 0 {
                let literal_op = get_and_increase_instr(
                    &instructions,
                    &mut instr_idx,
                );
                instr_idx = literal_op as usize;
            } else {
                instr_idx += 1;
            }
        } else if instr == 4 {
            let _literal_op =
                get_and_increase_instr(&instructions, &mut instr_idx);
            registry.b ^= registry.c;
        } else if instr == 5 {
            let literal_op =
                get_and_increase_instr(&instructions, &mut instr_idx);
            output.push(get_combo_value(&registry, &literal_op) % 8);
        } else if instr == 6 {
            let literal_op =
                get_and_increase_instr(&instructions, &mut instr_idx);
            registry.b = registry.a
                / 2u64
                    .pow(get_combo_value(&registry, &literal_op)
                        as u32);
        } else if instr == 7 {
            let literal_op =
                get_and_increase_instr(&instructions, &mut instr_idx);
            registry.c = registry.a
                / 2u64
                    .pow(get_combo_value(&registry, &literal_op)
                        as u32);
        }

        if instr_idx >= instructions.len() {
            break;
        }

        // sleep(Duration::from_millis(500));
    }
    output
}

pub fn solve_p1(
    contents: String,
) -> miette::Result<(RegistryState, String)> {
    println!("start solve p1");
    let (_, (mut registry, instructions)) =
        parse_input(&contents[..])
            .map_err(|e| miette!("could not parse, err: {}", e))?;

    // dbg!(&registry, &instructions);
    let output = run_program(&instructions, &mut registry);

    let out_str = output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");

    Ok((registry, out_str))
}

pub fn solve_p2(contents: String) -> miette::Result<(u64)> {
    println!("start solve p2");
    let (_, (mut registry, instructions)) =
        parse_input(&contents[..])
            .map_err(|e| miette!("could not parse, err: {}", e))?;

    let mut ii = 1;
    let reg_orig = registry.clone();
    loop {
        let mut reg_tmp = reg_orig.clone();
        reg_tmp.a = ii;
        let output = run_program(&instructions, &mut reg_tmp);
        if output == instructions {
            break;
        }
        if ii % 1_000_000 == 0 {
            println!("{}", ii);
        }
        ii += 1;
    }

    Ok(ii)
}

#[cfg(test)]
mod tests {
    use crate::{solve_p1, solve_p2};

    #[test]
    fn p1_1() -> miette::Result<()> {
        let (reg, out) = solve_p1(String::from(
            "Register A: 0
Register B: 0
Register C: 9

Program: 2,6",
        ))?;

        assert_eq!(reg.c, 9);

        Ok(())
    }

    #[test]
    fn p1_2() -> miette::Result<()> {
        let (reg, out) = solve_p1(String::from(
            "Register A: 10
Register B: 0
Register C: 0

Program: 5,0,5,1,5,4",
        ))?;

        assert_eq!(out, "0,1,2");
        Ok(())
    }

    #[test]
    fn p1_3() -> miette::Result<()> {
        let (reg, out) = solve_p1(String::from(
            "Register A: 2024
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
        ))?;

        assert_eq!(out, "4,2,5,6,7,7,7,7,3,1,0");
        Ok(())
    }

    #[test]
    fn p1_4() -> miette::Result<()> {
        let (reg, out) = solve_p1(String::from(
            "Register A: 0
Register B: 29
Register C: 0

Program: 1,7",
        ))?;

        assert_eq!(reg.b, 26);
        Ok(())
    }

    #[test]
    fn p1_5() -> miette::Result<()> {
        let (reg, out) = solve_p1(String::from(
            "Register A: 0
Register B: 2024
Register C: 43690

Program: 4,0",
        ))?;

        assert_eq!(reg.b, 44354);
        Ok(())
    }

    #[test]
    fn p1_run0() -> miette::Result<()> {
        let (reg, out) = solve_p1(String::from(
            "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0",
        ))?;

        println!("output: {}", out);
        println!("reg: {:?}", reg);

        assert_eq!(out, "4,6,3,5,6,3,5,2,1,0");
        Ok(())
    }

    #[test]
    fn p1_run1() -> miette::Result<()> {
        let (reg, out) = solve_p1(String::from(
            "Register A: 729
Register B: 0
Register C: 0

Program: 4,6,3,5,6,3,5,2,1,0",
        ))?;
        // 4,6,3,5,6,3,5,2,1,0

        println!("output: {}", out);

        assert_eq!(reg.b, 0);
        Ok(())
    }

    #[test]
    fn p1_print_itself() -> miette::Result<()> {
        let (reg, out) = solve_p1(String::from(
            "Register A: 117440
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0",
        ))?;
        // 4,6,3,5,6,3,5,2,1,0

        println!("output: {}", out);

        assert_eq!(reg.b, 0);
        Ok(())
    }

    #[test]
    fn p2_print_itself() -> miette::Result<()> {
        let (solution) = solve_p2(String::from(
            "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0",
        ))?;

        println!("solution: {}", solution);

        assert_eq!(solution, 117440);
        Ok(())
    }
}
