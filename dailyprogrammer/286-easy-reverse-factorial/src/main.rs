// Ref: https://www.reddit.com/r/dailyprogrammer/comments/55nior/20161003_challenge_286_easy_reverse_factorial/

#[allow(dead_code)]
fn factorial(n: usize) -> usize {
    let mut fac = 1;

    for i in 2..=n {
        fac *= i;
    }

    fac
}

fn reverse_factorial(mut n: usize) -> Option<usize> {
    if n == 1 { return Some(1); }
    let mut mult = 2;
    while n > 1 {
        if n % mult != 0 { return None; }
        n /= mult;
        mult += 1;
    }
    Some(mult - 1)
}

fn main() {
    let test_vec = vec![120, 150, 3628800, 479001600, 6, 18];
    for test in test_vec {
        println!("reverse_factorial({}) = {:?}", test, reverse_factorial(test));
    }
}
