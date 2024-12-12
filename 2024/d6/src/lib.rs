pub mod p1;

use rayon::prelude::*;
use std::collections::HashSet;
use winnow::{combinator::alt, token::any, PResult, Parser};

fn parse_is_obstruction(input: &mut &str) -> PResult<Field> {
    "#".parse_next(input)?;
    Ok(Field::Obstruction)
}

fn parse_is_guard(input: &mut &str) -> PResult<Field> {
    "^".parse_next(input)?;
    Ok(Field::Guard)
}

fn parse_empty(input: &mut &str) -> PResult<Field> {
    ".".parse_next(input)?;
    Ok(Field::None)
}

fn parse_newline(input: &mut &str) -> PResult<Field> {
    alt(("\n", "\r\n", "\n\r")).parse_next(input)?;
    Ok(Field::NewLine)
}

fn parse_chr(input: &mut &str) -> PResult<Field> {
    any.parse_next(input)?;
    Ok(Field::Unknown)
}

type WHSize = (i32, i32);

fn parse_level(c: String) -> Result<(WHSize, Vec<Field>), ()> {
    let cstr = &mut &c[..];
    let mut y = 0;
    let mut retvec = vec![];
    loop {
        let fmatch = alt((
            parse_is_obstruction,
            parse_is_guard,
            parse_empty,
            parse_newline,
            parse_chr,
        ))
        .parse_next(cstr);
        if let Ok(f) = fmatch {
            match f {
                Field::Guard => retvec.push(f),
                Field::Obstruction => retvec.push(f),
                Field::None => retvec.push(f),
                Field::NewLine => {
                    y += 1;
                }
                Field::Unknown => break,
                Field::Visited => {}
            }
        } else {
            break;
        }
    }

    Ok((((retvec.len() / y - 1) as i32, (y + 1) as i32), retvec))
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Field {
    Guard,
    Obstruction,
    None,
    Visited,
    NewLine,
    Unknown,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum GuardDirection {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct PosXY {
    x: i32,
    y: i32,
}

fn get_xy_from_idx(idx: usize, field_width: i32, field_height: i32) -> PosXY {
    let pos_y = (idx as i32 / field_height) + 1;
    let pos_x = (idx as i32 % field_width) + 1;

    PosXY { x: pos_x, y: pos_y }
}

fn get_idx_from_xy(xy: &PosXY, field_height: i32) -> usize {
    (((xy.y - 1) * field_height) + (xy.x - 1)) as usize
}

fn print_map(fields: &[Field], field_width: i32, field_height: i32) {
    let mut curridx = 0;
    for _ in 0..field_height {
        for _ in 0..field_width {
            match fields[curridx] {
                Field::Guard => print!("^"),
                Field::Obstruction => print!("#"),
                Field::None => print!("."),
                Field::Visited => print!("x"),
                Field::NewLine => print!(""),
                Field::Unknown => print!(""),
            }
            curridx += 1;
        }
        println!("");
    }
}

#[derive(Debug)]
enum FieldSolve {
    Ok,
    Loop,
}

pub fn get_exit(
    fields: &mut Vec<Field>,
    field_size_w: i32,
    field_size_h: i32,
    guard_dir_in: GuardDirection,
) -> Result<FieldSolve, ()> {
    let mut previous_visits: HashSet<(usize, GuardDirection)> = HashSet::new();
    let mut guard_dir = guard_dir_in;
    let guard_pos_try_find = fields.iter().position(|x| *x == Field::Guard);
    assert!(guard_pos_try_find.is_some());

    let mut guard_pos_idx = guard_pos_try_find.unwrap();

    let mut guard_pos_xy = get_xy_from_idx(guard_pos_idx, field_size_w, field_size_h);

    let mut watchdog = 100000;
    loop {
        guard_pos_idx = get_idx_from_xy(&guard_pos_xy, field_size_h);
        let inserted = previous_visits.insert((guard_pos_idx, guard_dir));

        if !inserted {
            return Ok(FieldSolve::Loop);
        }

        let guard_pos_next = match guard_dir {
            GuardDirection::Up => PosXY {
                x: guard_pos_xy.x,
                y: guard_pos_xy.y - 1,
            },
            GuardDirection::Right => PosXY {
                x: guard_pos_xy.x + 1,
                y: guard_pos_xy.y,
            },
            GuardDirection::Down => PosXY {
                x: guard_pos_xy.x,
                y: guard_pos_xy.y + 1,
            },
            GuardDirection::Left => PosXY {
                x: guard_pos_xy.x - 1,
                y: guard_pos_xy.y,
            },
        };

        if guard_pos_next.y <= 0 {
            // println!("moved out of map up");
            break;
        } else if guard_pos_next.y > field_size_h {
            // println!("moved out of map down");
            break;
        } else if guard_pos_next.x <= 0 {
            // println!("moved out of map left");
            break;
        } else if guard_pos_next.x > field_size_w + 1 {
            // println!("moved out of map right");
            break;
        }

        let next_pos_idx = get_idx_from_xy(&guard_pos_next, field_size_h);

        match fields[next_pos_idx] {
            Field::Guard => {
                panic!("cannot happen. guard inception");
            }
            Field::Obstruction => {
                guard_dir = match guard_dir {
                    GuardDirection::Up => GuardDirection::Right,
                    GuardDirection::Right => GuardDirection::Down,
                    GuardDirection::Down => GuardDirection::Left,
                    GuardDirection::Left => GuardDirection::Up,
                };
                // println!("next one is guard, rotating 90 degree right");
                continue;
            }
            Field::None => {}
            Field::Visited => {}
            Field::NewLine => {}
            Field::Unknown => {}
        }

        fields[guard_pos_idx] = Field::Visited;
        fields[next_pos_idx] = Field::Guard;

        guard_pos_xy = guard_pos_next;

        watchdog -= 1;
        if watchdog == 1 {
            break;
        }
    }

    fields[guard_pos_idx] = Field::Visited;

    Ok(FieldSolve::Ok)
}

pub fn solve(contents: String) -> Result<i32, ()> {
    let parsed = parse_level(contents)?;

    let mut fields = parsed.1;
    let field_size_w = parsed.0 .0;
    let field_size_h = parsed.0 .1;

    let mut guard_dir = GuardDirection::Up;

    let exit_res = get_exit(&mut fields, field_size_w, field_size_h, guard_dir).unwrap();

    match exit_res {
        FieldSolve::Ok => {}
        FieldSolve::Loop => {}
    }

    let res: i32 = fields
        .iter()
        .map(|x| match x {
            Field::Guard => 1,
            Field::Obstruction => 0,
            Field::None => 0,
            Field::Visited => 1,
            Field::NewLine => 0,
            Field::Unknown => 0,
        })
        .sum();

    // println!("visited: {}", res);

    Ok(res)
}

// struct SolveError {}

pub fn solve_loop(contents: String) -> Result<i32, ()> {
    let parsed = parse_level(contents)?;
    let mut fields = parsed.1;
    let fields_orig = fields.clone();

    let field_size_w = parsed.0 .0;
    let field_size_h = parsed.0 .1;
    let guard_dir = GuardDirection::Up;

    // print_map(&fields, field_size_w, field_size_h);

    {
        // solve with exit
        _ = get_exit(&mut fields, field_size_w, field_size_h, guard_dir).unwrap();
    }
    // print_map(&fields, field_size_w, field_size_h);

    let mut idx_matches = vec![];

    for el_idx in 0..fields.len() {
        match fields[el_idx] {
            Field::Guard => {}
            Field::Obstruction => {}
            Field::None => {}
            Field::Visited => {
                if fields_orig[el_idx] == Field::Guard {
                    continue;
                }
                idx_matches.push(el_idx);
            }
            Field::NewLine => {}
            Field::Unknown => {}
        }
    }

    let loops: i32 = idx_matches
        .par_iter()
        .map(|el_idx| {
            let mut fields_check = fields_orig.clone();

            fields_check[*el_idx] = Field::Obstruction;
            let exit_val =
                get_exit(&mut fields_check, field_size_w, field_size_h, guard_dir).unwrap();
            // println!("exit val: {:?}", exit_val);
            match exit_val {
                FieldSolve::Ok => {}
                FieldSolve::Loop => {
                    let _ = get_xy_from_idx(*el_idx, field_size_w, field_size_h);
                    // println!("loop position obstruction: {}x{}", p.x, p.y);
                    // print_map(&fields, field_size_w, field_size_h);
                    return 1;
                }
            }
            0
        })
        .sum();

    Ok(loops)
}
