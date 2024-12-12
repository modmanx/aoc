use std::collections::{HashMap, HashSet};

use glam::IVec2;
use miette::miette;
use nom::{
    character::complete::{line_ending, satisfy},
    multi::{many1, separated_list1},
    IResult,
};
use nom_locate::{position, LocatedSpan};

pub type Span<'a> = LocatedSpan<&'a str>;

fn parse_line(i: Span) -> IResult<Span, (IVec2, char)> {
    let (i, pos) = position(i)?;
    let x = pos.get_column() as i32 - 1;
    let y = pos.location_line() as i32 - 1;
    let (i, c) = satisfy(|c| c.is_alphanumeric())(i)?;
    Ok((i, (IVec2::new(x, y), c)))
}

fn parse_input(i: Span) -> IResult<Span, HashMap<IVec2, char>> {
    let (i, res) = separated_list1(line_ending, many1(parse_line))(i)?;

    let r = res
        .iter()
        .flatten()
        .copied()
        .collect::<HashMap<IVec2, char>>();

    Ok((i, r))
}

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

#[derive(Debug)]
struct Area {
    area: u32,
    fields: Vec<IVec2>,
}

fn visit(pos: IVec2, visited: &mut HashSet<IVec2>, hm: &HashMap<IVec2, char>) -> Vec<IVec2> {
    // println!("  start visit {}", pos);
    let mut ret = vec![];
    ret.push(pos);
    visited.insert(pos);

    for d in DIRECTIONS {
        let new_dir = pos + d;
        // println!("    moving {} to {}", d, new_dir);
        let new_dir_entry = hm.get(&new_dir);
        if new_dir_entry.is_none() {
            // println!("    new dir not found");
            continue;
        }
        if visited.contains(&new_dir) {
            // println!("      already visited");
            continue;
        }
        if new_dir_entry.unwrap() != hm.get(&pos).unwrap() {
            // println!("      different character");
            continue;
        }

        let mut vis = visit(pos + d, visited, hm);

        ret.append(&mut vis);
    }

    ret
}

pub fn solve_p1(contents: String) -> miette::Result<String> {
    let (_, hm) = parse_input(Span::new(&contents[..]))
        .map_err(|x| miette!("got error while parsing {}", x))?;

    let width = hm.keys().map(|k| k.x).max().unwrap() + 1;
    let height = hm.keys().map(|k| k.y).max().unwrap() + 1;

    let mut visited: Vec<IVec2> = Vec::new();
    let mut areas: Vec<Area> = vec![];
    loop {
        let not_visited_field = hm.keys().find(|p| !visited.contains(p));
        if not_visited_field.is_none() {
            break;
        }

        let f = visit(*not_visited_field.unwrap(), &mut HashSet::new(), &hm);

        visited.append(&mut f.clone());

        areas.push(Area { area: 0, fields: f });
    }

    // println!("{:?}", areas);

    let total: u32 = areas
        .iter_mut()
        .map(|a| {
            a.area = a.fields.len() as u32;
            a.area
                * a.fields
                    .iter()
                    .map(|current_field| {
                        let mut ret = 4;
                        for d in DIRECTIONS {
                            if a.fields.iter().any(|neighbour_field_myb| {
                                *neighbour_field_myb == (*current_field + d)
                            }) {
                                ret -= 1;
                            }
                        }
                        ret
                    })
                    .sum::<u32>()
        })
        .sum();

    // println!("total: {}", total);

    Ok(format!("{}", total))
}

pub fn solve_p2(contents: String) -> miette::Result<String> {
    let (_, hm) = parse_input(Span::new(&contents[..]))
        .map_err(|x| miette!("got error while parsing {}", x))?;

    let mut visited: Vec<IVec2> = Vec::new();
    let mut areas: Vec<Area> = vec![];
    loop {
        let not_visited_field = hm.keys().find(|p| !visited.contains(p));
        if not_visited_field.is_none() {
            break;
        }
        let f = visit(*not_visited_field.unwrap(), &mut HashSet::new(), &hm);

        visited.append(&mut f.clone());

        areas.push(Area { area: 0, fields: f });
    }

    let total: u32 = areas
        .iter_mut()
        .map(|current_area| {
            current_area.area = current_area.fields.len() as u32;

            // count angles
            current_area
                .fields
                .iter()
                .map(|current_field| {
                    let mut cnt = 0;
                    if !current_area.fields.iter().any(|neighbour_field_myb| {
                        *neighbour_field_myb == (*current_field + IVec2::NEG_X)
                    }) && !current_area.fields.iter().any(|neighbour_field_myb| {
                        *neighbour_field_myb == (*current_field + IVec2::NEG_Y)
                    }) {
                        cnt += 1; // top left angle
                    }

                    if !current_area.fields.iter().any(|neighbour_field_myb| {
                        *neighbour_field_myb == (*current_field + IVec2::X)
                    }) && !current_area.fields.iter().any(|neighbour_field_myb| {
                        *neighbour_field_myb == (*current_field + IVec2::NEG_Y)
                    }) {
                        cnt += 1; // top right angle
                    }

                    if !current_area.fields.iter().any(|neighbour_field_myb| {
                        *neighbour_field_myb == (*current_field + IVec2::X)
                    }) && !current_area.fields.iter().any(|neighbour_field_myb| {
                        *neighbour_field_myb == (*current_field + IVec2::Y)
                    }) {
                        cnt += 1; // bottom right angle
                    }

                    if !current_area.fields.iter().any(|neighbour_field_myb| {
                        *neighbour_field_myb == (*current_field + IVec2::NEG_X)
                    }) && !current_area.fields.iter().any(|neighbour_field_myb| {
                        *neighbour_field_myb == (*current_field + IVec2::Y)
                    }) {
                        cnt += 1; // bottom left angle
                    }

                    if current_area.fields.iter().any(|neighbour_field_myb| {
                        *neighbour_field_myb == (*current_field + IVec2::X)
                    }) && current_area.fields.iter().any(|neighbour_field_myb| {
                        *neighbour_field_myb == (*current_field + IVec2::NEG_Y)
                    }) && !current_area.fields.iter().any(|neighbour_field_myb| {
                        *neighbour_field_myb == (*current_field + IVec2::X + IVec2::NEG_Y)
                    }) {
                        cnt += 1; // inner
                    }

                    if current_area.fields.iter().any(|neighbour_field_myb| {
                        *neighbour_field_myb == (*current_field + IVec2::NEG_X)
                    }) && current_area.fields.iter().any(|neighbour_field_myb| {
                        *neighbour_field_myb == (*current_field + IVec2::NEG_Y)
                    }) && !current_area.fields.iter().any(|neighbour_field_myb| {
                        *neighbour_field_myb == (*current_field + IVec2::NEG_X + IVec2::NEG_Y)
                    }) {
                        cnt += 1; // inner
                    }

                    if current_area.fields.iter().any(|neighbour_field_myb| {
                        *neighbour_field_myb == (*current_field + IVec2::NEG_X)
                    }) && current_area.fields.iter().any(|neighbour_field_myb| {
                        *neighbour_field_myb == (*current_field + IVec2::Y)
                    }) && !current_area.fields.iter().any(|neighbour_field_myb| {
                        *neighbour_field_myb == (*current_field + IVec2::NEG_X + IVec2::Y)
                    }) {
                        cnt += 1; // inner
                    }

                    if current_area.fields.iter().any(|neighbour_field_myb| {
                        *neighbour_field_myb == (*current_field + IVec2::X)
                    }) && current_area.fields.iter().any(|neighbour_field_myb| {
                        *neighbour_field_myb == (*current_field + IVec2::Y)
                    }) && !current_area.fields.iter().any(|neighbour_field_myb| {
                        *neighbour_field_myb == (*current_field + IVec2::X + IVec2::Y)
                    }) {
                        cnt += 1; // inner
                    }

                    cnt
                })
                .sum::<u32>()
                * current_area.area
        })
        .sum();

    // println!("total: {}", total);

    Ok(format!("{}", total))
}

#[cfg(test)]
mod tests {
    use crate::{solve_p1, solve_p2};

    #[test]
    fn p1_1() -> miette::Result<()> {
        let s = solve_p1(String::from(
            "AAAA
BBCD
BBCC
EEEC",
        ));

        assert_eq!("140", s?);
        Ok(())
    }
    #[test]
    fn p1_2() -> miette::Result<()> {
        let s = solve_p1(String::from(
            "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
        ));

        assert_eq!("772", s?);
        Ok(())
    }
    #[test]
    fn p2_1() -> miette::Result<()> {
        let s = solve_p2(String::from(
            "AAAA
BBCD
BBCC
EEEC",
        ));

        assert_eq!("80", s?);
        Ok(())
    }
    #[test]
    fn p2_2() -> miette::Result<()> {
        let s = solve_p2(String::from(
            "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
        ));

        assert_eq!("436", s?);
        Ok(())
    }
}
