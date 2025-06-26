// Ref: https://www.reddit.com/r/dailyprogrammer/comments/bqy1cf/20190520_challenge_378_easy_the_havelhakimi/
// Ref: https://en.wikipedia.org/wiki/Havel%E2%80%93Hakimi_algorithm

fn havel_hakimi(v: &Vec<usize>) -> bool {
    // This implementation sorts in ascending order to avoid having to remove the first index
    // and uses .pop() instead, which is faster
    let mut v = v.clone();

    loop {
        v.retain(|&x| x != 0);

        if v.is_empty() { return true; }

        v.sort_unstable();

        let n = v.pop().unwrap();

        if n > v.len() { return false; }

        for i in v.len() - n..v.len() { v[i] -= 1; }
    }
}

fn main() {
    println!("{}", havel_hakimi(&vec![5, 3, 0, 2, 6, 2, 0, 7, 2, 5])); // false
    println!("{}", havel_hakimi(&vec![4, 2, 0, 1, 5, 0])); // false
    println!("{}", havel_hakimi(&vec![3, 1, 2, 3, 1, 0])); // true
    println!("{}", havel_hakimi(&vec![16, 9, 9, 15, 9, 7, 9, 11, 17, 11, 4, 9, 12, 14, 14, 12, 17, 0, 3, 16])); // true
    println!("{}", havel_hakimi(&vec![14, 10, 17, 13, 4, 8, 6, 7, 13, 13, 17, 18, 8, 17, 2, 14, 6, 4, 7, 12])); // true
    println!("{}", havel_hakimi(&vec![15, 18, 6, 13, 12, 4, 4, 14, 1, 6, 18, 2, 6, 16, 0, 9, 10, 7, 12, 3])); // false
    println!("{}", havel_hakimi(&vec![6, 0, 10, 10, 10, 5, 8, 3, 0, 14, 16, 2, 13, 1, 2, 13, 6, 15, 5, 1])); // false
    println!("{}", havel_hakimi(&vec![2, 2, 0])); // false
    println!("{}", havel_hakimi(&vec![3, 2, 1])); // false
    println!("{}", havel_hakimi(&vec![1, 1])); // true
    println!("{}", havel_hakimi(&vec![1])); // false
    println!("{}", havel_hakimi(&vec![])); // true
}
