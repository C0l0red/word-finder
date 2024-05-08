
use std::collections::HashSet;
use std::rc::Rc;
use crate::dictionaries::tries::Trie;
use crate::word_finder::anagram_word_finder::{AnagramWordFinder};
use crate::word_finder::matrix::Matrix;
use crate::word_finder::matrix_word_finder::MatrixWordFinder;
use crate::dictionaries::tries::trie::{SimpleTrie};
use crate::dictionaries::tries::trie_builder::{TrieBuilder, TxtFileTrieBuilder};
use crate::word_finder::{BasicWordFilters, WordFinder};

mod word_finder;
mod dictionaries;

pub const SCRABBLE_DICTIONARY_PATH: &str = "scrabble-dictionary.txt";

pub struct WordService {
    matrix_word_finder: MatrixWordFinder<SimpleTrie>,
    anagram_word_finder: AnagramWordFinder<SimpleTrie>,
}

impl WordService {
    pub fn new(path: &str) -> WordService {
        let mut simple_trie = Rc::new(SimpleTrie::new());
        let txt_file_trie_builder = TxtFileTrieBuilder::new(path);
        txt_file_trie_builder.build(Rc::make_mut(&mut simple_trie));

        WordService {
            matrix_word_finder: MatrixWordFinder::new(Rc::clone(&simple_trie), true),
            anagram_word_finder: AnagramWordFinder::new(Rc::clone(&simple_trie)),
        }
    }

    pub fn find_anagrams(&self, word: &str) -> HashSet<String> {
        // TODO: implement filters for WordService
        let filters: BasicWordFilters = Default::default();
        self.anagram_word_finder.search(&String::from(word), &filters)
    }

    pub fn find_words_in_matrix(&self, nested_slice: &[&[char]]) -> HashSet<String> {
        let matrix = Matrix::new(nested_slice);
        let filters: BasicWordFilters = Default::default();
        self.matrix_word_finder.search(&matrix, &filters)
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
    fn test_find_anagrams() {
        let word_service = WordService::new(SCRABBLE_DICTIONARY_PATH);
        let words = word_service.find_anagrams("people");

        assert_eq!(words.len(), 24);
        assert!(words.contains("pope"));
        assert!(words.contains("peep"));
    }

    #[test]
    fn test_find_words_in_matrix() {
        let word_service = WordService::new(SCRABBLE_DICTIONARY_PATH);
        let nested_slice: &[&[char]] = &[
            &['r', 'u', 'g', 's'],
            &['e', 'a', 't', 's'],
            &['s', 'p', 'o', 't'],
            &['h', 'e', 'l', 'p'],
        ];
        let words = word_service.find_words_in_matrix(&nested_slice);

        assert_eq!(words.len(), 320);
        assert!(words.contains("stoats"));
        assert!(words.contains("guttles"));
        assert!(words.contains("helots"));
        assert!(words.contains("plots"));
    }
}
