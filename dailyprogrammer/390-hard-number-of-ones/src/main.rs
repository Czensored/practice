// Ref: https://www.reddit.com/r/dailyprogrammer/comments/neg49j/20210517_challenge_390_difficult_number_of_1s/

use num_bigint::BigUint;
use num_traits::{One, Zero};

#[allow(dead_code)]
fn how_many_ones_brute_force(n: BigUint) -> BigUint {
    let mut count = BigUint::zero();
    let one = BigUint::one();
    let ten = BigUint::from(10u32);

    let mut i = one.clone();
    while &i <= &n {
        let mut x = i.clone();
        while x > BigUint::zero() {
            if &x % &ten == one {
                count += &one;
            }
            x /= &ten;
        }
        i += &one;
    }

    count
}

fn how_many_ones(n: &BigUint) -> BigUint {
    let mut sum = BigUint::zero();
    let mut place = BigUint::one();
    let ten = BigUint::from(10u32);

    while &place <= n {
        let right = n % (&ten * &place);
        let left = n / (&ten * &place);

        // Emulate saturating_sub: max(right - (place - 1), 0)
        let subtrahend = &place - BigUint::one();
        let diff = if right > subtrahend {
            right.clone() - subtrahend
        } else {
            BigUint::zero()
        };

        let min_val = place.clone().min(diff);
        sum += left * &place + min_val;

        place *= &ten;
    }

    sum
}

fn main() {
    let base = BigUint::from(3u32);
    let n = base.pow(35);
    println!("{}", how_many_ones(&n));
}
