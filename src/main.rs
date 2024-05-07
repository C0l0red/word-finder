use std::io::stdin;
use word_unscrambler::WordService;

fn main() {
    let word_service = WordService::new("scrabble-dictionary.txt");
    println!("Welcome to Word Unscrambler!");
    println!("Enter a word to unscramble\n");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Unable to read input");

    let words = word_service.unscramble_word(input.trim());

    println!("\nInput: {}", input.trim());
    println!("Words Found: {}", words.len());
    print!("Words: {:?}", words);
}