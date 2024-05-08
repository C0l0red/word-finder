use std::collections::HashSet;
use std::rc::Rc;
use crate::word_finder::{WordFilters, WordFinder};
use crate::dictionaries::Dictionary;

pub(crate) struct AnagramWordFinder<T: Dictionary> {
    dictionary: Rc<T>,
}

impl<T: Dictionary> AnagramWordFinder<T> {
    pub(crate) fn new(dictionary: Rc<T>) -> AnagramWordFinder<T> {
        AnagramWordFinder { dictionary }
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

impl<T: Dictionary> WordFinder<String> for AnagramWordFinder<T> {
    fn search<F: WordFilters>(&self, word: &String, filters: &F) -> HashSet<String> {
        let mut words = HashSet::new();
        self.find_permutations("", word, &mut words, filters);
        words
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::time::Instant;
    use crate::word_finder::BasicWordFilters;

    #[test]
    fn test_anagram_word_finder_search() {
        let dictionary = HashSet::from([
            "traction".to_string(),
            "electricity".to_string(),
            "cat".to_string(),
            "fear".to_string(),
            "apple".to_string(),
            "electric".to_string(),
            "craft".to_string(),
            "possible".to_string()
        ]);

        let string = String::from("electrification");
        let anagram_word_finder = AnagramWordFinder::new(Rc::new(dictionary));
        let filters: BasicWordFilters = Default::default();

        let start_time = Instant::now();
        let words = anagram_word_finder.search(&string, &filters);
        let end_time = Instant::now();

        println!(
            "Time taken: {:?} for {} letter word",
            end_time - start_time,
            string.len()
        );
        println!("Number of words found: {}", words.len());
        println!("{:?}", words);

        assert_eq!(words.len(), 5);
        assert!(words.contains("fear"));
        assert!(words.contains("traction"));
        assert!(!words.contains("electricity"));
    }
}
