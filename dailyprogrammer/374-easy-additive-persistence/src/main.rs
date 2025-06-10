// Ref: https://www.reddit.com/r/dailyprogrammer/comments/akv6z4/20190128_challenge_374_easy_additive_persistence/

fn additive_persistence(mut n: u128) -> u128 {
    let mut counter = 0;
    while n > 9 {
        let mut sum = 0;
        while n > 0 {
            sum += n % 10;
            n /= 10;
        }
        n = sum;
        counter += 1;
    }

    counter
}

fn main() {
    println!("{}", additive_persistence(13)); // 1
    println!("{}", additive_persistence(1234)); // 2
    println!("{}", additive_persistence(9876)); // 2
    println!("{}", additive_persistence(199)); // 3
}
