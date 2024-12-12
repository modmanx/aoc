#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use crate::{solve, solve_loop};

    #[test]
    fn it_works() {
        let s = solve(String::from(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        ));
        assert_eq!(s, Ok(41))
    }

    #[test]
    fn it_works_loop() {
        let s = solve_loop(String::from(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        ));
        assert_eq!(s, Ok(6))
    }
}
