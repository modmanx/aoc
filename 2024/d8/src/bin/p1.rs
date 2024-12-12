use std::time::Instant;

use d8::solve;

fn main() {
    let n = Instant::now();
    let res = solve(std::fs::read_to_string("in.txt").unwrap());
    let needed = Instant::now() - n;
    println!("res: {:?}     ... needed {}", res, needed.as_secs_f32());
}
