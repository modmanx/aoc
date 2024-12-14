use d13::{solve_p1, solve_p2};
use divan::black_box;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench(sample_size = 3, sample_count = 3)]
fn p1() {
    _ = solve_p1(std::fs::read_to_string(String::from("in.txt")).unwrap());
}

#[divan::bench(sample_size = 3, sample_count = 3)]
fn p2() {
    _ = solve_p2(std::fs::read_to_string(String::from("in.txt")).unwrap());
}
