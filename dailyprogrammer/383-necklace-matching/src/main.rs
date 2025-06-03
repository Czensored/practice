// Ref: https://www.reddit.com/r/dailyprogrammer/comments/ffxabb/20200309_challenge_383_easy_necklace_matching/

fn same_necklace(stri1: &str, stri2: &str) -> bool {
    let str1 = stri1.to_string();
    let mut str2 = stri2.to_string();
    if str1.len() != str2.len() {
        return false;
    }
    if str1.len() == 0 {
        return true;
    }

    for _ in 0..str1.len() {
        if str1 == str2 {
            return true;
        }
        let move_char = str2.chars().nth(0);
        str2.remove(0);
        str2.push(move_char.expect("Should have already returned if len is 0"));
    }
    false
}

fn repeats(str1: &str) -> usize {
    if str1.len() <= 1 {
        return 1;
    }
    let mut str2 = str1.to_string();
    let mut count = 0;

    for _ in 0..str1.len() {
        if str1 == str2 {
            count += 1;
        }
        let move_char = str2.chars().nth(0);
        str2.remove(0);
        str2.push(move_char.expect("Should have already returned if len is 0"));
    }
    count
}

fn main() {
    println!("Part 1:");
    println!("{}", same_necklace("nicole", "icolen")); // true
    println!("{}", same_necklace("nicole", "lenico")); // true
    println!("{}", same_necklace("nicole", "coneli")); // false
    println!("{}", same_necklace("aabaaaaabaab", "aabaabaabaaa")); // true
    println!("{}", same_necklace("abc", "cba")); // false
    println!("{}", same_necklace("xxyyy", "xxxyy")); // false
    println!("{}", same_necklace("xyxxz", "xxyxz")); // false
    println!("{}", same_necklace("x", "x")); // true
    println!("{}", same_necklace("x", "xx")); // false
    println!("{}", same_necklace("x", "")); // false
    println!("{}", same_necklace("", "")); // true
    println!("");
    println!("Part 2:");
    println!("{}", repeats("abc")); // 1
    println!("{}", repeats("abcabcabc")); // 3
    println!("{}", repeats("abcabcabcx")); // 1
    println!("{}", repeats("aaaaaa")); // 6
    println!("{}", repeats("a")); // 1
    println!("{}", repeats("")); // 1
}
