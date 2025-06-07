// Ref: https://www.reddit.com/r/dailyprogrammer/comments/a72sdj/20181217_challenge_370_easy_upc_check_digits/

// My implementation
fn _upc(mut n: u64) -> u64 {
    assert!(n < 10u64.pow(12), "Input must be <12 digits.");
    let mut num_vec = Vec::new();
    while n > 0 {
        num_vec.push(n % 10);
        n /= 10;
    }
    // make sure there is 11 elements
    for _ in num_vec.len()..11 {
        num_vec.push(0);
    }
    num_vec.reverse();
    let mut sum = 0;
    for i in 0..6 {
        sum += num_vec[2 * i];
    }
    sum *= 3;
    for i in 0..5 {
        sum += num_vec[2 * i + 1];
    }
    (10 - sum % 10) % 10
}

// ChatGPT simplifyied my implementation
fn upc(mut n: u64) -> u64 {
    assert!(n < 10u64.pow(12), "Input must be <12 digits.");
    let mut digits = [0; 11];
    for i in (0..11).rev() {
        digits[i] = n % 10;
        n /= 10;
    }
    let sum: u64 = (0..6).map(|i| digits[2 * i]).sum::<u64>() * 3
                   + (0..5).map(|i| digits[2 * i + 1]).sum::<u64>();
    (10 - sum % 10) % 10
}


fn main() {
    println!("{}", upc(4210000526)); // 4
    println!("{}", upc(3600029145)); // 2
    println!("{}", upc(12345678910)); // 4
    println!("{}", upc(1234567)); // 0
}
