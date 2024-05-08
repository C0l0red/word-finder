use crate::grid::Grid;
use crate::trie::{SimpleTrie, Trie};
use crate::trie_builder::{TrieBuilder, TxtFileTrieBuilder};
use crate::word_finder::{GridWordFinder, WordFinder};
use crate::word_unscrambler::{BasicWordFilters, PrefixBasedWordUnscrambler, WordUnscrambler};
use std::collections::HashSet;
use std::rc::Rc;

mod grid;
mod trie;
mod trie_builder;
mod word_finder;
mod word_unscrambler;

pub const SCRABBLE_DICTIONARY_PATH: &str = "scrabble-dictionary.txt";

pub struct WordService {
    word_finder: GridWordFinder<SimpleTrie>,
    word_unscrambler: PrefixBasedWordUnscrambler<SimpleTrie>,
}

impl WordService {
    pub fn new(path: &str) -> WordService {
        let mut simple_trie = Rc::new(SimpleTrie::new());
        let txt_file_trie_builder = TxtFileTrieBuilder::new(path);
        txt_file_trie_builder.build(Rc::make_mut(&mut simple_trie));

        WordService {
            word_finder: GridWordFinder::new(Rc::clone(&simple_trie), true),
            word_unscrambler: PrefixBasedWordUnscrambler::new(Rc::clone(&simple_trie)),
        }
    }

    pub fn unscramble_word(&self, word: &str) -> HashSet<String> {
        // TODO: implement filters for WordService
        let filters: BasicWordFilters = Default::default();
        self.word_unscrambler.unscramble(word, filters)
    }

    pub fn find_words_in_grid(&self, nested_slice: &[&[char]]) -> HashSet<String> {
        let grid = Grid::new(nested_slice);
        self.word_finder.search(&grid)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new_word_service() {
        WordService::new(SCRABBLE_DICTIONARY_PATH);
    }

    #[test]
    fn test_unscramble_word() {
        let word_service = WordService::new(SCRABBLE_DICTIONARY_PATH);
        let words = word_service.unscramble_word("people");

        assert_eq!(words.len(), 24);
        assert!(words.contains("pope"));
        assert!(words.contains("peep"));
    }

    #[test]
    fn test_find_words_in_grid() {
        let word_service = WordService::new(SCRABBLE_DICTIONARY_PATH);
        let nested_slice: &[&[char]] = &[
            &['r', 'u', 'g', 's'],
            &['e', 'a', 't', 's'],
            &['s', 'p', 'o', 't'],
            &['h', 'e', 'l', 'p'],
        ];
        let words = word_service.find_words_in_grid(&nested_slice);
        println!("{:?}", words);

        assert_eq!(words.len(), 320);
        assert!(words.contains("stoats"));
        assert!(words.contains("guttles"));
        assert!(words.contains("helots"));
        assert!(words.contains("plots"));
    }
}
