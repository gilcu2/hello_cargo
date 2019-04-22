use std::env;
use std::fs::File;
use std::io::prelude::BufRead;
use std::io::BufReader;
use std::collections::HashMap;

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
        for (key, value) in self.counts.iter() {
            if value >= &min_frequency {
                println!("{}: {}", key, value);
            }
        }
    }
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
                let normalized_word = word.to_lowercase();
                word_store.increment(&normalized_word);
            }
        }
    }

    word_store.display(min_frequency);
}