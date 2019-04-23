use std::env;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;
use std::collections::HashMap;
use regex::Regex;
use std::collections::BTreeMap;

extern crate regex;

#[derive(Debug)]
struct WordStore {
    counts: HashMap<String, u64>
}

impl WordStore {

    fn new() -> WordStore {
        WordStore {
            counts: HashMap::new()
        }
    }

    fn increment(&mut self, word: &str) {
        let key = word.to_string();
        let count = self.counts.entry(key).or_insert(0);
        *count += 1;
    }

    fn display(&self, min_frequency: u64) {
        let ordered: BTreeMap<_, _> = self.counts.iter()
            .filter(|pair| pair.1 >= &min_frequency)
            .collect();

        for (word, count) in ordered {
            println!("{}: {}", word, count);
        }
    }
}

fn normalize_word(word: &str) -> String {
    let separators_re = Regex::new(r"[ ,.!?]+").unwrap();
    let invalid_chars_re = Regex::new(r"[^a-z ]").unwrap();

    let lower = word.to_lowercase();
    let separators_replaced = separators_re.replace_all(&lower, " ");
    invalid_chars_re.replace_all(&separators_replaced, "").to_string()
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    println!("args {:?}", arguments);
    let filename = arguments[1].clone();
    let min_frequency: u64 = arguments[2].parse().unwrap();

    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut word_store = WordStore::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let words = line.split(" ");
        for word in words {
            if word == "" {
                continue;
            } else {
                let normalized_word = normalize_word(word);
                word_store.increment(&normalized_word);
            }
        }
    }

    word_store.display(min_frequency);
}