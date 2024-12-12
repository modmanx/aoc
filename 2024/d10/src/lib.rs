use std::collections::{HashMap, HashSet};
use std::fmt;

use itertools::Itertools;
use petgraph::algo::{all_simple_paths, astar, dijkstra};
use petgraph::data::DataMap;
use petgraph::dot::{Config, Dot};
use petgraph::graph::{DiGraph, NodeIndex, UnGraph};
use petgraph::visit::{EdgeRef, Visitable};
use petgraph::{Graph, Undirected};

// fn parse(i: &str) -> miette::Result<Graph<&TopoNode, i32>> {

// }

#[derive(Debug, Default)]
struct TopoNode {
    id: String,
    x: i32,
    y: i32,
    height: i32,
}

pub fn solve(contents: String) -> miette::Result<String> {
    let mut g: Graph<TopoNode, i32> = Graph::new();

    contents.lines().enumerate().for_each(|(line_idx, line)| {
        line.chars()
            .enumerate()
            .filter(|chr| chr.1 != '.')
            .for_each(|(char_idx, chr)| {
                g.add_node(TopoNode {
                    id: format!("{char_idx}_{line_idx}"),
                    x: char_idx as i32,
                    y: line_idx as i32,
                    height: chr.to_digit(10).unwrap() as i32,
                });
            })
    });

    let dirs = [(0, 1), (1, 0), (0, 1), (-1, 0)];

    let mut to_add: Vec<(usize, usize, i32)> = vec![];

    for node_to_check_idx in g.node_indices() {
        let node_to_check = g.node_weight(node_to_check_idx).unwrap();
        for d in dirs {
            let adj_node_idx_find = g.node_indices().find(|ni| {
                if let Some(node_myb) = g.node_weight(*ni) {
                    if node_to_check.x + d.0 == node_myb.x && node_to_check.y + d.1 == node_myb.y {
                        return true;
                    }
                }
                false
            });
            if let Some(adj_node_idx) = adj_node_idx_find {
                let adj_node = g.node_weight(adj_node_idx).unwrap();
                match adj_node.height - node_to_check.height {
                    -1 => {
                        if !to_add.contains(&(adj_node_idx.index(), node_to_check_idx.index(), 1))
                            && !to_add.contains(&(
                                node_to_check_idx.index(),
                                adj_node_idx.index(),
                                1,
                            ))
                        {
                            to_add.push((adj_node_idx.index(), node_to_check_idx.index(), 1));
                        }
                    }
                    1 => {
                        if !to_add.contains(&(adj_node_idx.index(), node_to_check_idx.index(), 1))
                            && !to_add.contains(&(
                                node_to_check_idx.index(),
                                adj_node_idx.index(),
                                1,
                            ))
                        {
                            to_add.push((node_to_check_idx.index(), adj_node_idx.index(), 1));
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    for ta in to_add {
        g.add_edge(NodeIndex::new(ta.0), NodeIndex::new(ta.1), ta.2);
    }

    let total_paths: i32 = g
        .node_indices()
        .filter(|x| {
            if let Some(xx) = g.node_weight(*x) {
                return xx.height == 0;
            }
            false
        })
        .map(|zero_idx| {
            let nines = g.node_indices().filter(|x| {
                if let Some(xx) = g.node_weight(*x) {
                    return xx.height == 9;
                }
                false
            });
            let paths_found: i32 = nines
                .into_iter()
                .map(|nine_idx| {
                    let path = astar(
                        &g,
                        zero_idx,
                        |finish| finish == nine_idx,
                        |e| *e.weight(),
                        |_| 0,
                    );
                    if path.is_some() {
                        return 1;
                    }
                    0
                })
                .sum();
            paths_found
        })
        .sum();

    Ok(format!("{}", total_paths))
}

pub fn solve_p2(contents: String) -> miette::Result<String> {
    let mut g: DiGraph<TopoNode, i32> = Graph::new();

    contents.lines().enumerate().for_each(|(line_idx, line)| {
        line.chars()
            .enumerate()
            .filter(|chr| chr.1 != '.')
            .for_each(|(char_idx, chr)| {
                g.add_node(TopoNode {
                    id: format!("{char_idx}_{line_idx}"),
                    x: char_idx as i32,
                    y: line_idx as i32,
                    height: chr.to_digit(10).unwrap() as i32,
                });
            })
    });

    let dirs = [(0, 1), (1, 0), (0, 1), (-1, 0)];

    let mut to_add: Vec<(usize, usize, i32)> = vec![];

    for node_to_check_idx in g.node_indices() {
        let node_to_check = g.node_weight(node_to_check_idx).unwrap();
        for d in dirs {
            let adj_node_idx_find = g.node_indices().find(|ni| {
                if let Some(node_myb) = g.node_weight(*ni) {
                    if node_to_check.x + d.0 == node_myb.x && node_to_check.y + d.1 == node_myb.y {
                        return true;
                    }
                }
                false
            });
            if let Some(adj_node_idx) = adj_node_idx_find {
                let adj_node = g.node_weight(adj_node_idx).unwrap();
                match adj_node.height - node_to_check.height {
                    -1 => {
                        if !to_add.contains(&(adj_node_idx.index(), node_to_check_idx.index(), 1))
                            && !to_add.contains(&(
                                node_to_check_idx.index(),
                                adj_node_idx.index(),
                                1,
                            ))
                        {
                            to_add.push((adj_node_idx.index(), node_to_check_idx.index(), 1));
                        }
                    }
                    1 => {
                        if !to_add.contains(&(adj_node_idx.index(), node_to_check_idx.index(), 1))
                            && !to_add.contains(&(
                                node_to_check_idx.index(),
                                adj_node_idx.index(),
                                1,
                            ))
                        {
                            to_add.push((node_to_check_idx.index(), adj_node_idx.index(), 1));
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    for ta in to_add {
        g.add_edge(NodeIndex::new(ta.0), NodeIndex::new(ta.1), ta.2);
    }

    let total_paths: usize = g
        .node_indices()
        .filter(|x| {
            if let Some(xx) = g.node_weight(*x) {
                return xx.height == 0;
            }
            false
        })
        .map(|zero_idx| {
            let nines = g.node_indices().filter(|x| {
                if let Some(xx) = g.node_weight(*x) {
                    return xx.height == 9;
                }
                false
            });
            nines
                .into_iter()
                .map(|nine_idx| {
                    let xyxyxy: Vec<Vec<_>> =
                        all_simple_paths(&g, zero_idx, nine_idx, 0, None).collect();
                    xyxyxy.len()
                })
                .sum::<usize>()
        })
        .sum();

    Ok(format!("{}", total_paths))
}

#[cfg(test)]
mod tests {
    use crate::{solve, solve_p2};
    // use miette::miette;

    #[test]
    fn p1_score_2() -> miette::Result<()> {
        let s = solve(String::from(
            "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9",
        ));

        assert_eq!("2", s?);
        Ok(())
    }

    #[test]
    fn p1_score_4() -> miette::Result<()> {
        let s = solve(String::from(
            "..90..9
...1.98
...2..7
6543456
765.987
876....
987....",
        ));

        assert_eq!("4", s?);
        Ok(())
    }

    #[test]
    /// This topographic map contains two trailheads; the trailhead at the top has a score of 1, while the trailhead at the bottom has a score of 2
    fn p1_score_1_2() -> miette::Result<()> {
        let s = solve(String::from(
            "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01",
        ));

        assert_eq!("3", s?);
        Ok(())
    }

    #[test]
    /// This topographic map contains two trailheads; the trailhead at the top has a score of 1, while the trailhead at the bottom has a score of 2
    fn p1_score_1_36() -> miette::Result<()> {
        let s = solve(String::from(
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        ));

        assert_eq!("36", s?);
        Ok(())
    }

    #[test]
    /// This topographic map contains two trailheads; the trailhead at the top has a score of 1, while the trailhead at the bottom has a score of 2
    fn p2_score_3() -> miette::Result<()> {
        let s = solve_p2(String::from(
            ".....0.
..4321.
..5..2.
..6543.
..7..4.
..8765.
..9....",
        ));

        assert_eq!("3", s?);
        Ok(())
    }

    #[test]
    /// This topographic map contains two trailheads; the trailhead at the top has a score of 1, while the trailhead at the bottom has a score of 2
    fn p2_score_13() -> miette::Result<()> {
        let s = solve_p2(String::from(
            "..90..9
...1.98
...2..7
6543456
765.987
876....
987....",
        ));

        assert_eq!("13", s?);
        Ok(())
    }

    #[test]
    /// This topographic map contains two trailheads; the trailhead at the top has a score of 1, while the trailhead at the bottom has a score of 2
    fn p2_score_227() -> miette::Result<()> {
        let s = solve_p2(String::from(
            "012345
123456
234567
345678
4.6789
56789.",
        ));

        assert_eq!("227", s?);
        Ok(())
    }
}
