use std::time::Instant;

use d11::solve;

fn main() {
    let n = Instant::now();
    let res = solve(std::fs::read_to_string("in.txt").unwrap(), 25);
    let needed = Instant::now() - n;
    println!("res: {:?}     ... needed {}", res, needed.as_secs_f32());
}
