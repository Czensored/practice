// Ref: https://www.reddit.com/r/dailyprogrammer/comments/6s70oh/2017087_challenge_326_easy_nearest_prime_numbers/
// Ref: https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test

use lazy_static::lazy_static;
use num_bigint::{BigUint, RandBigInt};
use num_traits::{One, Zero};
use rand::thread_rng;

lazy_static! {
    static ref SMALL_PRIMES: Vec<BigUint> = {
        let raw: [u64; 50] = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
            181, 191, 193, 197, 199, 211, 223, 227, 229,
        ];
        raw.iter().map(|&n| BigUint::from(n)).collect()
    };
}

fn miller_rabin_helper(n: &BigUint, k: u32) -> bool {
    let mut rng = thread_rng();

    let zero = BigUint::zero();
    let one = BigUint::one();
    let two = &one + &one;

    if n < &two {
        return false;
    }
    if n == &two || n == &BigUint::from(3u8) {
        return true;
    }
    if n % &two == zero {
        return false;
    }

    let mut d = n - &one;
    let mut s = 0;
    while &d % &two == zero {
        d >>= 1;
        s += 1;
    }

    'outer: for _ in 0..k {
        let a = rng.gen_biguint_range(&two, &(n - &two));
        let mut x = a.modpow(&d, n);

        if x == one || x == n - &one {
            continue;
        }

        for _ in 0..(s - 1) {
            x = x.modpow(&two, n);
            if x == n - &one {
                continue 'outer;
            }
        }

        return false;
    }

    true
}

fn miller_rabin(n: &BigUint, k: u32) -> bool {
    // Catch easy primes
    for p in SMALL_PRIMES.iter() {
        if n == p {
            return true;
        }
        if n % p == BigUint::zero() {
            return false;
        }
    }
    miller_rabin_helper(n, k)
}

fn check_numbers(nums: Vec<u128>) {
    for &n in &nums {
        let n_big = BigUint::from(n);

        if miller_rabin(&n_big, 10) {
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
        if miller_rabin(&candidate, 10) {
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
    check_numbers(nums);
}
