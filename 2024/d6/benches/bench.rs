use d6::{solve, solve_loop};
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
    _ = solve_loop(std::fs::read_to_string(String::from("in.txt")).unwrap());
}
