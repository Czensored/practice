// Ref: https://www.reddit.com/r/dailyprogrammer/comments/cdieag/20190715_challenge_379_easy_progressive_taxation/

fn tax(n: usize) -> usize {
    let brackets = vec![
        (10_000, 0.0),
        (30_000, 0.1),
        (100_000, 0.25),
        (usize::MAX, 0.4),
    ];

    let mut total_tax = 0.0;
    let mut lower = 0;

    for &(upper, rate) in &brackets {
        if n <= lower {
            break;
        }

        let taxable = n.min(upper) - lower;
        total_tax += taxable as f64 * rate;
        lower = upper;
    }

    total_tax as usize
}

fn main() {
    println!("{}", tax(usize::MAX));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tax() {
        assert_eq!(tax(0), 0);
        assert_eq!(tax(10000), 0);
        assert_eq!(tax(10009), 0);
        assert_eq!(tax(10010), 1);
        assert_eq!(tax(12000), 200);
        assert_eq!(tax(56789), 8697);
        assert_eq!(tax(1234567), 473326);
    }
}
