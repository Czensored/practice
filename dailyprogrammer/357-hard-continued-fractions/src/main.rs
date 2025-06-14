// Ref: https://www.reddit.com/r/dailyprogrammer/comments/8dqzyi/20180420_challenge_357_hard_continued_fractions/
// ^ The above ref's given answers are incorrect, but the problem itself is interesting
// I also am not sure why this problem was marked as hard, it's fairly trivial

fn frac_to_gauss(mut n: u64, mut d: u64) -> Vec<u64> {
    assert!(d != 0, "Fractions should not have a denominator of 0.");
    let mut ret_vec = Vec::new();

    while d != 0 {
        let mult = n / d;
        ret_vec.push(mult);
        n -= mult * d;
        std::mem::swap(&mut n, &mut d);
    }
    ret_vec
}

fn gauss_to_frac(mut v: Vec<u64>) -> (u64, u64) {
    assert!(!v.is_empty(), "Input vector must not be empty.");
    let mut d = v.pop().unwrap();
    let mut n = 1;
    while !v.is_empty() {
        let mult = v.pop().unwrap();
        n = n + mult * d;
        std::mem::swap(&mut n, &mut d);
    }
    std::mem::swap(&mut n, &mut d);
    (n, d)
}

fn main() {
    println!("{:?}", frac_to_gauss(16, 45));
    println!("{:?}", frac_to_gauss(45, 16));
    println!("{:?}", frac_to_gauss(7, 3));
    println!("{:?}", gauss_to_frac(frac_to_gauss(30, 7)));
    println!("{:?}", gauss_to_frac(frac_to_gauss(22, 7)));
    println!("{:?}", gauss_to_frac(vec![0, 2, 1, 4, 3]));
}
