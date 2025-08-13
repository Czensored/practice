// Ref: https://www.reddit.com/r/dailyprogrammer/comments/afxxca/20190114_challenge_372_easy_perfectly_balanced/

use std::collections::HashMap;

fn balanced(s: &str) -> bool {
    if s.len() % 2 != 0 {
        return false;
    }
    let num_xs = s.chars().filter(|&ch| ch == 'x').count();
    num_xs == s.len() / 2
}

fn balanced_bonus(s: &str) -> bool {
    if s.is_empty() {
        return true;
    }

    let mut counts = HashMap::new();
    for ch in s.chars() {
        *counts.entry(ch).or_insert(0) += 1;
    }

    let first_count = counts.values().next().unwrap();
    counts.values().all(|&count| count == *first_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_balanced() {
        assert_eq!(balanced("xxxyyy"), true);
        assert_eq!(balanced("yyyxxx"), true);
        assert_eq!(balanced("xxxyyyy"), false);
        assert_eq!(balanced("yyxyxxyxxyyyyxxxyxyx"), true);
        assert_eq!(balanced("xyxxxxyyyxyxxyxxyy"), false);
        assert_eq!(balanced(""), true);
        assert_eq!(balanced("x"), false);
    }

    #[test]
    fn test_balanced_bonus() {
        assert_eq!(balanced_bonus("xxxyyyzzz"), true);
        assert_eq!(balanced_bonus("abccbaabccba"), true);
        assert_eq!(balanced_bonus("xxxyyyzzzz"), false);
        assert_eq!(balanced_bonus("abcdefghijklmnopqrstuvwxyz"), true);
        assert_eq!(balanced_bonus("pqq"), false);
        assert_eq!(balanced_bonus("fdedfdeffeddefeeeefddf"), false);
        assert_eq!(balanced_bonus("www"), true);
        assert_eq!(balanced_bonus("x"), true);
        assert_eq!(balanced_bonus(""), true);
    }
}
