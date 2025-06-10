// Ref: https://www.reddit.com/r/dailyprogrammer/comments/99d24u/20180822_challenge_366_intermediate_word_funnel_2/
// Ref: https://raw.githubusercontent.com/dolph/dictionary/master/enable1.txt
// ^ the above file is 1.7mb, so I didn't want to download it

mod memo_impl;

use std::collections::HashSet;
use std::time::Instant;

fn load_word_list() -> HashSet<String> {
    let url = "https://raw.githubusercontent.com/dolph/dictionary/master/enable1.txt";
    let response = reqwest::blocking::get(url)
        .expect("Failed to fetch word list")
        .text()
        .expect("Failed to read response body");

    response.lines().map(|line| line.to_string()).collect()
}

fn funnel2_helper(s: &String, word_list: &HashSet<String>) -> usize {
    let mut largest = 0;
    for ind in 0..s.len() {
        let s_mod = format!("{}{}", &s[..ind], &s[ind + 1..]);
        if word_list.contains(&s_mod) {
            let depth = 1 + funnel2_helper(&s_mod, word_list);
            largest = largest.max(depth);
        }
    }
    largest
}

fn funnel2(s: &str, word_list: &HashSet<String>) -> usize {
    assert!(s.is_ascii(), "Input must be ASCII");
    1 + funnel2_helper(&s.to_string(), &word_list)
}

fn run(word_list: &HashSet<String>) {
    for word in [
        "gnash",
        "princesses",
        "turntables",
        "implosive",
        "programmer",
    ] {
        let result = funnel2(word, &word_list);
        println!("{} → {}", word, result);
    }

    // Optional bonus 1:
    for i in word_list {
        if funnel2(&i, word_list) == 10 {
            println!("length 10 word is: {}", i);
        }
    }
}

fn main() {
    let word_list = load_word_list();
    let start = Instant::now();
    run(&word_list);
    let duration = start.elapsed();
    println!("run() took: {:.2?}\n", duration);

    let start = Instant::now();
    memo_impl::run_with_memoization(&word_list);
    let duration = start.elapsed();
    println!("run_with_memoization() took: {:.2?}", duration);
}
