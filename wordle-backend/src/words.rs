use lazy_static::lazy_static;
use std::{collections::HashMap, fs};

lazy_static! {
    pub static ref WORDS: HashMap<u8, Vec<String>> = {
        fs::read_to_string("words.txt")
            .expect("Failed to read words file")
            .lines()
            .filter(|word| (3..=15).contains(&(word.len() as u8)))
            .map(str::to_string)
            .fold(HashMap::new(), |mut acc, word| {
                acc.entry(word.len() as u8).or_default().push(word);
                acc
            })
    };
}

pub fn get_random_words(length: u8, count: usize) -> Vec<String> {
    use rand::prelude::IndexedRandom;

    let mut rng = rand::rng();
    WORDS
        .get(&length)
        .unwrap_or(&Vec::new())
        .choose_multiple(&mut rng, count)
        .cloned()
        .collect()
}
