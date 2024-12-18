use std::time::Instant;

use d15::solve_p1;

fn main() {
    let n = Instant::now();
    let res = solve_p1(std::fs::read_to_string("in.txt").unwrap());
    let needed = Instant::now() - n;
    println!(
        "res: {:?}     ... needed {}",
        res,
        needed.as_secs_f32()
    );
}