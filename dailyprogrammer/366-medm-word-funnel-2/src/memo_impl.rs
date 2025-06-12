// Memoized version is generated via ChatGPT after inputting my code and asking for optimization
// It's faster to use the same memo for every word, but I think that that's an unfair comparison

use std::collections::{HashMap, HashSet};

fn funnel2_helper_memo(
    s: &String,
    word_list: &HashSet<String>,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if let Some(&cached) = memo.get(s) {
        return cached;
    }

    let mut largest = 0;
    for ind in 0..s.len() {
        let s_mod = format!("{}{}", &s[..ind], &s[ind + 1..]);
        if word_list.contains(&s_mod) {
            let depth = 1 + funnel2_helper_memo(&s_mod, word_list, memo);
            largest = largest.max(depth);
        }
    }

    memo.insert(s.clone(), largest);
    largest
}

pub fn funnel2_memo(
    s: &str,
    word_list: &HashSet<String>,
) -> usize {
    assert!(s.is_ascii(), "Input must be ASCII");
    let mut memo = HashMap::new();
    1 + funnel2_helper_memo(&s.to_string(), &word_list, &mut memo)
}

pub fn run_with_memoization(word_list: &HashSet<String>) {

    for word in [
        "gnash",
        "princesses",
        "turntables",
        "implosive",
        "programmer",
    ] {
        let result = funnel2_memo(word, &word_list);
        println!("{} → {}", word, result);
    }

    // Optional bonus 1:
    for i in word_list {
        if funnel2_memo(&i, word_list) == 10 {
            println!("length 10 word is: {}", i);
        }
    }
}
