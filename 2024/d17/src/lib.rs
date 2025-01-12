use itertools::Itertools;
use std::{f32::INFINITY, sync::Arc};

use miette::miette;
use nom::{
    bytes::streaming::tag,
    character::complete::{self, anychar},
    multi::many1,
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

fn get_and_increase_instr(instr: &[u64], idx: &mut usize) -> u64 {
    let ret = instr[*idx];
    *idx += 1;
    ret
}

#[derive(PartialEq, Eq)]
enum Instr {
    Adv0,
    Bxl1,
    Bst2,
    Jnz3,
    Bxc4,
    Out5,
    Bdv6,
    Cdv7,
    Unknown,
}

impl From<u64> for Instr {
    fn from(val: u64) -> Self {
        match val {
            0 => Instr::Adv0,
            1 => Instr::Bxl1,
            2 => Instr::Bst2,
            3 => Instr::Jnz3,
            4 => Instr::Bxc4,
            5 => Instr::Out5,
            6 => Instr::Bdv6,
            7 => Instr::Cdv7,
            _ => Instr::Unknown,
        }
    }
}

fn run_program(
    instructions: &[u64],
    registry: &mut RegistryState,
) -> Vec<u64> {
    let mut instr_idx = 0;
    let mut output = vec![];
    // println!(
    //     "start run_program    reg: {:?}     out: {:?}",
    //     registry, output
    // );
    loop {
        let instr =
            get_and_increase_instr(instructions, &mut instr_idx);
        let instr_enum: Instr = instr.into();
        // println!("working on {}", instr);
        match instr_enum {
            Instr::Adv0 => {
                let literal_op = get_and_increase_instr(
                    instructions,
                    &mut instr_idx,
                );
                // println!(
                //     " param {}   combo {}",
                //     literal_op,
                //     get_combo_value(registry, &literal_op) as u32
                // );
                registry.a /= 2u64
                    .pow(
                        get_combo_value(registry, &literal_op) as u32
                    );
            }
            Instr::Bxl1 => {
                let literal_op = get_and_increase_instr(
                    instructions,
                    &mut instr_idx,
                );
                // println!(" param {}", literal_op,);
                registry.b ^= literal_op; // get_literal(&registry, &literal_op);
            }
            Instr::Bst2 => {
                let literal_op = get_and_increase_instr(
                    instructions,
                    &mut instr_idx,
                );
                // println!(
                //     " param {}   combo {}",
                //     literal_op,
                //     get_combo_value(registry, &literal_op) as u32
                // );
                registry.b =
                    get_combo_value(registry, &literal_op) % 8;
            }
            Instr::Jnz3 => {
                if registry.a != 0 {
                    let literal_op = get_and_increase_instr(
                        instructions,
                        &mut instr_idx,
                    );
                    // println!(" param {}", literal_op,);
                    instr_idx = literal_op as usize;
                } else {
                    instr_idx += 1;
                }
            }
            Instr::Bxc4 => {
                let _literal_op = get_and_increase_instr(
                    instructions,
                    &mut instr_idx,
                );
                registry.b ^= registry.c;
            }
            Instr::Out5 => {
                let literal_op = get_and_increase_instr(
                    instructions,
                    &mut instr_idx,
                );
                // println!(
                //     " param {}   combo {}",
                //     literal_op,
                //     get_combo_value(registry, &literal_op) as u32
                // );
                output
                    .push(get_combo_value(registry, &literal_op) % 8);
            }
            Instr::Bdv6 => {
                let literal_op = get_and_increase_instr(
                    instructions,
                    &mut instr_idx,
                );
                // println!(
                //     " param {}   combo {}",
                //     literal_op,
                //     get_combo_value(registry, &literal_op) as u32
                // );
                registry.b = registry.a
                    / 2u64
                        .pow(get_combo_value(registry, &literal_op)
                            as u32);
            }
            Instr::Cdv7 => {
                let literal_op = get_and_increase_instr(
                    instructions,
                    &mut instr_idx,
                );
                // println!(
                //     " param {}   combo {}",
                //     literal_op,
                //     get_combo_value(registry, &literal_op) as u32
                // );
                registry.c = registry.a
                    / 2u64
                        .pow(get_combo_value(registry, &literal_op)
                            as u32);
            }
            Instr::Unknown => {}
        }
        // println!("  .. reg: {:?}     out: {:?}", registry, output);
        if instr_idx >= instructions.len() {
            break;
        }
    }
    output
}

fn run_program_with_out_match(
    instructions: &Vec<u64>,
    registry: &mut RegistryState,
) -> (bool, usize, std::vec::Vec<u64>) {
    let mut instr_idx = 0;
    let mut output = vec![];
    let mut matches = true;
    let mut out_idx = 0;

    loop {
        let instr =
            get_and_increase_instr(instructions, &mut instr_idx);
        let instr_enum: Instr = instr.into();

        match instr_enum {
            Instr::Adv0 => {
                let literal_op = get_and_increase_instr(
                    instructions,
                    &mut instr_idx,
                );
                registry.a /= 2u64
                    .pow(
                        get_combo_value(registry, &literal_op) as u32
                    );
            }
            Instr::Bxl1 => {
                let literal_op = get_and_increase_instr(
                    instructions,
                    &mut instr_idx,
                );
                registry.b ^= literal_op; // get_literal(&registry, &literal_op);
            }
            Instr::Bst2 => {
                let literal_op = get_and_increase_instr(
                    instructions,
                    &mut instr_idx,
                );
                registry.b =
                    get_combo_value(registry, &literal_op) % 8;
            }
            Instr::Jnz3 => {
                if registry.a != 0 {
                    let literal_op = get_and_increase_instr(
                        instructions,
                        &mut instr_idx,
                    );
                    instr_idx = literal_op as usize;
                } else {
                    instr_idx += 1;
                }
            }
            Instr::Bxc4 => {
                let _literal_op = get_and_increase_instr(
                    instructions,
                    &mut instr_idx,
                );
                registry.b ^= registry.c;
            }
            Instr::Out5 => {
                let literal_op = get_and_increase_instr(
                    instructions,
                    &mut instr_idx,
                );
                output
                    .push(get_combo_value(registry, &literal_op) % 8);
                if output[out_idx] != instructions[out_idx] {
                    matches = false;
                    break;
                }
                out_idx += 1;
            }
            Instr::Bdv6 => {
                let literal_op = get_and_increase_instr(
                    instructions,
                    &mut instr_idx,
                );
                registry.b = registry.a
                    / 2u64
                        .pow(get_combo_value(registry, &literal_op)
                            as u32);
            }
            Instr::Cdv7 => {
                let literal_op = get_and_increase_instr(
                    instructions,
                    &mut instr_idx,
                );
                registry.c = registry.a
                    / 2u64
                        .pow(get_combo_value(registry, &literal_op)
                            as u32);
            }
            Instr::Unknown => {}
        }
        if instr_idx >= instructions.len() {
            break;
        }
    }
    (matches && output == instructions.clone(), out_idx, output)
}

pub fn solve_p1(
    contents: String,
) -> miette::Result<(RegistryState, String)> {
    println!("start solve p1");
    let (_, (mut registry, instructions)) =
        parse_input(&contents[..])
            .map_err(|e| miette!("could not parse, err: {}", e))?;

    let output = run_program(&instructions, &mut registry);

    let out_str = output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");

    Ok((registry, out_str))
}

pub fn solve_p2(contents: String) -> miette::Result<u64> {
    let (_, (registry_orig, instructions_orig)) =
        parse_input(&contents[..])
            .map_err(|e| miette!("could not parse, err: {}", e))?;
    let mut best_match_out: Vec<u64> = vec![];
    let mut best_match_num = 0;
    let mut best_match_a: u64 = 0;
    // let start_num = 1;
    // let mut start_num = 10000431983002;
    let mut start_num = 1f64;
    // let start_num = 5017048261931305115;
    let mut mmax_num = 0;
    loop {
        // println!("sn: {}", start_num);
        let mut to_test = start_num;
        for _xy in 0..20 {
            to_test *= 8.;

            // if to_test == 10000431983002 {
            //     println!("totest: 10000431983002");
            // }

            let mut reg_tmp = registry_orig.clone();

            reg_tmp.a = to_test as u64;

            let asdgasgdasg = reg_tmp.a;

            let matches = run_program_with_out_match(
                &instructions_orig,
                &mut reg_tmp,
            );
            if matches.0 {
                println!("matches with ii = {} ", asdgasgdasg);
                return Ok(asdgasgdasg);
            }
            if matches.1 > best_match_num {
                best_match_a = asdgasgdasg;
                best_match_num = matches.1;
                best_match_out = matches.2;
                println!("{}", best_match_a);
                println!("{}", best_match_num);
                println!("{:?}", best_match_out);
            }
        }

        start_num += 0.0001;
    }
}

fn gen_nums(
    num: u64,
    nums: &mut Vec<u64>,
    max_steps: u32,
    current_step: u32,
) {
    if current_step > max_steps {
        return;
    }
    // println!("  gen_nums with {}", num);
    let n2 = num * 8;
    for xx in 0..16 {
        let xxx = xx - 8;
        let n3 = n2 as i64 + xxx;
        if n3 < 0 {
            continue;
        }
        if !nums.contains(&(n3 as u64)) {
            nums.push(n3 as u64);
        }
    }
    gen_nums(n2, nums, max_steps, current_step + 1);
}

pub fn solve_p2_blah3(contents: String) -> miette::Result<u64> {
    let (_, (mut registry_orig, mut instructions_orig)) =
        parse_input(&contents[..])
            .map_err(|e| miette!("could not parse, err: {}", e))?;
    let mut prev_max_ok = 0;
    let mut n = 0;
    loop {
        n += 1;
        let mut reg2 = registry_orig.clone();
        reg2.a = n;
        let out = run_program(&instructions_orig, &mut reg2);
        let mut instr_n = instructions_orig.len() - 1;
        let mut out_n = out.len() - 1;
        let mut is_ok = false;
        let mut found_solution = false;
        loop {
            if instructions_orig[instr_n] != out[out_n] {
                break;
            }

            if out_n == 0 {
                is_ok = true;
                break;
            }
            if instr_n == 0 {
                found_solution = true;
                break;
            }
            out_n -= 1;
            instr_n -= 1;
        }
        if is_ok {
            println!("n: {}     out: {:?}", n, out);
            if out.len() > 4 && out.len() > prev_max_ok {
                n *= 8;
                prev_max_ok = out.len();
            }
        }
        if found_solution {
            break;
        }
    }

    Ok(0)
}

pub fn solve_p2_blah2(contents: String) -> miette::Result<u64> {
    let (_, (mut registry_orig, mut instructions_orig)) =
        parse_input(&contents[..])
            .map_err(|e| miette!("could not parse, err: {}", e))?;

    println!("start gen nums");

    for xx in 0..100000 {
        let mut nums = vec![];
        gen_nums(xx, &mut nums, 30, 0);
        // println!("{:?}", nums);
        // println!("nums: {:?} ... start run_program", nums.len());

        for n in nums.iter() {
            let mut r = registry_orig.clone();
            r.a = *n;
            let out = run_program(&instructions_orig, &mut r);
            if out == instructions_orig {
                return Ok(*n);
            }
        }
    }

    Ok(0)
}

pub fn solve_p2_blah(contents: String) -> miette::Result<u64> {
    println!("start solve p2");
    let (_, (mut registry_orig, mut instructions_orig)) =
        parse_input(&contents[..])
            .map_err(|e| miette!("could not parse, err: {}", e))?;

    let mut instructions_to_work_with = instructions_orig.clone();

    // let instructions_orig_rev: Vec<u64> =
    //     instructions_orig.clone().into_iter().rev().collect();

    instructions_to_work_with
        .remove(instructions_to_work_with.len() - 1);
    instructions_to_work_with
        .remove(instructions_to_work_with.len() - 1);

    let instructions: Vec<u64> = instructions_to_work_with
        .chunks(2)
        .rev()
        .flatten()
        .copied()
        .collect();

    let incopts = vec![
        (0, 0, 0),
        (1, 0, 0),
        (2, 0, 0),
        (3, 0, 0),
        (4, 0, 0),
        (5, 0, 0),
        (6, 0, 0),
        (7, 0, 0),
        (0, 0, 0),
        (0, 1, 0),
        (0, 2, 0),
        (0, 3, 0),
        (0, 4, 0),
        (0, 5, 0),
        (0, 6, 0),
        (0, 7, 0),
        (0, 0, 0),
        (0, 0, 1),
        (0, 0, 2),
        (0, 0, 3),
        (0, 0, 4),
        (0, 0, 5),
        (0, 0, 6),
        (0, 0, 7),
    ];

    println!("rev: {:?}", instructions);

    registry_orig.a = 0;
    registry_orig.b = 0;
    registry_orig.c = 0;
    let mut out_idx = 0;

    let mut wdog = 2000000;
    let mut first_loop = true;

    let mut trying_registry = registry_orig.clone();
    let mut match_output_idx = instructions_orig.len() - 1;
    let mut mustexit = false;
    let mut finala = 0;

    'mainloop: loop {
        'forloop: for p in &incopts {
            let mut out_ok = false;
            let mut regreg = trying_registry.clone();
            let mut instr_idx = 0;
            regreg.a += p.0;
            regreg.b += p.1;
            regreg.c += p.2;

            println!(
                "starting loop with reg {:?}     must find {}",
                regreg, instructions_orig[match_output_idx]
            );

            loop {
                let instr = get_and_increase_instr(
                    &instructions,
                    &mut instr_idx,
                );
                let instr_enum: Instr = instr.into();

                println!("  working on {}", instr);

                match instr_enum {
                    Instr::Adv0 => {
                        let literal_op = get_and_increase_instr(
                            &instructions,
                            &mut instr_idx,
                        );
                        regreg.a *= 2u64.pow(get_combo_value(
                            &regreg,
                            &literal_op,
                        )
                            as u32);
                    }
                    Instr::Bxl1 => {
                        let literal_op = get_and_increase_instr(
                            &instructions,
                            &mut instr_idx,
                        );
                        regreg.b ^= literal_op; // get_literal(&regreg, &literal_op);
                    }
                    Instr::Bst2 => {
                        let literal_op = get_and_increase_instr(
                            &instructions,
                            &mut instr_idx,
                        );
                        regreg.b =
                            get_combo_value(&regreg, &literal_op) % 8;
                    }
                    Instr::Jnz3 => {
                        if regreg.a != 0 {
                            let literal_op = get_and_increase_instr(
                                &instructions,
                                &mut instr_idx,
                            );
                            instr_idx = literal_op as usize;
                        } else {
                            instr_idx += 1;
                        }
                    }
                    Instr::Bxc4 => {
                        let _literal_op = get_and_increase_instr(
                            &instructions,
                            &mut instr_idx,
                        );
                        regreg.b ^= regreg.c;
                    }
                    Instr::Out5 => {
                        let literal_op = get_and_increase_instr(
                            &instructions,
                            &mut instr_idx,
                        );
                        let outttt =
                            get_combo_value(&regreg, &literal_op) % 8;
                        if outttt
                            == instructions_orig[match_output_idx]
                        {
                            println!("    output OK: {:?}", outttt);
                            trying_registry = regreg.clone();
                            if match_output_idx == 0 {
                                println!("    must exit aaa");
                                mustexit = true;
                            } else {
                                match_output_idx -= 1;
                            }
                            out_ok = true;
                        }

                        out_idx += 1;
                    }
                    Instr::Bdv6 => {
                        let literal_op = get_and_increase_instr(
                            &instructions,
                            &mut instr_idx,
                        );
                        regreg.b = regreg.a
                            * 2u64.pow(get_combo_value(
                                &regreg,
                                &literal_op,
                            )
                                as u32);
                    }
                    Instr::Cdv7 => {
                        let literal_op = get_and_increase_instr(
                            &instructions,
                            &mut instr_idx,
                        );
                        regreg.c = regreg.a
                            * 2u64.pow(get_combo_value(
                                &regreg,
                                &literal_op,
                            )
                                as u32);
                    }
                    Instr::Unknown => {}
                }

                println!("  reg: {:?}", regreg);

                if instr_idx >= instructions.len() {
                    if out_ok {
                        trying_registry = regreg.clone();
                        println!(
                            "ok ... continuing with reg {:?}",
                            trying_registry
                        );
                        break 'forloop;
                    } else {
                        println!("  not found");
                        break;
                    }
                }
            }

            wdog -= 1;
            if wdog < 0 {
                break;
            }
        }

        if mustexit {
            finala = trying_registry.a;
            break 'mainloop;
        }
    }

    Ok(finala)

    // for ii in 0..1135184380000000 {
    //     let mut reg_tmp = reg_orig.clone();
    //     reg_tmp.a = ii;
    //     if ii % 1_000_000 == 0 {
    //         println!("at {ii}");
    //     }
    //     let matches =
    //         run_program_with_out_match(&instructions, &mut reg_tmp);
    //     if matches {
    //         println!("matches with ii = {ii}    final registry {reg_tmp:?}");
    //         return Ok(ii);
    //     }
    // }

    // Ok(0)

    // let mut ii = 0;
    // // let mut ttt = 1;
    // let reg_orig = registry.clone();
    // let mut found_len = false;
    // let mut found_match_12 = false;

    // loop {
    //     // ttt += 1;
    //     let mut reg_tmp = reg_orig.clone();
    //     reg_tmp.a = ii;
    //     let output = run_program(&instructions, &mut reg_tmp);
    //     if output == instructions {
    //         break;
    //     }
    //     // if ii % 1_000_000 == 0 {
    //     //     println!("{}    {output:?}", ii);
    //     // }

    //     if output.len() == instructions.len() {
    //         if !found_match_12
    //             && output[output.len() - 1]
    //                 == instructions[instructions.len() - 1]
    //             && output[output.len() - 2]
    //                 == instructions[instructions.len() - 2]
    //             && output[output.len() - 3]
    //                 == instructions[instructions.len() - 3]
    //             && output[output.len() - 4]
    //                 == instructions[instructions.len() - 4]
    //         {
    //             if !found_match_12 {
    //                 println!("found 12 match at {ii}");
    //             }
    //             found_match_12 = true;
    //         }

    //         if !found_len {
    //             println!("found len at {ii}");
    //             found_len = true;
    //         }
    //         if found_match_12 {
    //             ii += 1;
    //         } else {
    //             ii += 1_000_000;
    //         }

    //         // println!("{}    {output:?}", ii);
    //     } else {
    //         if found_len {
    //             println!("len no more at {ii}");
    //             break;
    //         }
    //         ii += 1_000_000;
    //     }
    // }

    // // 35184380000000
    // // 281474980000000
    // Ok(ii)
}

#[cfg(test)]
mod tests {
    use crate::{solve_p1, solve_p2, solve_p2_blah2};

    #[test]
    fn p1_1() -> miette::Result<()> {
        let (reg, _out) = solve_p1(String::from(
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
        let (_reg, out) = solve_p1(String::from(
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
        let (_reg, out) = solve_p1(String::from(
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
        let (reg, _out) = solve_p1(String::from(
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
        let (reg, _out) = solve_p1(String::from(
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
        let solution = solve_p2_blah2(String::from(
            "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0",
        ))?;

        println!("solution: {}", solution);

        assert_eq!(solution, 117440);
        Ok(())
    }

    #[test]
    fn p2_print_steps() -> miette::Result<()> {
        let solution = solve_p1(String::from(
            "Register A: 117440
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0",
        ))?;

        Ok(())
    }
}
