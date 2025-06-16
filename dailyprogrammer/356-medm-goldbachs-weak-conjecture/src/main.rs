// Ref: https://www.reddit.com/r/dailyprogrammer/comments/8bh8dh/20180411_challenge_356_intermediate_goldbachs/

use primality_utils::MillerRabin;

fn goldbach_triple(n: usize) -> (usize, usize, usize) {
    assert!(n > 5, "Input must be greater than 5");
    assert!(n % 2 == 1, "Input must be odd");

    if 2usize.miller_rabin(10) {
        let candidate = n - 2 * 2;
        if candidate.miller_rabin(10) {
            return (candidate, 2, 2);
        }
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

#[allow(dead_code)]
fn verify_goldbach_up_to(max: usize, step: usize) {
    assert!(max > 5, "Max must be greater than 5");
    assert!(step % 2 == 0, "step should be an even number");

    for n in (7..=max).step_by(step) {
        goldbach_triple(n);
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
        println!("{:?}", goldbach_triple(i));
    }
    // verify_goldbach_up_to(100001, 10);
}
