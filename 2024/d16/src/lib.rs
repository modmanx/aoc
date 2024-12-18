use std::collections::{HashMap, HashSet};

use glam::IVec2;
use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::satisfy,
    multi::{many1, separated_list1},
    IResult,
};
use nom_locate::{position, LocatedSpan};
use pathfinding::prelude::{astar, astar_bag, astar_bag_collect};
use petgraph::{
    algo::astar as petgraphastar, dot::Dot, graph::NodeIndex, Graph,
};

type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, PartialEq, Eq)]
enum MapEntry {
    Path(IVec2),
    Start(IVec2),
    Finish(IVec2),
}

fn parse_map(i: Span) -> IResult<Span, Option<MapEntry>> {
    let (i, pos) = position(i)?;
    let x = pos.get_column() as i32 - 1;
    let y = pos.location_line() as i32 - 1;
    let (i, c) = satisfy(|c| c != '\n')(i)?;
    Ok((
        i,
        match c {
            '.' => Some(MapEntry::Path(IVec2::new(x, y))),
            'S' => Some(MapEntry::Start(IVec2::new(x, y))),
            'E' => Some(MapEntry::Finish(IVec2::new(x, y))),
            _ => None,
        },
    ))
}

fn parse_input(i: Span) -> IResult<Span, Vec<MapEntry>> {
    let r = separated_list1(tag("\n"), many1(parse_map))(i)?;
    Ok((r.0, r.1.into_iter().flatten().flatten().collect())) // second flatten removes None options
}

// fn print_map(m: &[MapEntry], robot_pos: &IVec2, w: &i32, h: &i32) {
//     for yy in 0..*h {
//         for xx in 0..*w {
//         }
//     }
// }

fn print_map_p2(m: &[MapEntry], places: &[IVec2], w: &i32, h: &i32) {
    for yy in 0..*h {
        for xx in 0..*w {
            if places.contains(&IVec2::new(xx, yy)) {
                print!("O");
            } else {
                let me = m.iter().find(|x| match x {
                    MapEntry::Path(ivec2) => {
                        *ivec2 == IVec2::new(xx, yy)
                    }
                    MapEntry::Start(ivec2) => {
                        *ivec2 == IVec2::new(xx, yy)
                    }
                    MapEntry::Finish(ivec2) => {
                        *ivec2 == IVec2::new(xx, yy)
                    }
                });
                if me.is_some() {
                    match me.unwrap() {
                        MapEntry::Path(ivec2) => print!("."),
                        MapEntry::Start(ivec2) => print!("."),
                        MapEntry::Finish(ivec2) => print!("."),
                    }
                } else {
                    print!(" ");
                }
            }
        }
        println!();
    }
}

const DIRS: [IVec2; 4] =
    [IVec2::Y, IVec2::NEG_Y, IVec2::X, IVec2::NEG_X];

fn build_graph_iter(
    map: &[IVec2],
    init_pos: IVec2,
    res: &mut HashSet<(IVec2, IVec2, u32)>,
    diags: &mut HashSet<(IVec2, IVec2, IVec2)>,
) {
    // find all crosses

    let cross_opt = [
        [IVec2::X, IVec2::NEG_Y],
        [IVec2::NEG_Y, IVec2::NEG_X],
        [IVec2::NEG_X, IVec2::Y],
        [IVec2::Y, IVec2::X],
    ];

    let mut ignore_pos: Vec<IVec2> = vec![];

    ignore_pos.push(init_pos);

    for d in DIRS {
        let init_d = init_pos + d;
        if map.contains(&init_d) {
            res.insert((
                init_pos,
                init_d,
                if d == IVec2::X { 1 } else { 1001 },
            ));
        }
    }

    for me in map.iter() {
        let mut is_cross = false;
        for co in cross_opt {
            if map.contains(&(me + co[0]))
                && map.contains(&(me + co[1]))
            {
                is_cross = true;

                // connect both outer to me

                res.insert((me + co[0], *me, 1));
                res.insert((me + co[1], *me, 1));

                // connect outer ones between with 1000€€€
                res.insert((me + co[0], me + co[1], 1002));
                res.insert((me + co[1], me + co[0], 1002));

                diags.insert((me + co[0], *me, me + co[1]));
                diags.insert((me + co[1], *me, me + co[0]));
            }
        }
        if is_cross {
            ignore_pos.push(*me);
            // also check if it has
            //     |
            //   --+---
            //     |
            //  to connect |s  and -s  skipping +

            let updownleftrights =
                [[IVec2::NEG_X, IVec2::X], [IVec2::NEG_Y, IVec2::Y]];

            for udlr in updownleftrights {
                if map.contains(&(me + udlr[0]))
                    && map.contains(&(me + udlr[1]))
                {
                    res.insert((me + udlr[0], me + udlr[1], 2));
                    res.insert((me + udlr[1], me + udlr[0], 2));
                }
            }

            ignore_pos.push(*me);
        }
    }

    // go through rest and make conns as usual

    for me in map.iter() {
        if ignore_pos.contains(me) {
            continue;
        }
        for d in DIRS {
            let me_d_pos = me + d;
            if ignore_pos.contains(&me_d_pos) {
                continue;
            }
            if !map.contains(&me_d_pos) {
                continue;
            }
            res.insert((*me, me_d_pos, 1));
        }
    }
}

pub fn solve_p1(contents: String) -> miette::Result<String> {
    println!("start solve p1");
    let contents_span = Span::new(&contents[..]);
    let (_, map) = parse_input(contents_span)
        .map_err(|e| miette!("could not parse, err: {}", e))?;

    let start = match map
        .iter()
        .find(|x| matches!(x, MapEntry::Start(_)))
        .unwrap()
    {
        MapEntry::Path(ivec2) => *ivec2,
        MapEntry::Start(ivec2) => *ivec2,
        MapEntry::Finish(ivec2) => *ivec2,
    };

    let finish = match map
        .iter()
        .find(|x| matches!(x, MapEntry::Finish(_)))
        .unwrap()
    {
        MapEntry::Path(ivec2) => *ivec2,
        MapEntry::Start(ivec2) => *ivec2,
        MapEntry::Finish(ivec2) => *ivec2,
    };

    let map2: Vec<IVec2> = map
        .iter()
        .map(|x| match x {
            MapEntry::Path(ivec2) => *ivec2,
            MapEntry::Start(ivec2) => *ivec2,
            MapEntry::Finish(ivec2) => *ivec2,
        })
        .collect::<Vec<IVec2>>();

    let mut res = HashSet::new();
    let mut diags = HashSet::new();

    println!("build graph");

    build_graph_iter(&map2, start, &mut res, &mut diags);

    // dbg!(&res);

    let mut graph = Graph::<IVec2, i32>::new();
    let mut ivec2_to_node: HashMap<IVec2, usize> = HashMap::new();
    let mut node_to_ivec2: HashMap<usize, IVec2> = HashMap::new();
    for mapnode in map2 {
        if res
            .iter()
            .filter(|(x1, x2, _)| *x1 == mapnode || *x2 == mapnode)
            .count()
            == 0
        {
            continue;
        }
        let ni = graph.add_node(mapnode);
        ivec2_to_node.insert(mapnode, ni.index());
        node_to_ivec2.insert(ni.index(), mapnode);
    }

    for current_node_idx in graph.node_indices() {
        let current_node_data =
            graph.node_weight(current_node_idx).unwrap();

        let connections: Vec<(IVec2, IVec2, u32)> = res
            .iter()
            .filter(|x| x.0 == *current_node_data)
            .map(|x| (x.0, x.1, x.2))
            .collect();

        for c in connections {
            // find node with this index
            if let Some(ni) = ivec2_to_node.get(&c.1) {
                graph.add_edge(
                    current_node_idx,
                    NodeIndex::new(*ni),
                    c.2 as i32,
                );
            }
        }
    }

    println!("{:?}", Dot::with_config(&graph, &[]));

    let start_node_idx =
        *ivec2_to_node.iter().find(|x| x.0 == &start).unwrap().1;

    let finish_node_idx =
        *ivec2_to_node.iter().find(|x| x.0 == &finish).unwrap().1;

    let path = petgraphastar(
        &graph,
        NodeIndex::new(start_node_idx),
        |finish| finish == NodeIndex::new(finish_node_idx),
        |e| *e.weight(),
        |_ne| 0,
    )
    .unwrap();

    // let mut curr_pos = start;
    // let mut score = 0i32;
    // println!("path: {}", path.0);
    // println!("len: {}", path.1.len());
    // for node in path.1 {
    //     let xy = node_to_ivec2.get(&node.index()).unwrap();

    //     let this_dir = xy - curr_pos;
    //     if curr_pos - xy == IVec2::new(0, 0) {
    //         continue;
    //     }
    //     if curr_pos == start && this_dir != IVec2::X {
    //         score += 1001
    //     } else if this_dir.x.abs() > 0 && this_dir.y.abs() > 0 {
    //         score += 1002;
    //     } else {
    //         score += this_dir.x.abs() + this_dir.y.abs();
    //     }
    //     println!(
    //         "{} -> {}   {}   score {}",
    //         curr_pos,
    //         xy,
    //         curr_pos - xy,
    //         score
    //     );

    //     curr_pos = *xy;
    // }

    // println!("calc score: {}", score);
    // println!("score astar: {}", path.0);

    let GOAL: (i32, i32) = (finish.x, finish.y);
    let result = astar_bag_collect(
        &(start.x, start.y),
        |&(x, y)| {
            res.iter()
                .filter(|(src, _, _)| src.x == x && src.y == y)
                .map(|(_, dest, price)| {
                    ((dest.x, dest.y), *price as i32)
                })
                .collect::<Vec<((i32, i32), i32)>>()
        },
        |&(_, _)| 0,
        |&p| p == GOAL,
    );

    println!("result astar2");
    let mut hs: HashSet<IVec2> = HashSet::new();
    let mut hs_diag: HashSet<(IVec2, IVec2)> = HashSet::new();
    if let Some(resok) = result {
        // let mut ii = 1;
        // println!("solutions found: {}", resok.0.len())
        println!("score: {}", resok.1);
        for sol in resok.0.iter() {
            let mut prev_pos = sol[0];
            let mut prev_pos_ivec =
                IVec2::new(prev_pos.0, prev_pos.1);
            let mut is_first = true;
            for sol_xy in sol {
                if is_first {
                    is_first = false;
                    continue;
                }

                let this_pos_ivec = IVec2::new(sol_xy.0, sol_xy.1);

                let diff = prev_pos_ivec - this_pos_ivec;

                if diff.x.abs() == 2 || diff.y.abs() == 2 {
                    let middle_pos = this_pos_ivec + diff / 2;
                    hs.insert(this_pos_ivec);
                    hs.insert(middle_pos);
                    hs.insert(prev_pos_ivec);
                    println!(
                        "{:?} -> {:?} -> {:?}   diff: {:?}   jump 2",
                        this_pos_ivec,
                        middle_pos,
                        prev_pos_ivec,
                        diff
                    );
                } else if diff.x.abs() == 1 && diff.y.abs() == 1 {
                    let middle_pos = this_pos_ivec + IVec2::new(0, 0);

                    let conn =
                        diags.iter().find(|(xy1, xymid, xy2)| {
                            xy1 == &this_pos_ivec
                                && xy2 == &prev_pos_ivec
                                || xy2 == &this_pos_ivec
                                    && xy1 == &prev_pos_ivec
                        });

                    hs.insert(this_pos_ivec);
                    hs.insert(conn.unwrap().1);
                    hs.insert(prev_pos_ivec);

                    println!(
                        "{:?} -> {:?} -> {:?}  diff: {:?}   jump diag",
                        this_pos_ivec, conn.unwrap().1, prev_pos_ivec, diff
                    );
                } else {
                    println!(
                        "{:?} -> {:?}   diff: {:?}",
                        this_pos_ivec, prev_pos_ivec, diff
                    );
                    hs.insert(this_pos_ivec);
                    hs.insert(prev_pos_ivec);
                }

                prev_pos_ivec = this_pos_ivec;
                // hs.insert((sol_xy.0, sol_xy.1));
            }

            println!();
            println!();
        }
        println!("unique fields: {}", hs.len() + hs_diag.len());
        println!("{:?}", hs);
    }
    let hs_vec = &hs.iter().copied().collect::<Vec<IVec2>>();
    print_map_p2(&map, hs_vec, &20, &20);

    Ok(format!("{}", path.0))
}

pub fn solve_p2(contents: String) -> miette::Result<String> {
    Ok(format!("{}", 1))
}

#[cfg(test)]
mod tests {
    use crate::{solve_p1, solve_p2};

    #[test]
    fn p1_1() -> miette::Result<()> {
        let s = solve_p1(String::from(
            "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############",
        ));

        assert_eq!("7036", s?);
        Ok(())
    }
    #[test]
    fn p1_2() -> miette::Result<()> {
        let s = solve_p1(String::from(
            "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################",
        ));

        assert_eq!("11048", s?);
        Ok(())
    }

    #[test]
    fn p1_3() -> miette::Result<()> {
        let s = solve_p1(String::from(
            "#####
#..E#
#.#.#
#...#
#.###
#S..#
#####",
        ));

        assert_eq!("11048", s?);
        Ok(())
    }
    #[test]
    fn p1_4() -> miette::Result<()> {
        let s = solve_p1(String::from(
            "#######
#.....#
#.#.#.#
#.....#
#.#.#.#
#S..#E#
#######",
        ));

        assert_eq!("11048", s?);
        Ok(())
    }
}
