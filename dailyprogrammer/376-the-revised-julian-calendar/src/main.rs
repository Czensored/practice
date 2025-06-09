// Ref: https://www.reddit.com/r/dailyprogrammer/comments/b0nuoh/20190313_challenge_376_intermediate_the_revised/

fn leaps_naive(b: u128, e: u128) -> u128 {
    let mut sum = 0;
    for i in b..e {
        let n = i % 900;
        if n == 200 || n == 600 {
            sum += 1;
        } else if n % 100 != 0 && n % 4 == 0 {
            sum += 1;
        }
    }
    sum
}

fn leaps(b: u128, e: u128) -> u128 {
}

fn main() {
    println!("{}", leaps(2016, 2017)); // 1
    println!("{}", leaps(2019, 2020)); // 0
    println!("{}", leaps(1900, 1901)); // 0
    println!("{}", leaps(2000, 2001)); // 1
    println!("{}", leaps(2800, 2801)); // 0
    println!("{}", leaps(123456, 123456)); // 0
    println!("{}", leaps(1234, 5678)); // 1077
    println!("{}", leaps(123456, 7891011)); // 1881475
    // Challenge:
    // println!("{}", leaps(123456789101112, 1314151617181920)); // 288412747246240
}
