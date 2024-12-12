use d6::solve_loop;

fn main() {
    let solve_res = solve_loop(std::fs::read_to_string("in.txt").unwrap());
    println!("res: {:?}", solve_res);
}
