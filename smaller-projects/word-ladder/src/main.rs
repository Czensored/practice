mod dictionary;
use dictionary::Dictionary;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_start_target_words(input_file_name: &str) -> Vec<(String, String)> {
    let file = File::open(input_file_name).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut start_target_vector: Vec<(String, String)> = Vec::new();

    for line_result in reader.lines().map_while(Result::ok) {
        let mut words = line_result.split_whitespace();
        if let (Some(word1), Some(word2)) = (words.next(), words.next()) {
            start_target_vector.push((word1.to_string(), word2.to_string()));
        }
    }

    start_target_vector
}

fn main() {
    let pairs = "ladder_longer_dictionary_start_target_pairs.txt";
    let dict = "ladder_longer_dictionary_data.txt";

    let array_of_pairs_to_build_ladders: Vec<(String, String)> = read_start_target_words(pairs);
    let mut dictionary = Dictionary::new(dict);

    for (a, b) in &array_of_pairs_to_build_ladders {
        let my_ladder = dictionary.path_from_to(a, b);
        if my_ladder.is_empty() {
            println!("No ladder for the pair {} and {} exists.", a, b);
        } else {
            println!(
                "A ladder in {} word{} for the pair {} and {} is:",
                my_ladder.len(),
                if my_ladder.len() != 1 { "s" } else { "" },
                a,
                b
            );
            for i in &my_ladder {
                println!("{}", i);
            }
        }
        println!();
    }
}
