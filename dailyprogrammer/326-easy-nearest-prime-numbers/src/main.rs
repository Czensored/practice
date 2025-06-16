// Ref: https://www.reddit.com/r/dailyprogrammer/comments/6s70oh/2017087_challenge_326_easy_nearest_prime_numbers/
// Ref: https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test

use primality_utils::MillerRabin;
use num_bigint::BigUint;
use num_traits::One;

fn check_numbers(nums: &Vec<u128>) {
    for &n in nums {
        let n_big = BigUint::from(n);

        // if miller_rabin(&n_big, 10) {
        if n_big.miller_rabin(10) {
            println!("{} is prime.", n);
        } else if n == 1 {
            println!("1 < 2");
        } else {
            let lower = find_nearest_prime(&n_big, false);
            let upper = find_nearest_prime(&n_big, true);
            println!("{} < {} < {}", lower, n, upper);
        }
    }
}

fn find_nearest_prime(n: &BigUint, is_above: bool) -> BigUint {
    let one = BigUint::one();
    let mut candidate = if is_above { n + &one }  else { n - &one };

    loop {
        if candidate.miller_rabin(10) {
            return candidate;
        }
        if is_above {
            candidate += &one;
        } else {
            candidate -= &one;
        }
    }
}

fn main() {
    let nums = vec![270u128, 541, 993, 649, 2, 3, 42010741, 1425172824437700148];
    check_numbers(&nums);
}
