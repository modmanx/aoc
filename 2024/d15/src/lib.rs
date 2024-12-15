use std::{
    collections::{HashMap, HashSet},
    thread::sleep,
    time::Duration,
};

use glam::{BVec2, IVec2};
use itertools::enumerate;
use miette::miette;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, satisfy},
    multi::{many0, many1, separated_list0, separated_list1},
    sequence::separated_pair,
    IResult,
};
use nom_locate::{position, LocatedSpan};

type Span<'a> = LocatedSpan<&'a str>;

fn parse_map(i: Span) -> IResult<Span, MapEntry> {
    let (i, pos) = position(i)?;
    let x = pos.get_column() as i32 - 1;
    let y = pos.location_line() as i32 - 1;
    let (i, c) = satisfy(|c| c != '\n')(i)?;
    Ok((
        i,
        match c {
            'O' => MapEntry {
                thing: Thing::Box,
                pos: IVec2::new(x, y),
                is_left: false,
                is_right: false,
                chr: 'x',
            },
            '@' => MapEntry {
                thing: Thing::Robot,
                pos: IVec2::new(x, y),
                is_left: false,
                is_right: false,
                chr: 'x',
            },
            '#' => MapEntry {
                thing: Thing::Wall,
                pos: IVec2::new(x, y),
                is_left: false,
                is_right: false,
                chr: 'x',
            },
            _ => MapEntry {
                thing: Thing::None,
                pos: IVec2::new(x, y),
                is_left: false,
                is_right: false,
                chr: 'x',
            },
        },
    ))
}

fn parse_moves(i: Span) -> IResult<Span, IVec2> {
    let (i, ch) = anychar(i)?;
    let r = match ch {
        '^' => IVec2::NEG_Y,
        '>' => IVec2::X,
        'v' => IVec2::Y,
        '<' => IVec2::NEG_X,
        _ => IVec2::new(0, 0),
    };
    Ok((i, r))
}

#[derive(Debug, Clone)]
struct MapEntry {
    thing: Thing,
    pos: IVec2,
    is_left: bool,
    is_right: bool,
    chr: char,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Thing {
    Robot,
    Box,
    Wall,
    None,
}

// #[derive(Debug, PartialEq, Eq)]
// enum Move {
//     Up,
//     Right,
//     Down,
//     Left,
//     Nowhere,
// }

fn parse_input(
    i: Span,
) -> IResult<Span, (Vec<MapEntry>, Vec<IVec2>)> {
    let r = separated_pair(
        separated_list1(tag("\n"), many1(parse_map)),
        tag("\n\n"),
        separated_list1(tag("\n"), many1(parse_moves)),
    )(i)?;
    Ok((
        r.0,
        (
            r.1 .0.into_iter().flatten().collect::<Vec<MapEntry>>(),
            r.1 .1.into_iter().flatten().collect(),
        ),
    ))
}

fn print_map(m: &[MapEntry], robot_pos: &IVec2, w: &i32, h: &i32) {
    for yy in 0..*h {
        for xx in 0..*w {
            if *robot_pos == IVec2::new(xx, yy) {
                print!("@");
            } else if let Some(x) =
                m.iter().find(|me| me.pos == IVec2::new(xx, yy))
            {
                match x.thing {
                    Thing::Box => {
                        if x.is_left {
                            print!("[")
                        } else if x.is_right {
                            print!("]")
                        } else {
                            print!("o")
                        }
                    }
                    Thing::Wall => print!("#"),
                    _ => print!("."),
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn solve_p1(contents: String) -> miette::Result<String> {
    let contents_span = Span::new(&contents[..]);
    let (_, (map, moves)) = parse_input(contents_span)
        .map_err(|e| miette!("could not parse, err: {}", e))?;

    println!("moves: {}", moves.len());

    let map_width = map.iter().map(|x| x.pos.x).max().unwrap() + 1;
    let map_height = map.iter().map(|x| x.pos.y).max().unwrap() + 1;

    let mut robot_pos = map
        .iter()
        .find(|x| {
            if Thing::Robot == x.thing {
                return true;
            }
            false
        })
        .unwrap()
        .pos
        .clone(); // must be

    let mut map2: Vec<MapEntry> = map
        .iter()
        .filter(|x| x.thing == Thing::Wall || x.thing == Thing::Box)
        .cloned()
        .collect();

    print_map(&map2, &robot_pos, &map_width, &map_height);

    // println!("robot pos: {:?}", robot_pos);
    // let mut iii = 8;
    for m in moves {
        let mut nextpos = robot_pos + m;
        // let mut untilpos = robot_pos;
        let mut to_move: Vec<usize> = vec![];
        let mut move_for = IVec2::new(0, 0);

        // println!(
        //     "move {}",
        //     match m {
        //         IVec2::X => "right",
        //         IVec2::NEG_X => "left",
        //         IVec2::Y => "down",
        //         IVec2::NEG_Y => "up",
        //         _ => "",
        //     }
        // );
        // println!();
        // println!();

        // find items to move
        loop {
            if let Some(next) =
                map2.iter().enumerate().find(|f| f.1.pos == nextpos)
            {
                if next.1.thing == Thing::Wall {
                    break;
                }
                // robot_pos += m;
                to_move.push(next.0);
                // untilpos = robocopy;
            } else {
                // next pos is empty space
                move_for = m;
                break;
            }
            nextpos += m;
        }

        // robot_pos = robocopy;

        // println!("move idx {:?}", to_move);

        for me in to_move {
            map2[me].pos += move_for;
        }

        robot_pos += move_for;

        // print_map(&map2, &robot_pos, &map_width, &map_height);

        // println!();
        // println!();
    }

    println!();
    println!("finished");
    println!();
    print_map(&map2, &robot_pos, &map_width, &map_height);

    let res = map2
        .iter()
        .map(|me| {
            if me.thing == Thing::Box {
                return me.pos.y * 100 + me.pos.x;
            }
            0
        })
        .sum::<i32>();

    Ok(format!("{}", res))
}

pub fn solve_p2(contents: String) -> miette::Result<String> {
    let contents_span = Span::new(&contents[..]);
    let (_, (map, moves)) = parse_input(contents_span)
        .map_err(|e| miette!("could not parse, err: {}", e))?;

    println!("solve part 2");
    println!("moves: {}", moves.len());

    let mut robot_pos = map
        .iter()
        .find(|x| {
            if Thing::Robot == x.thing {
                return true;
            }
            false
        })
        .unwrap()
        .pos; // must be

    robot_pos.x *= 2;

    let mut map2: Vec<MapEntry> = vec![];
    for me in map
        .iter()
        .filter(|x| x.thing == Thing::Wall || x.thing == Thing::Box)
    {
        if me.thing == Thing::Wall || me.thing == Thing::Box {
            let xpos = IVec2::new(me.pos.x * 2, me.pos.y);
            map2.push(MapEntry {
                thing: me.thing.clone(),
                pos: xpos,
                is_left: true,
                is_right: false,
                chr: '1',
            });
            map2.push(MapEntry {
                thing: me.thing.clone(),
                pos: xpos + IVec2::new(1, 0),
                is_left: false,
                is_right: true,
                chr: '2',
            });
        }
    }

    let map_width = map2.iter().map(|x| x.pos.x).max().unwrap() + 1;
    let map_height = map2.iter().map(|x| x.pos.y).max().unwrap() + 1;

    print_map(&map2, &robot_pos, &map_width, &map_height);

    for m in moves {
        println!("move {}", m);
        if m == IVec2::new(0, 0) {
            continue;
        }

        // print!("{}[2J", 27 as char);

        // sleep(Duration::from_secs(1));

        let mut to_move: Vec<usize> = vec![];
        let mut move_for = IVec2::new(0, 0);

        // println!(
        //     "move {}",
        //     match m {
        //         IVec2::X => "right",
        //         IVec2::NEG_X => "left",
        //         IVec2::Y => "down",
        //         IVec2::NEG_Y => "up",
        //         _ => "",
        //     }
        // );
        // println!();
        // println!();

        let mut nextpos = robot_pos + m;
        let mut check_move_y: HashSet<usize> = HashSet::new();
        let mut x_limit_left = 0;
        let mut x_limit_right = 0;
        let mut can_move_y = true;

        if m == IVec2::Y || m == IVec2::NEG_Y {
            // find el directly above/below
            let mut tmp_pos = robot_pos + m;
            x_limit_left = tmp_pos.x;
            x_limit_right = tmp_pos.x;
            let mut elms_minmax: Vec<i32> = vec![tmp_pos.x];
            let mut wd = 100;
            loop {
                let mut has_match = false;
                let x_limit_left2 =
                    *elms_minmax.iter().min().unwrap();
                let x_limit_right2 =
                    *elms_minmax.iter().max().unwrap();
                // println!(
                //     "  logic updown {} {}",
                //     x_limit_left2, x_limit_right2
                // );

                elms_minmax = vec![];

                for above_below in
                    map2.iter().enumerate().filter(|f| {
                        f.1.pos.y == tmp_pos.y
                            && f.1.pos.x >= x_limit_left2
                            && f.1.pos.x <= x_limit_right2
                    })
                {
                    // println!("    found el");
                    if above_below.1.thing == Thing::Wall {
                        // println!("hit wall, don't move robot also");
                        can_move_y = false;
                        break;
                    }

                    if above_below.1.is_right {
                        elms_minmax.push(above_below.1.pos.x);
                        elms_minmax.push(above_below.1.pos.x - 1);
                        // also find left one
                        if let Some(xyxy) =
                            map2.iter().enumerate().find(|xyxy| {
                                xyxy.1.pos
                                    == IVec2::new(
                                        above_below.1.pos.x - 1,
                                        above_below.1.pos.y,
                                    )
                            })
                        {
                            check_move_y.insert(xyxy.0);
                        }
                    } else {
                        elms_minmax.push(above_below.1.pos.x);
                        elms_minmax.push(above_below.1.pos.x + 1);

                        if let Some(xyxy) =
                            map2.iter().enumerate().find(|xyxy| {
                                xyxy.1.pos
                                    == IVec2::new(
                                        above_below.1.pos.x + 1,
                                        above_below.1.pos.y,
                                    )
                            })
                        {
                            check_move_y.insert(xyxy.0);
                        }
                    }

                    has_match = true;
                    check_move_y.insert(above_below.0);
                }

                if !has_match {
                    break;
                }

                if can_move_y {
                    tmp_pos += m;
                }
                wd -= 1;
                if wd == 0 {
                    break;
                }
            }
        }

        loop {
            if m == IVec2::X || m == IVec2::NEG_X {
                if let Some(next) = map2
                    .iter()
                    .enumerate()
                    .find(|f| f.1.pos == nextpos)
                {
                    if next.1.thing == Thing::Wall {
                        break;
                    }
                    to_move.push(next.0);
                } else {
                    // next pos is empty space
                    move_for = m;
                    break;
                }
                nextpos += m;
            } else {
                // up down has different logic
                move_for = IVec2::new(0, 0);
                if can_move_y {
                    to_move.append(
                        &mut check_move_y
                            .iter()
                            .map(|x| *x)
                            .collect::<Vec<usize>>(),
                    );
                    move_for = m;
                }
                break;
            }
        }

        // println!("move idx {:?}", to_move);

        for me in to_move {
            map2[me].pos += move_for;
        }

        robot_pos += move_for;

        // print_map(&map2, &robot_pos, &map_width, &map_height);

        // println!();
        // println!();
    }

    println!();
    println!("finished");
    println!();
    print_map(&map2, &robot_pos, &map_width, &map_height);

    let res = map2
        .iter()
        .map(|me| {
            if me.thing == Thing::Box && me.is_left {
                return me.pos.y * 100 + me.pos.x;
            }
            0
        })
        .sum::<i32>();

    Ok(format!("{}", res))
}

#[cfg(test)]
mod tests {
    use crate::{solve_p1, solve_p2};

    #[test]
    fn p1_1() -> miette::Result<()> {
        let s = solve_p1(String::from(
            "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
        ));

        assert_eq!("10092", s?);
        Ok(())
    }

    #[test]
    fn p1_2() -> miette::Result<()> {
        let s = solve_p1(String::from(
            "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<",
        ));

        assert_eq!("2028", s?);
        Ok(())
    }

    #[test]
    fn p2_1() -> miette::Result<()> {
        let s = solve_p2(String::from(
            "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
asdf",
        ));

        assert_eq!("9021", s?);
        Ok(())
    }

    #[test]
    fn p2_2() -> miette::Result<()> {
        let s = solve_p2(String::from(
            "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^",
        ));

        assert_eq!("9021", s?);
        Ok(())
    }

    #[test]
    fn p2_3() -> miette::Result<()> {
        let s = solve_p2(String::from(
            "########
#......#
#......#
#.@O...#
#..O...#
#...O..#
#......#
#......#
########

>>^>v",
        ));

        assert_eq!("9021", s?);
        Ok(())
    }
}
