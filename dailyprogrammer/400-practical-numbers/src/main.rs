// Ref: https://www.reddit.com/r/dailyprogrammer/comments/13m4bz1/20230519_challenge_400_intermediate_practical/

// Here's a good naive approach by someone in the comments:
// https://github.com/cbarrick/practice/blob/master/reddit/dailyprogrammer/400-med/practical.rs
// Fun terrifying multithreaded solution by someone else in the comments:
// https://github.com/skeeto/scratch/blob/master/misc/practical.c

use prime_factorization::Factorization;
use std::time::Instant;

fn practical(n: u64) -> bool {
    fn sum_of_divisors(n: u64) -> u64 {
        let mut sum = 0;
        let sqrt_n = (n as f64).sqrt() as u64;

        for i in 1..=sqrt_n {
            if n % i == 0 {
                sum += i;
                if i != n / i {
                    sum += n / i;
                }
            }
        }

        sum
    }
    if n % 2 == 1 && n != 1 { return false; }

    let factor_repr = Factorization::<u64>::run(n);

    let mut last_checked = 2;

    for (index, &prime_fac) in factor_repr.factors.iter().enumerate() {
        if prime_fac == last_checked { continue; }

        let slice_prod = &factor_repr.factors[0..index].iter().product();
        let divisors_sum = sum_of_divisors(*slice_prod);

        // Formula
        if prime_fac > divisors_sum + 1 { return false; }

        last_checked = prime_fac;
    }

    true
}

#[allow(unused)]
fn calc_bonus_challenge(n: u32) {
    let num = 10u64.pow(n);
    let start = Instant::now();
    let mut sum = 0;
    for i in 1..=10000 {
        if practical(i + num) { sum += i; }
    }
    let duration = start.elapsed();
    println!("{} in {:?}", sum, duration);
}

fn main() {
    // let n: u64 = 429606;
    // println!("{}", practical(n));

    // Should probably compile with --release if this is being run
    let power = 19;
    calc_bonus_challenge(power);
}
