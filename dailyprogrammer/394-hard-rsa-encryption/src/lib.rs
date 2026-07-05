use num_bigint::{BigInt, BigUint, RandBigInt, ToBigInt};
use num_integer::Integer;
use num_traits::{One, Zero};
use primality_utils::MillerRabin;
use rand::{RngCore, thread_rng};
use std::fmt;
use std::mem;

#[derive(Debug, Clone)]
pub struct RsaKeyPair {
    pub p: BigUint,
    pub q: BigUint,
    pub n: BigUint,
    pub e: BigUint,
    pub d: BigUint,
}

#[derive(Debug, Clone)]
pub enum KeyFormatError {
    MissingModInverse(&'static str),
}

impl fmt::Display for KeyFormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingModInverse(name) => {
                write!(f, "failed to compute modular inverse for {name}")
            }
        }
    }
}

impl std::error::Error for KeyFormatError {}

fn generate_prime(bits: usize) -> BigUint {
    let mut rng = thread_rng();

    let small_primes: Vec<BigUint> = vec![2u32, 3, 5, 7, 11]
        .into_iter()
        .map(BigUint::from)
        .collect();

    let lower = BigUint::one() << (bits - 1);
    let upper = BigUint::one() << bits;

    loop {
        let mut candidate = rng.gen_biguint_range(&lower, &upper);
        if candidate.is_even() {
            candidate += BigUint::one();
        }

        if small_primes
            .iter()
            .any(|p| &candidate % p == BigUint::zero())
        {
            continue;
        }

        if candidate.miller_rabin(10) {
            return candidate;
        }
    }
}

fn lcm(a: &BigUint, b: &BigUint) -> BigUint {
    if a.is_zero() || b.is_zero() {
        return BigUint::zero();
    }

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

pub fn modular_inverse(a: &BigUint, b: &BigUint) -> Option<BigUint> {
    let (gcd, inv, _) = extended_gcd(a, b);

    if gcd != BigUint::one() {
        return None;
    }

    let b_bigint = b.to_bigint().unwrap();
    let pos_inv = ((inv % &b_bigint) + &b_bigint) % &b_bigint;

    pos_inv.to_biguint()
}

pub fn generate_keypair() -> RsaKeyPair {
    generate_keypair_with_prime_bits(1024)
}

pub fn generate_keypair_with_prime_bits(prime_bits: usize) -> RsaKeyPair {
    assert!(
        prime_bits >= 512,
        "RSA prime size must be at least 512 bits"
    );

    loop {
        let p = generate_prime(prime_bits);
        let q = generate_prime(prime_bits);
        if p == q {
            eprintln!("Same p and q was chosen, picking again.");
            continue;
        }

        let n = &p * &q;
        if n.bits() < (prime_bits * 2) as u64 {
            continue;
        }

        let p_minus_1 = &p - BigUint::one();
        let q_minus_1 = &q - BigUint::one();
        let c_tot = lcm(&p_minus_1, &q_minus_1);
        let e = BigUint::from(65537u32);

        if let Some(d) = modular_inverse(&e, &c_tot) {
            return RsaKeyPair { p, q, n, e, d };
        } else {
            eprintln!("Modular inverse failed; retrying with new primes.");
        }
    }
}

pub fn run_rsa() -> (BigUint, BigUint, BigUint) {
    let keypair = generate_keypair();
    println!("Public key n:\n{}", keypair.n);
    println!("Public key e:\n{}", keypair.e);
    println!("Private key d:\n{}", keypair.d);
    (keypair.n, keypair.e, keypair.d)
}

pub fn encrypt_rsa(m: &BigUint, n: &BigUint, e: &BigUint) -> BigUint {
    assert!(m < n, "Message too large for encryption");
    m.modpow(e, n)
}

pub fn decrypt_rsa(c: &BigUint, n: &BigUint, d: &BigUint) -> BigUint {
    c.modpow(d, n)
}

impl RsaKeyPair {
    pub fn to_pkcs1_private_pem(&self) -> Result<String, KeyFormatError> {
        let one = BigUint::one();
        let dp = &self.d % (&self.p - &one);
        let dq = &self.d % (&self.q - &one);
        let qi = modular_inverse(&self.q, &self.p)
            .ok_or(KeyFormatError::MissingModInverse("q inverse mod p"))?;

        let der = der_sequence(&[
            der_integer(&BigUint::zero()),
            der_integer(&self.n),
            der_integer(&self.e),
            der_integer(&self.d),
            der_integer(&self.p),
            der_integer(&self.q),
            der_integer(&dp),
            der_integer(&dq),
            der_integer(&qi),
        ]);

        Ok(pem_block("RSA PRIVATE KEY", &der))
    }

    pub fn to_pkcs1_public_pem(&self) -> String {
        let der = der_sequence(&[der_integer(&self.n), der_integer(&self.e)]);
        pem_block("RSA PUBLIC KEY", &der)
    }

    pub fn to_openssh_public_key(&self, comment: &str) -> String {
        let blob = self.to_openssh_public_blob();

        let encoded = base64_encode(&blob);
        if comment.is_empty() {
            format!("ssh-rsa {encoded}\n")
        } else {
            format!("ssh-rsa {encoded} {comment}\n")
        }
    }

    pub fn to_openssh_private_key(&self, comment: &str) -> Result<String, KeyFormatError> {
        let qi = modular_inverse(&self.q, &self.p)
            .ok_or(KeyFormatError::MissingModInverse("q inverse mod p"))?;

        let public_blob = self.to_openssh_public_blob();
        let mut rng = thread_rng();
        let checkint = rng.next_u32();

        let mut private_blob = Vec::new();
        private_blob.extend_from_slice(&checkint.to_be_bytes());
        private_blob.extend_from_slice(&checkint.to_be_bytes());
        ssh_write_string(&mut private_blob, b"ssh-rsa");
        ssh_write_mpint(&mut private_blob, &self.n);
        ssh_write_mpint(&mut private_blob, &self.e);
        ssh_write_mpint(&mut private_blob, &self.d);
        ssh_write_mpint(&mut private_blob, &qi);
        ssh_write_mpint(&mut private_blob, &self.p);
        ssh_write_mpint(&mut private_blob, &self.q);
        ssh_write_string(&mut private_blob, comment.as_bytes());

        let mut padding = 1u8;
        while private_blob.len() % 8 != 0 {
            private_blob.push(padding);
            padding = padding.wrapping_add(1);
        }

        let mut out = Vec::new();
        out.extend_from_slice(b"openssh-key-v1\0");
        ssh_write_string(&mut out, b"none");
        ssh_write_string(&mut out, b"none");
        ssh_write_string(&mut out, b"");
        out.extend_from_slice(&1u32.to_be_bytes());
        ssh_write_string(&mut out, &public_blob);
        ssh_write_string(&mut out, &private_blob);

        Ok(armored_block("OPENSSH PRIVATE KEY", &out, 70))
    }

    fn to_openssh_public_blob(&self) -> Vec<u8> {
        let mut blob = Vec::new();
        ssh_write_string(&mut blob, b"ssh-rsa");
        ssh_write_mpint(&mut blob, &self.e);
        ssh_write_mpint(&mut blob, &self.n);
        blob
    }
}

fn der_sequence(parts: &[Vec<u8>]) -> Vec<u8> {
    let content: Vec<u8> = parts.iter().flat_map(|part| part.iter().copied()).collect();
    der_tagged(0x30, &content)
}

fn der_integer(n: &BigUint) -> Vec<u8> {
    let mut bytes = n.to_bytes_be();
    if bytes.is_empty() {
        bytes.push(0);
    } else if bytes[0] & 0x80 != 0 {
        bytes.insert(0, 0);
    }

    der_tagged(0x02, &bytes)
}

fn der_tagged(tag: u8, content: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(1 + der_length(content.len()).len() + content.len());
    out.push(tag);
    out.extend(der_length(content.len()));
    out.extend_from_slice(content);
    out
}

fn der_length(len: usize) -> Vec<u8> {
    if len < 128 {
        return vec![len as u8];
    }

    let mut bytes = Vec::new();
    let mut value = len;
    while value > 0 {
        bytes.push((value & 0xff) as u8);
        value >>= 8;
    }
    bytes.reverse();

    let mut out = Vec::with_capacity(bytes.len() + 1);
    out.push(0x80 | bytes.len() as u8);
    out.extend(bytes);
    out
}

fn pem_block(label: &str, der: &[u8]) -> String {
    armored_block(label, der, 64)
}

fn armored_block(label: &str, bytes: &[u8], line_len: usize) -> String {
    let base64 = base64_encode(bytes);
    let mut out = String::new();
    out.push_str(&format!("-----BEGIN {label}-----\n"));

    for chunk in base64.as_bytes().chunks(line_len) {
        out.push_str(std::str::from_utf8(chunk).unwrap());
        out.push('\n');
    }

    out.push_str(&format!("-----END {label}-----\n"));
    out
}

fn ssh_write_string(out: &mut Vec<u8>, bytes: &[u8]) {
    out.extend_from_slice(&(bytes.len() as u32).to_be_bytes());
    out.extend_from_slice(bytes);
}

fn ssh_write_mpint(out: &mut Vec<u8>, n: &BigUint) {
    let mut bytes = n.to_bytes_be();
    if bytes.is_empty() {
        bytes.push(0);
    } else if bytes[0] & 0x80 != 0 {
        bytes.insert(0, 0);
    }
    ssh_write_string(out, &bytes);
}

fn base64_encode(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);

    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = *chunk.get(1).unwrap_or(&0);
        let b2 = *chunk.get(2).unwrap_or(&0);
        let triple = ((b0 as u32) << 16) | ((b1 as u32) << 8) | (b2 as u32);

        out.push(TABLE[((triple >> 18) & 0x3f) as usize] as char);
        out.push(TABLE[((triple >> 12) & 0x3f) as usize] as char);

        if chunk.len() > 1 {
            out.push(TABLE[((triple >> 6) & 0x3f) as usize] as char);
        } else {
            out.push('=');
        }

        if chunk.len() > 2 {
            out.push(TABLE[(triple & 0x3f) as usize] as char);
        } else {
            out.push('=');
        }
    }

    out
}
