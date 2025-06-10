// Ref: https://www.reddit.com/r/dailyprogrammer/comments/myx3wn/20210426_challenge_387_easy_caesar_cipher/

fn caesar_shift_char(ch: char, n: u8) -> char {
    if !ch.is_ascii_alphabetic() {
        return ch;
    }
    let base = if ch.is_ascii_uppercase() { b'A' } else { b'a' };
    let shifted = (ch as u8 - base + n) % 26 + base;
    shifted as char
}

#[allow(dead_code)]
fn caesar_shift_string(st: &str, n: u8) -> String {
    let mut ret_string = String::new();
    for ch in st.chars() {
        ret_string.push(caesar_shift_char(ch, n));
    }
    ret_string
}

fn caesar_shift(st: &str, n: u8) -> String {
    st.chars().map(|ch| caesar_shift_char(ch, n)).collect()
}

fn decode_caesar_shift(st: &str) -> (u8, String) {
    let char_frequency = [3, -1, 1, 1, 4, 0, 0, 2, 2, -5, -2, 1, 0, 2, 3, 0, -6, 2, 2, 3, 1, -1, 0, -5, 0, -7];
    let mut largest = (-100, 0);
    for n in 0..26 {
        let new_text = caesar_shift(st, n);
        let mut sum = 0;

        for ch in new_text.chars() {
            if !ch.is_ascii_alphabetic() { continue; }

            let base = if ch.is_ascii_uppercase() { b'A' } else { b'a' };
            sum += char_frequency[(ch as u8 - base) as usize];
        }
        if sum > largest.0 {
            largest = (sum, n);
        }
    }
    (26 - largest.1, caesar_shift(st, largest.1))
}

fn main() {
    // let caesar = caesar_shift("Daily Programmer!", 6);
    let caesar = decode_caesar_shift("Tfdv ef wlikyvi, wfi uvrky rnrzkj pfl rcc nzky erjkp, szx, gfzekp kvvky.");
    // let caesar = decode_caesar_shift("EXXEGOEXSRGI");

    println!("{}: {}", caesar.0, caesar.1);
}
