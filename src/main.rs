use std::io::stdin;
use word_unscrambler::{SCRABBLE_DICTIONARY_PATH, WordService};

fn main() {
    let word_service = WordService::new(SCRABBLE_DICTIONARY_PATH);
    println!("Welcome to Word Finder!");
    println!("Enter a word to find anagrams\n");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Unable to read input");

    let words = word_service.find_anagrams(input.trim());

    println!("\nInput: {}", input.trim());
    println!("Words Found: {}", words.len());
    print!("Words: {:?}", words);
}
