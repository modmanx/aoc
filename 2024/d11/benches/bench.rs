use d11::{solve, solve_p2};
use divan::black_box;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn p1() {
    _ = solve(std::fs::read_to_string(String::from("in.txt")).unwrap(), 25);
}

#[divan::bench]
fn p2() {
    _ = solve(std::fs::read_to_string(String::from("in.txt")).unwrap(), 75);
}
