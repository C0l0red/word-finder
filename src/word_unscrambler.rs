use std::collections::HashSet;
use crate::trie::Trie;

pub(crate) trait WordUnscrambler {
    fn unscramble(&self, word: &str) -> HashSet<String>;
}

pub(crate) struct PrefixBasedWordUnscrambler<T: Trie> {
    dictionary: T
}

impl<T: Trie> PrefixBasedWordUnscrambler<T> {
    pub(crate) fn new(dictionary: T) -> PrefixBasedWordUnscrambler<T> {
        PrefixBasedWordUnscrambler {
            dictionary
        }
    }

    fn permutations(&self, prefix: &str, suffix: &str, trie: &T, words: &mut HashSet<String>) {
        if suffix.len() == 0 {
            return;
        } else {
            for (index, char) in suffix.chars().enumerate() {
                let new_prefix = format!("{}{}", prefix, char);
                let new_suffix = format!("{}{}", &suffix[0..index], &suffix[index + 1..]);

                if trie.starts_with(&new_prefix) {
                    if trie.search(&new_prefix) {
                        words.insert(String::from(&new_prefix));
                    }
                    self.permutations(&new_prefix, &new_suffix, trie, words);
                }
            }
        }
    }
}

impl<T: Trie> WordUnscrambler for PrefixBasedWordUnscrambler<T> {
    fn unscramble(&self, word: &str) -> HashSet<String> {
        let mut words = HashSet::new();
        self.permutations("", word, &self.dictionary, &mut words);
        words
    }
}

#[cfg(test)]
mod test {
    use std::time::Instant;
    use crate::trie::SimpleTrie;
    use crate::trie_builder::{TrieBuilder, TxtFileTrieBuilder};
    use super::*;

    #[test]
    fn test_permutations() {
        let txt_file_trie_builder = TxtFileTrieBuilder::new("scrabble-dictionary.txt");
        let mut simple_trie = SimpleTrie::new();
        txt_file_trie_builder.build(&mut simple_trie);

        let string = "electrification";
        let prefix_based_word_unscrambler = PrefixBasedWordUnscrambler::new(simple_trie);

        let start_time = Instant::now();
        let words = prefix_based_word_unscrambler.unscramble(string);
        let end_time = Instant::now();

        println!("Time taken: {:?} for {} letter word", end_time - start_time, string.len());
        println!("Number of words found: {}", words.len());
        println!("{:?}", words);
    }
}