// Ref: https://www.reddit.com/r/dailyprogrammer/comments/jfcuz5/20201021_challenge_386_intermediate_partition/
// Ref: https://www.youtube.com/watch?v=iJ8pnCO0nTY

use num_bigint::BigUint;
use num_traits::{One, Zero};

fn p(n: usize) -> BigUint {
    let mut partitions = vec![BigUint::one()];

    for i in 1..=n {
        let mut sum = BigUint::zero();
        let mut k = 1;
    }

    partitions[n].clone()
}

fn main() {
    let n = 66;
    let result = p(n);
    println!("{}", result);
}

