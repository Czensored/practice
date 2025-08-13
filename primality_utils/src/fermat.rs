use num_bigint::{BigUint, RandBigInt};
use num_traits::One;
use num_integer::Integer;
use rand::thread_rng;

fn fermat_test_biguint(n: &BigUint) -> bool {
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
        fermat_test_biguint(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fermat_biguint() {
        // Known primes (usize)
        let primes = [104729u32, 99991, 65537];
        for &p in &primes {
            assert!(fermat_test_biguint(&BigUint::from(p)), "Failed on known BigUint prime: {}", p);
        }

        // Known composites (usize)
        let composites = [10000u32, 12345, 65535];
        for &c in &composites {
            assert!(
                !fermat_test_biguint(&BigUint::from(c)),
                "Incorrectly marked BigUint composite as prime: {}",
                c
            );
        }
    }
}
