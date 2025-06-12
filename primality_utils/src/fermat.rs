use num_bigint::{BigUint, RandBigInt};
use num_traits::One;
use num_integer::Integer;
use rand::thread_rng;

pub fn fermat_primality_test(n: &BigUint) -> bool {
    let mut rng = thread_rng();

    let a = loop {
        let candidate = rng.gen_biguint_range(&BigUint::from(2u32), &(n - 2u32));
        if candidate.gcd(n) == BigUint::one() {
            break candidate;
        }
    };

    let n_minus_1 = n - BigUint::one();
    a.modpow(&n_minus_1, n) == BigUint::one()
}
