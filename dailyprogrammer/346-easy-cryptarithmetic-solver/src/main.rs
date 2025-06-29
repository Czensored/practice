// Ref: https://www.reddit.com/r/dailyprogrammer/comments/7p5p2o/20180108_challenge_346_easy_cryptarithmetic_solver/
// This one is rated easy, but it is not easy

fn solve_helper(lhs: Vec<&str>, rhs: &str) -> Vec<(char, usize)> {

}

fn solve(s: &str) -> Vec<(char, usize)> {
    let parts: Vec<&str> = s.split("==").collect();
    assert!(parts.len() == 2, "Input must contain '==' exactly once");

    let lhs_str = parts[0].trim();
    let rhs = parts[1].trim();

    let lhs: Vec<&str> = lhs_str
        .split('+')
        .map(|s| s.trim())
        .collect();

    solve_helper(lhs, rhs)
}

fn main() {
    let code = "THIS + IS + HIS == CLAIM";
    println!("{:?}", solve(code));
}
