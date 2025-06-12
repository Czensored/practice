// Ref: https://www.reddit.com/r/dailyprogrammer/comments/nzmvsj/20210614_challenge_394_difficult_rsa_encryption/
// This algorithm is not actually cryptographically secure, I did not use secure rng

use num_bigint::{BigInt, BigUint, RandBigInt, ToBigInt};
use num_integer::Integer;
use num_traits::{One, Zero};
use rand::thread_rng;
use std::mem;

fn fermat_prime(n: &BigUint) -> bool {
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

fn generate_prime() -> BigUint {
    let mut rng = thread_rng();

    let small_primes: Vec<BigUint> = vec![2u32, 3, 5, 7, 11]
        .into_iter()
        .map(BigUint::from)
        .collect();

    let lower = BigUint::one() << 256;
    let upper = BigUint::one() << 512;

    loop {
        let candidate = rng.gen_biguint_range(&lower, &upper);
        // let candidate = rng.gen_biguint(256);

        if small_primes
            .iter()
            .any(|p| &candidate % p == BigUint::zero())
        {
            continue;
        }

        // Simple probabilistic primality test
        if fermat_prime(&candidate) {
            return candidate;
        }
    }
}

fn lcm(a: &BigUint, b: &BigUint) -> BigUint {
    if a.is_zero() || b.is_zero() {
        return BigUint::zero();
    }
    // normally have to take abs(a * b), but they are uints, so no worries
    (a * b) / a.gcd(b)
}

fn extended_gcd(a: &BigUint, b: &BigUint) -> (BigUint, BigInt, BigInt) {
    let mut r0 = a.to_bigint().unwrap();
    let mut r1 = b.to_bigint().unwrap();
    let mut s0 = BigInt::one();
    let mut s1 = BigInt::zero();
    let mut t0 = BigInt::zero();
    let mut t1 = BigInt::one();

    while !r1.is_zero() {
        let q = &r0 / &r1;

        let r_temp = &r0 - &q * &r1;
        r0 = mem::replace(&mut r1, r_temp);

        let s_temp = &s0 - &q * &s1;
        s0 = mem::replace(&mut s1, s_temp);

        let t_temp = &t0 - &q * &t1;
        t0 = mem::replace(&mut t1, t_temp);
    }

    let gcd = r0.to_biguint().unwrap();

    (gcd, s0, t0)
}

fn modular_inverse(a: &BigUint, b: &BigUint) -> Option<BigUint> {
    let (gcd, inv, _) = extended_gcd(a, b);

    if gcd != BigUint::one() {
        return None;
    }

    let b_bigint = b.to_bigint().unwrap();
    let pos_inv = ((inv % &b_bigint) + &b_bigint) % &b_bigint;

    pos_inv.to_biguint()
}

fn run_rsa() -> (BigUint, BigUint, BigUint) {
    loop {
        let p = generate_prime();
        let q = generate_prime();
        if p == q {
            eprintln!("Same p and q was chosen, picking again.");
            continue;
        }
        let n = &p * &q;
        let p_minus_1 = &p - BigUint::one();
        let q_minus_1 = &q - BigUint::one();
        let c_tot = lcm(&p_minus_1, &q_minus_1);
        let e = BigUint::from(65537u32);

        if let Some(d) = modular_inverse(&e, &c_tot) {
            println!("Public key n:\n{}", n);
            println!("Public key e:\n{}", e);
            println!("Private key d:\n{}", d);
            return (n, e, d);
        } else {
            // I've never seen this trigger, exists just in case fermat's fails
            eprintln!("Modular inverse failed; retrying with new primes.");
        }
    }
}

fn encrypt_rsa(m: &BigUint, n: &BigUint, e: &BigUint) -> BigUint {
    if m >= n {
        panic!("Message too large for encryption");
    }
    m.modpow(e, n)
}

fn decrypt_rsa(c: &BigUint, n: &BigUint, d: &BigUint) -> BigUint {
    c.modpow(d, n)
}

fn main() {
    let (n, e, d) = run_rsa();
    println!();

    let message = "43110";
    println!("Message:\n{}", &message);
    let m = BigUint::parse_bytes(message.as_bytes(), 10).unwrap();

    let c = encrypt_rsa(&m, &n, &e);
    println!("Encrypted ciphertext:\n{}", &c);

    let m_new = decrypt_rsa(&c, &n, &d);
    println!("Decrypted message:\n{}", &m_new);
}
