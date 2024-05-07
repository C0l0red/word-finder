use crate::trie::Trie;
use std::collections::HashSet;

pub(crate) trait WordUnscrambler {
    fn unscramble<F: WordFilters>(&self, word: &str, filters: F) -> HashSet<String>;
}

pub(crate) trait WordFilters {
    fn passes_filters(&self, word: &str) -> bool;
}

pub(crate) struct PrefixBasedWordUnscrambler<T: Trie> {
    dictionary: T,
}

#[derive(Default)]
pub struct BasicWordFilters {
    starts_with: String,
    ends_with: String,
    contains: String,
}

impl<T: Trie> PrefixBasedWordUnscrambler<T> {
    pub(crate) fn new(dictionary: T) -> PrefixBasedWordUnscrambler<T> {
        PrefixBasedWordUnscrambler { dictionary }
    }

    fn find_permutations<F: WordFilters>(
        &self,
        prefix: &str,
        suffix: &str,
        words: &mut HashSet<String>,
        filters: &F,
    ) {
        if suffix.len() == 0 {
            return;
        } else {
            for (index, char) in suffix.chars().enumerate() {
                let new_prefix = format!("{}{}", prefix, char);
                let new_suffix = format!("{}{}", &suffix[0..index], &suffix[index + 1..]);

                if self.dictionary.starts_with(&new_prefix) {
                    if filters.passes_filters(&new_prefix) && self.dictionary.search(&new_prefix) {
                        words.insert(String::from(&new_prefix));
                    }
                    self.find_permutations(&new_prefix, &new_suffix, words, filters);
                }
            }
        }
    }
}

impl<T: Trie> WordUnscrambler for PrefixBasedWordUnscrambler<T> {
    fn unscramble<F: WordFilters>(&self, word: &str, filters: F) -> HashSet<String> {
        let mut words = HashSet::new();
        self.find_permutations("", word, &mut words, &filters);
        words
    }
}

impl BasicWordFilters {
    pub(crate) fn new(starts_with: &str, ends_with: &str, contains: &str) -> BasicWordFilters {
        BasicWordFilters {
            starts_with: starts_with.to_owned(),
            ends_with: ends_with.to_owned(),
            contains: contains.to_owned(),
        }
    }
}

impl WordFilters for BasicWordFilters {
    fn passes_filters(&self, word: &str) -> bool {
        word.starts_with(self.starts_with.as_str())
            && word.ends_with(self.ends_with.as_str())
            && word.contains(self.contains.as_str())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::trie::SimpleTrie;
    use crate::trie_builder::{TrieBuilder, TxtFileTrieBuilder};
    use std::time::Instant;

    #[test]
    fn test_prefix_based_word_unscrambler() {
        let txt_file_trie_builder = TxtFileTrieBuilder::new("scrabble-dictionary.txt");
        let mut simple_trie = SimpleTrie::new();
        txt_file_trie_builder.build(&mut simple_trie);

        let string = "electrification";
        let prefix_based_word_unscrambler = PrefixBasedWordUnscrambler::new(simple_trie);
        let filters: BasicWordFilters = BasicWordFilters::new("", "", "");

        let start_time = Instant::now();
        let words = prefix_based_word_unscrambler.unscramble(string, filters);
        let end_time = Instant::now();

        println!(
            "Time taken: {:?} for {} letter word",
            end_time - start_time,
            string.len()
        );
        println!("Number of words found: {}", words.len());
        println!("{:?}", words);
    }

    #[test]
    fn test_new_word_unscrambler_filters() {
        let filters = BasicWordFilters::new("p", "op", "o");

        assert_eq!(filters.contains, "o".to_owned());
        assert_eq!(filters.starts_with, "p".to_owned());
        assert_eq!(filters.ends_with, "op".to_owned());
    }

    #[test]
    fn test_check_word_validity() {
        let filters = BasicWordFilters::new("app", "ion", "cat");

        assert!(filters.passes_filters("application"));
        assert!(!filters.passes_filters("apple"));
        assert!(!filters.passes_filters("caution"));
        assert!(!filters.passes_filters("bobcat"));
    }
}
