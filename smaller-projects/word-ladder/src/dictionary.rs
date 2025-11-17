use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Dictionary {
    words: Vec<String>,
    used: Vec<bool>,
}

impl Dictionary {
    pub fn new(input_file_name: &str) -> Self {
        let file = File::open(input_file_name).expect("Failed to open file");
        let mut dictionary = Dictionary {
            words: Vec::new(),
            used: Vec::new(),
        };

        for line in BufReader::new(file).lines() {
            let word = line.expect("Failed to read line");
            dictionary.words.push(word);
        }

        dictionary.used = vec![false; dictionary.words.len()];
        dictionary
    }

    pub fn size(&self) -> usize {
        self.words.len()
    }

    pub fn member(&self, word: &str) -> Option<usize> {
        for (i, w) in self.words.iter().enumerate() {
            if w == word {
                return Some(i);
            }
        }
        None
    }

    pub fn path_from_to(&mut self, from: &str, to: &str) -> Vec<String> {
        self.reset_path();

        let Some(from_index) = self.member(from) else {
            return vec![];
        };
        let Some(to_index) = self.member(to) else {
            return vec![];
        };

        if from_index == to_index {
            return vec![self.words[from_index].clone()];
        }

        let prev = self.bfs_shortest_path(from_index, to_index);
        self.reconstruct_path(to_index, &prev)
    }

    fn bfs_shortest_path(&mut self, from_index: usize, to_index: usize) -> Vec<Option<usize>> {
        let mut queue = VecDeque::new();
        let mut prev = vec![None; self.words.len()];

        self.used[from_index] = true;
        queue.push_back(from_index);

        while let Some(current_idx) = queue.pop_front() {
            for i in self.successors(current_idx) {
                self.used[i] = true;
                prev[i] = Some(current_idx);
                if i == to_index {
                    return prev;
                }
                queue.push_back(i);
            }
        }

        prev
    }

    fn successors(&self, index: usize) -> Vec<usize> {
        let word = &self.words[index];
        self.words
            .iter()
            .enumerate()
            .filter_map(|(i, candidate)| {
                if !self.used[i] && Dictionary::positional_diff(word, candidate) == 1 {
                    Some(i)
                } else {
                    None
                }
            })
            .collect()
    }

    fn reconstruct_path(&self, to_index: usize, prev: &[Option<usize>]) -> Vec<String> {
        if prev[to_index].is_none() {
            return vec![];
        }

        let mut path = vec![];
        let mut at = Some(to_index);
        while let Some(i) = at {
            path.push(self.words[i].clone());
            at = prev[i];
        }
        path.reverse();
        path
    }

    #[allow(dead_code)]
    fn path_from_to_suboptimal(&mut self, from: &str, to: &str) -> Vec<String> {
        let mut ladder: Vec<(&str, usize)> = Vec::new();
        self.reset_path();

        if let Some(from_index) = self.member(from) {
            ladder.push((from, from_index));
            self.used[from_index] = true;
        } else {
            println!("'{}' was not found in the dictionary.", from);
        }

        let mut backtrack_idx = 0;

        while !ladder.is_empty() && ladder.last().unwrap().0 != to {
            let idx = self.idx_of_successor_word_from(ladder.last().unwrap().0, backtrack_idx);
            if idx < self.words.len() {
                backtrack_idx = 0;
                ladder.push((&self.words[idx], idx));
                self.used[idx] = true;
            } else {
                backtrack_idx = ladder.last().unwrap().1;
                ladder.pop();
            }
        }

        let mut word_ladder: Vec<String> = Vec::new();
        for i in &ladder {
            word_ladder.push(i.0.to_string())
        }

        word_ladder
    }

    fn positional_diff(word1: &str, word2: &str) -> usize {
        assert_eq!(word1.len(), word2.len());
        let mut counter = 0;

        for (char1, char2) in word1.chars().zip(word2.chars()) {
            if char1 != char2 {
                counter += 1;
            }
        }

        counter
    }

    fn reset_path(&mut self) {
        self.used = vec![false; self.size()];
    }

    fn idx_of_successor_word_from(&self, word: &str, from_idx: usize) -> usize {
        for i in from_idx..self.words.len() {
            if !self.used[i] && Dictionary::positional_diff(word, &self.words[i]) == 1 {
                return i;
            }
        }
        self.words.len()
    }
}
