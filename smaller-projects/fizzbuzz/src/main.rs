fn fizzbuzz(n: usize) {
    let rules = vec![
        (3, "Fizz"),
        (5, "Buzz"),
        (7, "Baz"),
    ];

    for i in 1..=n {
        let mut output = String::new();
        for (num, val) in &rules {
            if i % num == 0 {
                output.push_str(val);
            }
        }
        if output.is_empty() {
            println!("{}", i);
        } else {
            println!("{}", output);
        }
    }
}

fn main() {
    fizzbuzz(105);
}
