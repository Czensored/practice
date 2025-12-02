use core::time;

use num_bigint::BigUint;
use num_traits::{One, Zero};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng, rng};
use rayon::prelude::*;

fn trial_exponent<R: Rng + ?Sized>(rng: &mut R) -> u32 {
    let bits: u64 = rng.random();
    bits.trailing_ones() + 1
}

pub fn run_trial<R: Rng + ?Sized>(rng: &mut R) -> BigUint {
    BigUint::one() << trial_exponent(rng)
}

pub fn rolling_avg(num_trials: u64) -> BigUint {
    let mut rng = SmallRng::from_os_rng();

    // count occurences of each exponent in an array
    // u64 + 1 spaces (additional +1 because 0 index not used)
    let mut counts = [0u64; 66];

    for _ in 0..num_trials {
        let e = trial_exponent(&mut rng) as usize;
        counts[e] += 1;
    }

    let mut sum = BigUint::zero();

    for (e, &c) in counts.iter().enumerate() {
        if c == 0 || e == 0 {
            continue;
        }

        // term = 2^e * count
        let term = (BigUint::one() << e) * BigUint::from(c);
        sum += term;
    }

    sum / num_trials
}

pub fn rolling_avg_old(num_trials: u64) -> BigUint {
    let mut rng = rng();
    let mut sum = BigUint::zero();
    for _ in 0..num_trials {
        sum += run_trial(&mut rng);
    }
    sum / num_trials
}

pub fn rolling_avg_paralell(num_trials: u64) -> BigUint {
    // ! TODO spawn fewer threads but still use multithreading
    // Have each thread handle more trials each
    let counts: [u64; 66] = (0..num_trials)
        .into_par_iter()
        .map_init(SmallRng::from_os_rng, |rng, _| trial_exponent(rng) as usize)
        .fold(
            || [0u64; 66],
            |mut local_counts, e| {
                local_counts[e] += 1;
                local_counts
            },
        )
        .reduce(
            || [0u64; 66],
            |mut a, b| {
                for i in 0..a.len() {
                    a[i] += b[i];
                }
                a
            },
        );

    let mut sum = BigUint::zero();

    for (e, &c) in counts.iter().enumerate() {
        if c == 0 || e == 0 {
            continue;
        }

        let term = (BigUint::one() << e) * BigUint::from(c);
        sum += term;
    }

    sum / num_trials
}
