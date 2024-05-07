use std::collections::HashSet;
use crate::grid::Grid;
use crate::trie::{SimpleTrie, Trie};
use crate::trie_builder::{TrieBuilder, TxtFileTrieBuilder};
use crate::word_finder::{GridWordFinder, WordFinder};
use crate::word_unscrambler::{PrefixBasedWordUnscrambler, WordUnscrambler};

mod grid;
mod trie;
mod word_finder;
mod trie_builder;
mod word_unscrambler;

pub struct WordService {
    word_finder: GridWordFinder<SimpleTrie>,
    word_unscrambler: PrefixBasedWordUnscrambler<SimpleTrie>,
}

impl WordService {
    pub fn new(path: &str) -> WordService {
        let mut simple_trie = SimpleTrie::new();
        let txt_file_trie_builder = TxtFileTrieBuilder::new(path);
        txt_file_trie_builder.build(&mut simple_trie);

        WordService {
            word_finder: GridWordFinder::new(simple_trie.clone(), true),
            word_unscrambler: PrefixBasedWordUnscrambler::new(simple_trie),
        }
    }

    pub fn unscramble_word(&self, word: &str) -> HashSet<String> {
        self.word_unscrambler.unscramble(word)
    }

    pub fn find_words_in_grid(&self, nested_slice: &[&[char]]) -> HashSet<String> {
        let grid = Grid::new(nested_slice);
        self.word_finder.search(&grid)
    }
}

