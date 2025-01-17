use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use nom::{
    bytes::complete::tag, character::complete::alpha0,
    multi::separated_list0, IResult,
};

use miette::miette;

use pathfinding::prelude::count_paths;

fn parse(s: &str) -> IResult<&str, (Vec<String>, Vec<String>)> {
    let (s, r1) = separated_list0(tag(", "), alpha0)(s)?;
    let (s, _r2) = tag("\n\n")(s)?;
    let (s, r3) = separated_list0(tag("\n"), alpha0)(s)?;

    Ok((
        s,
        (
            r1.into_iter().map(String::from).collect(),
            r3.into_iter().map(String::from).collect(),
        ),
    ))
}

pub fn solve_p1(contents: String) -> miette::Result<String> {
    println!("start solve p1");

    let (_, (parts, words)) = parse(&contents[..])
        .map_err(|e| miette!("could not parse, err: {}", e))?;
    let cc = Arc::new(Mutex::new(0));

    words.into_iter().for_each(|current_word| {
        println!("working on word {}", current_word);
        let mut ok_parts = vec![];
        for p in parts.iter() {
            if current_word.contains(p) {
                ok_parts.push(p);
            }
        }
        println!("  ok parts: {:?}", ok_parts);

        let mut counters = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let mut current_word_depth_idx = 0;
        'xyxy: loop {
            let mut curr_try_word = String::from("");

            for ii in 0..=current_word_depth_idx {
                if counters[ii] >= ok_parts.len() {
                    break 'xyxy;
                }
                curr_try_word += ok_parts[counters[ii]];
            }

            println!("  trying {}", curr_try_word);

            if curr_try_word == current_word {
                *cc.lock().unwrap() += 1;
                // counters[current_word_depth_idx] += 1;
                break;
                // break;
            }

            if current_word.starts_with(curr_try_word.as_str()) {
                println!("    moving in");
                (current_word_depth_idx + 1..counters.len())
                    .for_each(|jj| {
                        counters[jj] = 0;
                    });
                current_word_depth_idx += 1;
            } else {
                println!("    skipping {}", curr_try_word);
                counters[current_word_depth_idx] += 1;
                if counters[current_word_depth_idx] >= ok_parts.len()
                {
                    if current_word_depth_idx == 0 {
                        println!(".... got to end");
                        break;
                    }
                    current_word_depth_idx -= 1;
                    counters[current_word_depth_idx] += 1;
                    (current_word_depth_idx + 1..counters.len())
                        .for_each(|jj| {
                            counters[jj] = 0;
                        });
                }
            }
        }
    });

    // for current_word in words.into_par_iter() {

    // }
    let ans = *cc.lock().unwrap();
    Ok(ans.to_string())
}

pub fn solve_p2(contents: String) -> miette::Result<String> {
    println!("start solve p2");

    let (_, (parts, words)) = parse(&contents[..])
        .map_err(|e| miette!("could not parse, err: {}", e))?;
    let cc = Arc::new(Mutex::new(0));

    words.into_iter().for_each(|current_word| {
        println!("working on word {}", current_word);
        let mut ok_parts = vec![];
        for p in parts.iter() {
            if current_word.contains(p) {
                ok_parts.push(p);
            }
        }
        println!("  ok parts: {:?}", ok_parts);

        let mut counters = vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let mut current_word_depth_idx = 0;
        'xyxy: loop {
            let mut curr_try_word = String::from("");

            for ii in 0..=current_word_depth_idx {
                if counters[ii] >= ok_parts.len() {
                    break 'xyxy;
                }
                curr_try_word += ok_parts[counters[ii]];
            }

            println!("  trying {}", curr_try_word);

            if curr_try_word == current_word {
                *cc.lock().unwrap() += 1;
                counters[current_word_depth_idx] += 1;

                if counters[current_word_depth_idx] >= ok_parts.len()
                {
                    if current_word_depth_idx == 0 {
                        println!(".... got to end");
                        break;
                    }
                    current_word_depth_idx -= 1;
                    counters[current_word_depth_idx] += 1;
                    (current_word_depth_idx + 1..counters.len())
                        .for_each(|jj| {
                            counters[jj] = 0;
                        });
                }

                continue;
            }

            if current_word.starts_with(curr_try_word.as_str()) {
                println!("    moving in");
                (current_word_depth_idx + 1..counters.len())
                    .for_each(|jj| {
                        counters[jj] = 0;
                    });
                current_word_depth_idx += 1;
            } else {
                println!("    skipping {}", curr_try_word);
                counters[current_word_depth_idx] += 1;
                if counters[current_word_depth_idx] >= ok_parts.len()
                {
                    if current_word_depth_idx == 0 {
                        println!(".... got to end");
                        break;
                    }
                    current_word_depth_idx -= 1;
                    counters[current_word_depth_idx] += 1;
                    (current_word_depth_idx + 1..counters.len())
                        .for_each(|jj| {
                            counters[jj] = 0;
                        });
                }
            }
        }
    });

    // for current_word in words.into_par_iter() {

    // }
    let ans = *cc.lock().unwrap();
    Ok(ans.to_string())
}

fn solve_rec(
    word: String,
    curr_root: String,
    // chr_idx: usize,
    ok_parts: Vec<String>,
    cc: Arc<Mutex<i32>>,
    // s2: &rayon::Scope<'_>,
    depth: i32,
) {
    // let depthstr = (0..depth).map(|x| "  ").join("");
    // println!("{}{}", depthstr, curr_root);
    if curr_root == word {
        // println!("found {}", curr_root);
        *cc.lock().unwrap() += 1;
    } else {
        ok_parts.iter().for_each(|p| {
            let new_word = curr_root.clone() + p;
            if word.starts_with(new_word.as_str()) {
                let word2 = word.clone();
                let ok_parts2 = ok_parts.clone();
                let cc2 = cc.clone();
                let depth2 = depth + 1;
                rayon::spawn(move || {
                    solve_rec(
                        word2, new_word, // next_chr_idx,
                        ok_parts2, cc2, depth2,
                    );
                });
            }
        });
    }
}

pub fn solve_p2_v2(contents: String) -> miette::Result<String> {
    println!("start solve p2");

    let (_, (parts, words)) = parse(&contents[..])
        .map_err(|e| miette!("could not parse, err: {}", e))?;
    let cc = Arc::new(Mutex::new(0));

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(10)
        .build()
        .unwrap();

    words.iter().for_each(|current_word| {
        println!("working on word {}", current_word);
        let ok_parts: Vec<String> = parts
            .iter()
            .filter(|x| current_word.contains(x.as_str()))
            .map(String::from)
            .collect();

        println!("  ok parts: {:?}", ok_parts);

        ok_parts.iter().for_each(|ok_part| {
            println!("starting new - {}", ok_part);
            let s = String::from(ok_part);
            let cword = current_word.clone();
            let ok2 = ok_parts.clone();
            let cc2 = cc.clone();
            pool.install(|| {
                solve_rec(
                    cword, s, ok2, cc2, // pool,
                    0,
                );
            });
        });

        println!("ok_parts all done ... wait all");
    });
    println!("scope ok");
    let res = *cc.lock().unwrap();
    Ok(res.to_string())
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct TreeEntry {
    from: String,
    root: String,
    is_ok_path: bool,
    is_final: bool,
    _d: usize,
}

fn solve_rec_v3(
    hm: &mut HashMap<String, TreeEntry>,
    current_root: String,
    final_word: String,
    ok_parts: &Vec<String>,
) {
    ok_parts.iter().for_each(|ok_part| {
        let new_root = current_root.clone() + ok_part;
        let new_key = current_root.clone() + "-" + &new_root;
        if hm.contains_key(new_key.as_str()) {
            return;
        }
        if !final_word.starts_with(new_root.as_str()) {
            hm.insert(
                new_key.clone(),
                TreeEntry {
                    from: current_root.clone(),
                    root: new_root.clone(),
                    is_ok_path: false,
                    is_final: true,
                    _d: 1,
                },
            );
        } else if new_root.as_str() == final_word.as_str() {
            hm.insert(
                new_key.clone(),
                TreeEntry {
                    from: current_root.clone(),
                    root: new_root.clone(),
                    is_ok_path: true,
                    is_final: new_root.as_str()
                        == final_word.as_str(),
                    _d: 2,
                },
            );
        } else {
            hm.insert(
                new_key.clone(),
                TreeEntry {
                    from: current_root.clone(),
                    root: new_root.clone(),
                    is_ok_path: true,
                    is_final: new_root.as_str()
                        == final_word.as_str(),
                    _d: 3,
                },
            );
            solve_rec_v3(
                hm,
                new_root.clone(),
                final_word.clone(),
                ok_parts,
            );
        }
    });
}

pub fn solve_p2_v3(contents: String) -> miette::Result<String> {
    println!("start solve p2");

    let (_, (parts, words)) = parse(&contents[..])
        .map_err(|e| miette!("could not parse, err: {}", e))?;
    let cc = Arc::new(Mutex::new(0));

    let mut total = 0;

    words.iter().for_each(|current_word| {
        let mut hm: HashMap<String, TreeEntry> = HashMap::default();

        // hm.insert(
        //     "".to_string(),
        //     TreeEntry {
        //         _root: "".to_string(),
        //         // children: vec![],
        //         _is_final: false,
        //         _is_ok_path: true,
        //     },
        // );

        println!("working on word {}", current_word);
        let ok_parts: Vec<String> = parts
            .iter()
            .filter(|x| current_word.contains(x.as_str()))
            .map(String::from)
            .collect();

        println!("  ok parts: {:?}", ok_parts);

        ok_parts.iter().for_each(|ok_part| {
            // println!("starting new - {}", ok_part);
            // start with root if word begins with part
            if !current_word.starts_with(ok_part.as_str()) {
                return;
            }

            hm.insert(
                ok_part.clone(),
                TreeEntry {
                    from: "".to_string(),
                    root: ok_part.clone(),
                    is_ok_path: true,
                    is_final: false,
                    _d: 0,
                },
            );

            // build all options
            solve_rec_v3(
                &mut hm,
                ok_part.to_string(),
                current_word.clone(),
                &ok_parts,
            );
        });

        let hmvals: Vec<TreeEntry> = hm.into_values().collect();

        let n = count_paths(
            &TreeEntry {
                from: "".to_string(),
                root: "".to_string(),
                is_ok_path: true,
                is_final: false,
                _d: 255,
            },
            |x| {
                // println!("find all successors for {:?}", x);
                let ok = hmvals
                    .iter()
                    .filter(|y| y.from == x.root && y.is_ok_path);
                // let ok2 = hmvals
                //     .iter()
                //     .filter(|y| y.from == x.root && y.is_ok_path);
                // println!("  {:?}", ok2.collect::<Vec<&TreeEntry>>());
                ok
            },
            |&c| {
                // println!("check if ok: {:?}", c);
                c.is_final && c.is_ok_path
            },
        );

        println!("solution: {:?}", n);

        total += n;

        // println!("{:#?}", hm);

        // println!(
        //     "{:#?}",
        //     hm.iter()
        //         .filter(|(_, y)| { y._is_ok_path })
        //         .map(|(_, y)| y)
        //         .collect::<Vec<&TreeEntry>>()
        // );

        // println!(
        //     "{:#?}",
        //     hm.iter()
        //         .filter(|(_, y)| { y._is_final && y._is_ok_path })
        //         .map(|(_, y)| y)
        //         .collect::<Vec<&TreeEntry>>()
        // );

        // println!("ok_parts all done ... wait all");
    });
    println!("scope ok");
    let res = total;
    Ok(res.to_string())
}

#[cfg(test)]
mod tests {
    use crate::{solve_p1, solve_p2, solve_p2_v2, solve_p2_v3};

    #[test]
    fn p1_1() -> miette::Result<()> {
        let res = solve_p1(String::from(
            "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb",
        ))?;

        assert_eq!(res, "6");

        Ok(())
    }

    #[test]
    fn p2_1() -> miette::Result<()> {
        let res = solve_p2(String::from(
            "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb",
        ))?;

        assert_eq!(res, "16");

        Ok(())
    }

    #[test]
    fn p2_2() -> miette::Result<()> {
        let res = solve_p2_v2(String::from(
            "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb",
        ))?;

        assert_eq!(res, "16");

        Ok(())
    }

    #[test]
    fn p2_3() -> miette::Result<()> {
        let res = solve_p2_v3(String::from(
            "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb",
        ))?;

        assert_eq!(res, "16");

        Ok(())
    }
}
