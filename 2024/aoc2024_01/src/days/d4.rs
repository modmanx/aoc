use std::{
    fs::File,
    io::{BufReader, Lines},
};

pub struct D4P1 {}

struct DiagWay {
    _name: String,
    move_x: i32,
    move_y: i32,
}

impl D4P1 {
    fn check_xmas(&self, l: &[String], x: usize, y: usize, w: usize, h: usize) -> usize {
        // println!("starting at {} {}", x, y);

        let diag_ways: [DiagWay; 8] = [
            DiagWay {
                _name: String::from("d desno dol"),
                move_x: 1,
                move_y: 1,
            },
            DiagWay {
                _name: String::from("d desno gor"),
                move_x: 1,
                move_y: -1,
            },
            DiagWay {
                _name: String::from("d levo dol"),
                move_x: -1,
                move_y: 1,
            },
            DiagWay {
                _name: String::from("d levo gor"),
                move_x: -1,
                move_y: -1,
            },
            DiagWay {
                _name: String::from("levo"),
                move_x: -1,
                move_y: 0,
            },
            DiagWay {
                _name: String::from("desno"),
                move_x: 1,
                move_y: 0,
            },
            DiagWay {
                _name: String::from("gor"),
                move_x: 0,
                move_y: -1,
            },
            DiagWay {
                _name: String::from("dol"),
                move_x: 0,
                move_y: 1,
            },
        ];

        let mut found_nb = 0;
        for dw in diag_ways {
            let mut curr_pos = [x as i32, y as i32];

            // println!("  checking {}  starting at {} {}", dw.name, x, y);

            for mn in 1..=3 {
                curr_pos[0] += dw.move_x;
                curr_pos[1] += dw.move_y;
                // check index is whitin
                if curr_pos[0] < 0 || curr_pos[1] < 0 {
                    // println!("    x y < 0");
                    break;
                }
                if curr_pos[0] >= w as i32 || curr_pos[1] >= h as i32 {
                    // println!("    x y > max");
                    break;
                }

                let ch = l[curr_pos[1] as usize]
                    .chars()
                    .nth(curr_pos[0] as usize)
                    .unwrap();
                // println!("    char: {}", ch);
                if (mn == 1 && ch != 'M') || (mn == 2 && ch != 'A') || (mn == 3 && ch != 'S') {
                    break;
                }

                if mn == 3 {
                    // println!("    found at {} {}   dir {}", x, y, dw.name);

                    found_nb += 1;
                }
            }
        }

        found_nb
    }

    pub fn solve(&self, lines: Lines<BufReader<File>>) {
        let lines_data: Vec<String> = lines.map(|x| x.unwrap()).collect();

        let h = lines_data.len();
        let w = lines_data[0].len();
        let mut totals: usize = 0;
        for y in 0..h {
            for x in 0..w {
                let chr = lines_data[y].chars().nth(x).unwrap();
                if chr == 'X' {
                    totals += self.check_xmas(&lines_data, x, y, w, h);
                }
            }
        }
        println!("totals: {}", totals)
    }
}

pub struct D4P2 {}

struct MustBeMas(i32, i32, char);

impl D4P2 {
    fn check_x_mas(&self, l: &[String], x: usize, y: usize, w: usize, h: usize) -> usize {
        if x < 1 || y < 1 || x > w - 2 || y > h - 2 {
            return 0_usize;
        }

        let mbm: [[MustBeMas; 4]; 4] = [
            [
                MustBeMas(-1, -1, 'M'),
                MustBeMas(1, 1, 'S'),
                MustBeMas(-1, 1, 'M'),
                MustBeMas(1, -1, 'S'),
            ],
            [
                MustBeMas(-1, -1, 'M'),
                MustBeMas(1, 1, 'S'),
                MustBeMas(1, -1, 'M'),
                MustBeMas(-1, 1, 'S'),
            ],
            [
                MustBeMas(1, -1, 'M'),
                MustBeMas(-1, 1, 'S'),
                MustBeMas(1, 1, 'M'),
                MustBeMas(-1, -1, 'S'),
            ],
            [
                MustBeMas(1, 1, 'M'),
                MustBeMas(-1, -1, 'S'),
                MustBeMas(-1, 1, 'M'),
                MustBeMas(1, -1, 'S'),
            ],
        ];

        let mut found_nb = 0;
        for dw in mbm {
            // println!("  checking {}  starting at {} {}", dw.name, x, y);
            let mut isok = true;
            for mmmm in dw {
                let ch = l[(y as i32 + mmmm.1) as usize]
                    .chars()
                    .nth((x as i32 + mmmm.0) as usize)
                    .unwrap();
                if ch != mmmm.2 {
                    isok = false;
                    break;
                }
            }
            if isok {
                found_nb += 1;
            }
        }

        found_nb
    }

    pub fn solve(&self, lines: Lines<BufReader<File>>) {
        let lines_data: Vec<String> = lines.map(|x| x.unwrap()).collect();

        let h = lines_data.len();
        let w = lines_data[0].len();
        let mut totals: usize = 0;
        for y in 0..h {
            for x in 0..w {
                let chr = lines_data[y].chars().nth(x).unwrap();
                if chr == 'A' {
                    totals += self.check_x_mas(&lines_data, x, y, w, h);
                }
            }
        }
        println!("totals: {}", totals)
    }
}
