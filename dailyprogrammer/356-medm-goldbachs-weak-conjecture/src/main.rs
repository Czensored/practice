// Ref: https://www.reddit.com/r/dailyprogrammer/comments/8bh8dh/20180411_challenge_356_intermediate_goldbachs/
// Ref: https://www.youtube.com/watch?v=x32Zq-XvID4

use primality_utils::MillerRabin;

#[allow(dead_code)]
fn goldbach_triple(n: usize) -> (usize, usize, usize) {
    assert!(n > 5, "Input must be greater than 5");
    assert!(n % 2 == 1, "Input must be odd");

    let candidate = n - 4;
    if candidate.miller_rabin(10) {
        return (candidate, 2, 2);
    }

    for i in (3..n / 3).step_by(2) {
        if i.miller_rabin(10) {
            let candidate = n - 2 * i;
            if candidate.miller_rabin(10) {
                return (candidate, i, i);
            }
        }
    }

    unreachable!("No valid Goldbach triple was found for n = {}", n);
}

fn strong_golbach(n: usize) -> (usize, usize) {
    assert!(n > 2, "Input must be greater than 2");
    assert!(n % 2 == 0, "Input must be even");

    // let candidate = n - 2;
    // if candidate.miller_rabin(10) {
    //     return (candidate, 2);
    // }
    if n == 4 {
        return (2, 2);
    }

    for i in (3..=n / 2).step_by(2) {
        if i.miller_rabin(10) {
            let candidate = n - i;
            if candidate.miller_rabin(10) {
                return (candidate, i);
            }
        }
    }

    unreachable!("No valid Goldbach double was found for n = {}", n);
}

fn strong_goldbach_triple(n: usize) -> (usize, usize, usize) {
    assert!(n > 5, "Input must be greater than 5");
    assert!(n % 2 == 1, "Input must be odd");

    let (a, b) = strong_golbach(n - 3);
    (a, b, 3)
}

#[allow(dead_code)]
fn verify_goldbach_up_to(max: usize, step: usize) {
    assert!(max > 5, "Max must be greater than 5");
    assert!(step % 2 == 0, "step should be an even number");

    for n in (7..=max).step_by(step) {
        strong_goldbach_triple(n);
    }

    println!("All odd numbers up to {} passed the goldbach_triple test.", max);
}

fn main() {
    let v = vec![
        11usize,
        35,
        111,
        17,
        199,
        287,
        53,
        1902090121,
        1280938102984091281,
    ];

    for &i in &v {
        println!("{:?}", strong_goldbach_triple(i));
    }
    // verify_goldbach_up_to(100001, 10);
}
