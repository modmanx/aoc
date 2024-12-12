#![allow(dead_code)]

#[cfg(test)]
mod tests {
    use crate::{solve, solve_concat};

    #[test]
    fn it_works() {
        let s = solve(String::from(
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        ));
        assert_eq!(s, Ok(3749))
    }

    #[test]
    fn concat() {
        let s = solve_concat(String::from(
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        ));
        assert_eq!(s, Ok(11387))
    }

    #[test]
    fn concat2() {
        let s = solve_concat(String::from("111: 1 1 1"));
        assert_eq!(s, Ok(111))
    }
    #[test]
    fn concat_not_solvable() {
        let s = solve_concat(String::from("1111: 1 1 1 2"));
        assert_eq!(s, Ok(0))
    }
}
