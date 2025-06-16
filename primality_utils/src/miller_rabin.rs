use lazy_static::lazy_static;
use num_bigint::{BigUint, RandBigInt};
use num_traits::{FromPrimitive, One, Zero};
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
    let zero = BigUint::zero();
    let one = BigUint::one();
    let two = &one + &one;

    if n < &two {
        return false;
    }

    let mut rng = thread_rng();

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

fn miller_rabin_big(n: &BigUint, k: u32) -> bool {
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

pub trait MillerRabin {
    /// Trait for testing probable primality using the Miller-Rabin test.
    ///
    /// This provides a unified `.miller_rabin(k)` method for both `BigUint` and `usize`
    /// values, internally delegating to a fast-checked probabilistic primality test.
    ///
    /// # Arguments
    ///
    /// * `k` - The number of witness rounds to run in the Miller-Rabin test. More rounds
    ///         increase the accuracy of the test.
    ///
    /// # Returns
    ///
    /// * `true` if the number is probably prime (with high confidence)
    /// * `false` if the number is definitely composite
    ///
    /// # Notes
    ///
    /// - For large numbers, use a higher `k` (e.g., 20–40) for strong confidence.
    /// - The test is probabilistic and may yield false positives for pseudoprimes,
    ///   though it is highly reliable in practice.
    ///
    /// # Examples
    ///
    /// ```
    /// use num_bigint::BigUint;
    /// use my_primes::MillerRabin;
    ///
    /// let prime = BigUint::parse_bytes(b"104729", 10).unwrap(); // 10000th prime
    /// assert!(prime.miller_rabin(10));
    ///
    /// let composite = BigUint::from(10000u32);
    /// assert!(!composite.miller_rabin(10));
    ///
    /// assert!(997usize.miller_rabin(10)); // Works for usize too
    /// ```
    fn miller_rabin(&self, k: u32) -> bool;
}

impl MillerRabin for BigUint {
    fn miller_rabin(&self, k: u32) -> bool {
        miller_rabin_big(self, k)
    }
}

impl MillerRabin for usize {
    fn miller_rabin(&self, k: u32) -> bool {
        let n = BigUint::from_usize(*self).expect("Failed to convert usize to BigUint");
        miller_rabin_big(&n, k)
    }
}
