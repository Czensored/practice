// Ref: https://www.reddit.com/r/dailyprogrammer/comments/jfcuz5/20201021_challenge_386_intermediate_partition/
// Ref: https://www.youtube.com/watch?v=iJ8pnCO0nTY

use num_bigint::BigUint;
use num_traits::{One, Zero};

fn p(n: usize) -> BigUint {
    let mut partitions = vec![BigUint::one()]; // p(0) = 1

    for i in 1..=n {
        let mut total = BigUint::zero();
        let mut k = 1;

        loop {
            let g1 = k * (3 * k - 1) / 2;
            let g2 = k * (3 * k + 1) / 2;
            if g1 > i && g2 > i {
                break;
            }

            let sign = if k % 2 == 0 { -1 } else { 1 };

            if g1 <= i {
                let t = &partitions[i - g1];
                total = if sign > 0 { total + t } else { total - t };
            }
            if g2 <= i {
                let t = &partitions[i - g2];
                total = if sign > 0 { total + t } else { total - t };
            }

            k += 1;
        }

        partitions.push(total);
    }

    partitions[n].clone()
}

fn digit_sum(n: BigUint) -> u32 {
    n.to_str_radix(10).chars().map(|c| c.to_digit(10).unwrap()).sum()
}

fn main() {
    let n = 66;
    let result = p(n);

    println!("p({}) = {}", n, result);
    println!("Sum of digits = {}", digit_sum(result));
}
