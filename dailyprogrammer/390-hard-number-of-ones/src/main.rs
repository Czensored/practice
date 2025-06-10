// Ref: https://www.reddit.com/r/dailyprogrammer/comments/neg49j/20210517_challenge_390_difficult_number_of_1s/
// In Progress

fn how_many_ones_brute_force(n: usize) -> usize {
    let mut count = 0;
    for i in 1..=n {
        let mut x = i;
        while x > 0 {
            if x % 10 == 1 {
                count += 1;
            }
            x /= 10;
        }
    }
    count
}

fn how_many_ones(mut n: usize) -> usize {
    let mut sum = 0;
    let mut place = 1;

    while n > 0 {
        place *= 10;
    }

    return 0;
}

fn main() {
    let n = 111;
    println!("{}", how_many_ones_brute_force(n));
}
