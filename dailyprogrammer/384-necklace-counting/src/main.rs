// Ref: https://www.reddit.com/r/dailyprogrammer/comments/g1xrun/20200415_challenge_384_intermediate_necklace/

fn unique_paired_factors(n: u128) -> Vec<(u128, u128)> {
    let mut pairs = Vec::new();
    let mut i = 1;

    while i * i <= n {
        if n % i == 0 {
            let a = i;
            let b = n / i;
            pairs.push((a, b));
        }
        i += 1;
    }

    pairs
}

fn euler_phi(mut n: u128) -> u128 {
    let mut result = n;
    let mut i = 2;

    while i * i <= n {
        if n % i == 0 {
            while n % i == 0 {
                n /= i;
            }
            result -= result / i;
        }
        i += 1;
    }

    if n > 1 {
        result -= result / n;
    }

    result
}

fn necklaces(k: u128, n: u128) -> u128 {
    // Still dont love the "as u32" in this function
    let mut sum = 0;
    let pairs = unique_paired_factors(n);
    for (a, b) in pairs {
        sum += euler_phi(a) * k.pow(b as u32);
        if a != b {
            sum += euler_phi(b) * k.pow(a as u32);
        }
    }
    sum / n
}

fn main() {
    // println!("{:?}", unique_paired_factors(36));
    println!("{}", necklaces(2, 12)); // 352
    println!("{}", necklaces(3, 7)); // 315
    println!("{}", necklaces(9, 4)); // 1665
    println!("{}", necklaces(21, 3)); // 3101
    println!("{}", necklaces(99, 2)); // 4950

    // Can't run this one as I'd have to use BigUInt, and I don't want to
    // println!("{}", necklaces(3, 90)); // 96977372978752360287715019917722911297222
    println!("{}", necklaces(123, 18)); // 2306850769218800390268044415272597042
    println!("{}", necklaces(1234567, 6)); // 590115108867910855092196771880677924
    println!("{}", necklaces(12345678910, 3)); // 627225458787209496560873442940
}
