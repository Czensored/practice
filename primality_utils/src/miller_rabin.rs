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

/// Determines if a given `BigUint` is probably prime using the Miller-Rabin test.
///
/// This implementation includes a fast check for small known primes and performs
/// `k` rounds of the Miller-Rabin primality test on larger inputs.
///
/// # Arguments
///
/// * `n` - A reference to a `BigUint` number to be tested for primality.
/// * `k` - The number of witness rounds to run in the Miller-Rabin test. More rounds
///         increase the accuracy of the test.
///
/// # Returns
///
/// * `true` if the number is probably prime (with a high probability depending on `k`)
/// * `false` if the number is definitely composite
///
/// # Notes
///
/// - For very large `n`, increase `k` (e.g., 20–40) for stronger confidence.
/// - This function is probabilistic and may yield false positives for pseudoprimes,
///   but is very reliable in practice.
///
/// # Examples
///
/// ```
/// use num_bigint::BigUint;
/// use my_primes::miller_rabin;
///
/// let prime = BigUint::parse_bytes(b"104729", 10).unwrap(); // 10000th prime
/// assert!(miller_rabin(&prime, 10));
///
/// let composite = BigUint::from(10000u32);
/// assert!(!miller_rabin(&composite, 10));
/// ```
pub fn miller_rabin(n: &BigUint, k: u32) -> bool {
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
