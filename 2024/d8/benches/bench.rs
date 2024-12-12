use d8::{solve, solve_p2};
use divan::black_box;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]

fn p1() {
    _ = solve(std::fs::read_to_string(String::from("in.txt")).unwrap());
}

#[divan::bench]
fn p2() {
    _ = solve_p2(std::fs::read_to_string(String::from("in.txt")).unwrap());
}
