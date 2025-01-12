use std::time::Instant;

use d17::{
    solve_p1, solve_p2, solve_p2_blah, solve_p2_blah2, solve_p2_blah3,
};

fn main() {
    let n = Instant::now();
    let res =
        solve_p2_blah3(std::fs::read_to_string("in.txt").unwrap());
    let needed = Instant::now() - n;
    println!(
        "res: {:?}     ... needed {}",
        res,
        needed.as_secs_f32()
    );
}
