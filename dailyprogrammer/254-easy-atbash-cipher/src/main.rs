// Ref: https://www.reddit.com/r/dailyprogrammer/comments/45w6ad/20160216_challenge_254_easy_atbash_cipher/

fn atbash_encrypt(n: &str) -> String {
    let mut ret_string = String::new();

    for i in n.chars() {
        if !i.is_ascii_alphabetic() {
            ret_string.push(i)
        } else {
            let difference = if i.is_ascii_uppercase() { 65 } else { 97 };
            let new_letter = 25 - (i as u8 - difference);
            ret_string.push((new_letter + difference) as char);
        }
    }
    ret_string
}

#[allow(dead_code)]
fn atbash_encrypt2(n: &str) -> String {
    n.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                (base + (25 - (c as u8 - base))) as char
            } else {
                c
            }
        })
        .collect()
}

fn main() {
    println!("{}", atbash_encrypt("foobar"));
    println!("{}", atbash_encrypt("wizard"));
    println!("{}", atbash_encrypt("/r/dailyprogrammer"));
    println!("{}", atbash_encrypt("gsrh rh zm vcznkov lu gsv zgyzhs xrksvi"));
}
