use num_bigint::{BigUint, RandBigInt};
use num_traits::One;
use num_integer::Integer;
use rand::thread_rng;

fn fermat_test(n: &BigUint) -> bool {
    let mut rng = thread_rng();

    let a = loop {
        let candidate = rng.gen_biguint_range(&BigUint::from(2u32), &(n - 2u32));
        if candidate.gcd(n) == BigUint::one() {
            break candidate;
        }
    };


    a.modpow(&(n - BigUint::one()), n) == BigUint::one()
}

pub trait FermatPrimalityTest {
    fn fermat_primality_test(&self) -> bool;
}

impl FermatPrimalityTest for BigUint {
    fn fermat_primality_test(&self) -> bool {
        fermat_test(&self)
    }
}
