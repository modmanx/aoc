use std::collections::{HashMap, HashSet};

pub mod p1;

#[derive(Debug)]
struct FieldInfo {
    x: i32,
    y: i32,
    freq: char,
}

type FieldInfoHashMap = HashMap<(i32, i32), FieldInfo>;

fn parse(i: &str) -> miette::Result<(FieldInfoHashMap, i32, i32)> {
    let mut fields: HashMap<(i32, i32), FieldInfo> = HashMap::new();

    let mut yy = 0;
    let mut xx = 0;
    for l in i.lines() {
        xx = 0;
        for c in l.chars() {
            if c != '.' {
                fields.entry((xx, yy)).or_insert(FieldInfo {
                    x: xx,
                    y: yy,
                    freq: c,
                });
            }
            xx += 1;
        }
        yy += 1;
    }

    Ok((fields, xx, yy))
}

pub fn solve(contents: String) -> miette::Result<String> {
    let fields = parse(&contents)?;
    let field_w = fields.1;
    let field_h = fields.2;

    let mut ab_to_add: HashSet<(i32, i32)> = HashSet::default();

    for (fk, fv) in fields.0.iter() {
        // find other the same character
        fields
            .0
            .iter()
            .filter(|f1| f1.1.freq == fv.freq && f1.1.x != fv.x && f1.1.y != fv.y)
            .for_each(|f_other| {
                let other_diff_xy = (fv.x - f_other.1.x, fv.y - f_other.1.y);

                // insert antinodes for both
                let self_antinode_xy = (fk.0 + other_diff_xy.0, fk.1 + other_diff_xy.1);
                // let other_antinode_xy =
                //     (f_other.1.x - other_diff_xy.0, f_other.1.y - other_diff_xy.1);

                if self_antinode_xy.0 >= 0
                    && self_antinode_xy.0 < field_w
                    && self_antinode_xy.1 >= 0
                    && self_antinode_xy.1 < field_h
                {
                    ab_to_add.insert((self_antinode_xy.0, self_antinode_xy.1));
                }
                // if other_antinode_xy.0 >= 0
                //     && other_antinode_xy.0 < field_w
                //     && other_antinode_xy.1 >= 0
                //     && other_antinode_xy.1 < field_h
                // {
                //     ab_to_add.insert((other_antinode_xy.0, other_antinode_xy.1));
                // }
            });
    }

    Ok(format!("{}", ab_to_add.len()))
}

pub fn solve_p2(contents: String) -> miette::Result<String> {
    let fields = parse(&contents)?;
    let field_w = fields.1;
    let field_h = fields.2;

    // println!("field size: {}x{}", field_w, field_h);

    let mut ab_to_add: HashSet<(i32, i32)> = HashSet::default();

    for (fk, fv) in fields.0.iter() {
        // find other the same character

        fields
            .0
            .iter()
            .filter(|f1| f1.1.freq == fv.freq && f1.1.x != fv.x && f1.1.y != fv.y)
            .for_each(|f_other| {
                let other_diff_xy = (fv.x - f_other.1.x, fv.y - f_other.1.y);

                // insert antinodes for both
                let mut self_an_pos = (fk.0, fk.1);
                let mut other_an_pos = (f_other.1.x, f_other.1.y);

                ab_to_add.insert((self_an_pos.0, self_an_pos.1));
                ab_to_add.insert((other_an_pos.0, other_an_pos.1));

                loop {
                    self_an_pos.0 += other_diff_xy.0;
                    self_an_pos.1 += other_diff_xy.1;

                    if self_an_pos.0 < 0
                        || self_an_pos.1 < 0
                        || self_an_pos.0 >= field_w
                        || self_an_pos.1 >= field_h
                    {
                        break;
                    }

                    ab_to_add.insert((self_an_pos.0, self_an_pos.1));
                }

                loop {
                    other_an_pos.0 -= other_diff_xy.0;
                    other_an_pos.1 -= other_diff_xy.1;

                    if other_an_pos.0 < 0
                        || other_an_pos.1 < 0
                        || other_an_pos.0 >= field_w
                        || other_an_pos.1 >= field_h
                    {
                        break;
                    }

                    ab_to_add.insert((other_an_pos.0, other_an_pos.1));
                }
            });
    }

    Ok(format!("{}", ab_to_add.len()))
}
