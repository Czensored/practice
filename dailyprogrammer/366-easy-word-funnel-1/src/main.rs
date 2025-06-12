// Ref: https://www.reddit.com/r/dailyprogrammer/comments/98ufvz/20180820_challenge_366_easy_word_funnel_1/

use std::collections::HashSet;

fn funnel(s1: &str, s2: &str) -> bool {
    for ind in 0..s1.len() {
        if *s2 == format!("{}{}", &s1[..ind], &s1[ind + 1..]) {
            return true;
        }
    }
    false
}

fn load_word_list() -> HashSet<String> {
    let url = "https://raw.githubusercontent.com/dolph/dictionary/master/enable1.txt";
    let response = reqwest::blocking::get(url)
        .expect("Failed to fetch word list")
        .text()
        .expect("Failed to read response body");

    response.lines().map(|line| line.to_string()).collect()
}

fn funnel_children(word: &str, word_list: &HashSet<String>) -> HashSet<String> {
    let mut children = HashSet::new();
    for i in 0..word.len() {
        let candidate = format!("{}{}", &word[..i], &word[i + 1..]);
        if word_list.contains(&candidate) {
            children.insert(candidate);
        }
    }
    children
}

fn bonus(s: &str, word_list: &HashSet<String>) -> Vec<String> {
    funnel_children(s, word_list).into_iter().collect()
}

fn bonus_2(word_list: &HashSet<String>) -> Vec<String> {
    let mut ret_vec = Vec::new();
    for word in word_list {
        let children = funnel_children(word, word_list);
        if children.len() == 5 {
            ret_vec.push(word.clone());
        }
    }
    ret_vec
}

fn main() {
    println!("{}", funnel("leave", "eave")); // true
    println!("{}", funnel("reset", "rest")); // true
    println!("{}", funnel("dragoon", "dragon")); // true
    println!("{}", funnel("eave", "leave")); // false
    println!("{}", funnel("sleet", "lets")); // false
    println!("{}", funnel("skiff", "ski")); // false

    let word_list = load_word_list();
    println!("{:?}", bonus("dragoon", &word_list)); // ["dragon"]
    println!("{:?}", bonus("boats", &word_list)); // ["oats", "bats", "bots", "boas", "boat"]
    println!("{:?}", bonus("affidavit", &word_list)); // []

    let ans = bonus_2(&word_list);
    println!("len: {}, {:?}", ans.len(), ans);
}
