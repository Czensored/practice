// Ref: https://www.reddit.com/r/dailyprogrammer/comments/9cvo0f/20180904_challenge_367_easy_subfactorials_another/

use std::time::Instant;

fn num_derangements_recursive(n: usize) -> usize {
    let mut lookup = vec![0; n + 1];
    lookup[0] = 1;
    if n >= 1 {
        lookup[1] = 0;
    }
    num_derangements_recursive_helper(n, &mut lookup)
}

fn num_derangements_recursive_helper(n: usize, lookup: &mut Vec<usize>) -> usize {
    if lookup[n] != 0 || n == 0 || n == 1 {
        return lookup[n];
    }
    let result = (n - 1) * (
        num_derangements_recursive_helper(n - 1, lookup) +
        num_derangements_recursive_helper(n - 2, lookup)
    );
    lookup[n] = result;
    result
}

fn main() {
    let inputs = [6, 9, 14, 18, 21, 24, 27];

    for &n in &inputs {
        let start = Instant::now();
        let result = num_derangements_recursive(n);
        let duration = start.elapsed();
        println!("Recursive D({}) = {} ({:?})", n, result, duration);
    }
}
