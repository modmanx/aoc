use std::time::Instant;

use d18::solve_p2_v3;

fn main() {
    let n = Instant::now();
    let res = solve_p2_v3(std::fs::read_to_string("in.txt").unwrap());
    let needed = Instant::now() - n;
    println!(
        "res: {:?}     ... needed {}",
        res,
        needed.as_secs_f32()
    );
}
