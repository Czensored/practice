// Ref: https://www.reddit.com/r/dailyprogrammer/comments/b0nuoh/20190313_challenge_376_intermediate_the_revised/

#[allow(dead_code)]
fn leaps_naive(b: u128, e: u128) -> u128 {
    assert!(b <= e);
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

fn leaps_helper(start: u128, end: u128, k: u128) -> u128 {
    let difference = end - start;
    let mut sum = difference / k;
    if difference % k >= end % k {
        sum += 1;
    }
    sum
}

fn leaps(mut start: u128, mut end: u128) -> u128 {
    assert!(start <= end);
    start -= 1;
    end -= 1;
    leaps_helper(start, end, 4)
        - leaps_helper(start, end, 100)
        + leaps_helper(start - 200, end - 200, 900)
        + leaps_helper(start - 600, end - 600, 900)
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
    println!("{}", leaps(123456789101112, 1314151617181920)); // 288412747246240
}
