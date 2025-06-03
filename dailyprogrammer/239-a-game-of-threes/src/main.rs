// Ref: https://www.reddit.com/r/dailyprogrammer/comments/3r7wxz/20151102_challenge_239_easy_a_game_of_threes/

fn game_of_threes(mut n: usize) {
    while n > 1 {
        let adjustment = match n % 3 {
            0 => 0,
            1 => -1,
            _ => 1,
        };
        println!("{} {}", n, adjustment);
        n = (n as isize + adjustment) as usize / 3;
    }
    println!("{}", n);
}

fn main() {
    let n = 100;
    game_of_threes(n);
}
