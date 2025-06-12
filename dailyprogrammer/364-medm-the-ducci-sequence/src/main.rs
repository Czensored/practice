// Ref: https://www.reddit.com/r/dailyprogrammer/comments/8sjcl0/20180620_challenge_364_intermediate_the_ducci/

use std::collections::HashSet;

fn ducci(mut v: Vec<i64>) {
    let mut seen = HashSet::new();
    let len = v.len();
    let mut steps = 1;

    while !v.iter().all(|&x| x == 0) && seen.insert(v.clone()) {
        // println!("{:?}", v);
        v = (0..len)
            .map(|i| (v[i] - v[(i + 1) % len]).abs())
            .collect();
        steps += 1;
    }

    println!("{}", steps);
}

fn main() {
    ducci(vec![0, 653, 1854, 4063]);
    ducci(vec![1, 5, 7, 9, 9]);
    ducci(vec![1, 2, 1, 2, 1, 0]);
    ducci(vec![10, 12, 41, 62, 31, 50]);
    ducci(vec![10, 12, 41, 62, 31]);
}
