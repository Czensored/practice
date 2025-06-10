// Ref: https://www.reddit.com/r/dailyprogrammer/comments/aq6gfy/20190213_challenge_375_intermediate_a_card/
// Ref: https://www.youtube.com/watch?v=CCxs-tu8tOU

fn flipping_seq(v: &Vec<usize>) -> bool {
    if v.iter().sum::<usize>() % 2 == 0 {
        println!("no solution");
        return false;
    }

    let mut v_cl = v.clone();
    let mut order = Vec::new();
    let mut before = Vec::new();

    for ind in 0..v_cl.len() {
        if v_cl[ind] == 1 {
            order.push(ind);
            order.extend(before.drain(..).rev());
            if ind + 1 < v_cl.len() {
                v_cl[ind + 1] = if v_cl[ind + 1] == 0 { 1 } else { 0 };
            }
        } else {
            before.push(ind);
        }
    }
    order.extend(before.drain(..).rev());

    for i in order {
        print!("{} ", i);
    }
    println!();

    true
}

fn inputs_to_vec(inputs: &[&str]) -> Vec<Vec<usize>> {
    inputs
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn main() {
    let inputs = vec![
        "0100110",
        "01001100111",
        "100001100101000",
    ];

    for i in inputs_to_vec(&inputs) {
        flipping_seq(&i);
    }

    let challenge_inputs = vec![
        "0100110",
        "001011011101001001000",
        "1010010101001011011001011101111",
        "1101110110000001010111011100110",
        "010111111111100100101000100110111000101111001001011011000011000",
    ];

    for i in inputs_to_vec(&challenge_inputs) {
        flipping_seq(&i);
    }
}
