// Ref: https://www.reddit.com/r/dailyprogrammer/comments/o9k0p0/20210628_challenge_395_intermediate_phone_drop/

fn binomial(x: usize, n: usize, k: usize) -> usize {
    let mut answer: usize = 0;
    let mut aux: usize = 1;

    for i in 1..=n {
        if i > x { break; } // prevent x - i + 1 underflow
        aux = aux.saturating_mul(x - i + 1); // avoid overflow
        aux /= i;
        answer = answer.saturating_add(aux); // avoid overflow

        if answer > k {
            return k + 1; // shortcut: only care if it's > k
        }
    }

    answer
}

fn phone_drop(n: usize, k: usize) -> usize {
    let mut left = 1;
    let mut right = k;

    while left < right {
        let mid = left + (right - left) / 2;
        if binomial(mid, n, k) >= k {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    left
}

fn optional_bonus() {
    let floors = 123456789;
    let mut ans = 1;
    while phone_drop(ans, floors) != 27 {
        ans += 1;
    }
    println!("The smallest n such that phone_drop(n, 123456789) = 27 is {}", ans);
}

fn main() {
    println!("{}", phone_drop(1, 100)); // 100
    println!("{}", phone_drop(2, 100)); // 14
    println!("{}", phone_drop(3, 100)); // 9
    println!("{}", phone_drop(1, 1)); // 1
    println!("{}", phone_drop(2, 456)); // 30
    println!("{}", phone_drop(3, 456)); // 14
    println!("{}", phone_drop(4, 456)); // 11
    println!("{}", phone_drop(2, 789)); // 40
    println!("{}", phone_drop(3, 789)); // 17
    println!("{}", phone_drop(4, 789)); // 12
    optional_bonus();
}
