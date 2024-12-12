use std::fmt;

#[derive(Debug)]
struct FB {
    id: i64,
}

fn parse(i: &str) -> miette::Result<Vec<Option<FB>>> {
    let mut storage = vec![];

    let mut file_id = 0;
    let mut is_file = true;
    for c in i.chars() {
        let cint = c.to_digit(10).unwrap();
        for _ in 0..cint {
            if is_file {
                storage.push(Some(FB { id: file_id }));
            } else {
                storage.push(None);
            }
        }
        if is_file {
            file_id += 1;
        }
        is_file = !is_file;
    }

    Ok(storage)
}

pub fn solve(contents: String) -> miette::Result<String> {
    let mut s = parse(&contents)?;

    for ii in 0..s.len() {
        if s[ii].is_some() {
            continue;
        }
        for jj in (ii..s.len()).rev() {
            if s[jj].is_none() {
                continue;
            }
            s.swap(jj, ii);

            break;
        }
    }

    let res: i64 = s
        .into_iter()
        .filter(|x| x.is_some())
        .enumerate()
        .map(|(idx, x)| idx as i64 * x.unwrap().id)
        .sum();

    Ok(format!("{}", res))
}

struct FbSpace {
    id: Option<i64>,
    len: i64,
}

impl fmt::Debug for FbSpace {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ret = String::new();
        let chr_to_repeat = if let Some(xx) = self.id {
            xx.to_string()
        } else {
            String::from(".")
        };
        for _ in 0..self.len {
            ret += &chr_to_repeat;
        }
        write!(f, "{}", ret)
    }
}

fn parse2(i: &str) -> miette::Result<Vec<FbSpace>> {
    let mut storage = vec![];

    let mut file_id = 0;
    let mut is_file = true;
    for c in i.chars() {
        let cint = c.to_digit(10).unwrap();
        storage.push(FbSpace {
            id: if is_file { Some(file_id) } else { None },
            len: cint as i64,
        });
        if is_file {
            file_id += 1;
        }
        is_file = !is_file;
    }

    Ok(storage)
}

pub fn solve_p2(contents: String) -> miette::Result<String> {
    let mut p = parse2(&contents)?;

    let mut current_el_idx = p.len() - 1;

    loop {
        if p[current_el_idx].id.is_none() {
            current_el_idx -= 1;
            continue;
        }

        for myb_idx in 0..current_el_idx {
            if p[myb_idx].id.is_some() {
                continue;
            }
            if p[myb_idx].len >= p[current_el_idx].len {
                let diff_len = p[myb_idx].len - p[current_el_idx].len;

                p[myb_idx].len = p[current_el_idx].len;
                p[myb_idx].id = p[current_el_idx].id;
                p[current_el_idx].id = None;

                p.insert(
                    myb_idx + 1,
                    FbSpace {
                        id: None,
                        len: diff_len,
                    },
                );

                break;
            }
        }

        current_el_idx -= 1;
        if current_el_idx == 0 {
            break;
        }
    }

    let mut checksum = 0i64;

    let mut curr_idx = 0;
    for x in p.into_iter().filter(|xx| xx.len > 0) {
        for _ in 0..x.len {
            if let Some(id) = x.id {
                checksum += curr_idx * id;
            }
            curr_idx += 1;
        }
    }

    Ok(checksum.to_string())
}

#[cfg(test)]
mod tests {
    use crate::{solve, solve_p2};
    // use miette::miette;

    #[test]
    fn p1() -> miette::Result<()> {
        let s = solve(String::from("2333133121414131402"));

        assert_eq!("1928", s?);
        Ok(())
    }

    #[test]
    fn p2() -> miette::Result<()> {
        let s = solve_p2(String::from("2333133121414131402"));

        assert_eq!("2858", s?);
        Ok(())
    }
}
