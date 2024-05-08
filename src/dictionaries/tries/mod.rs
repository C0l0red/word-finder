use crate::dictionaries::Dictionary;

pub(crate) mod trie;
pub(crate) mod trie_builder;

pub(crate) trait Trie: Dictionary {
    fn new() -> impl Dictionary;
    fn insert(&mut self, word: &str);
}