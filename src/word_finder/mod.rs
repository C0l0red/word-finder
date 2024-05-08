use std::collections::HashSet;

pub(crate) mod matrix_word_finder;
pub(crate) mod matrix;
pub(crate) mod anagram_word_finder;

pub(crate) trait WordFinder<T> {
    fn search<F: WordFilters>(&self, data: &T, filters: &F) -> HashSet<String>;
}

pub(crate) trait WordFilters {
    fn passes_filters(&self, word: &str) -> bool;
}

#[derive(Default)]
pub struct BasicWordFilters {
    starts_with: String,
    ends_with: String,
    contains: String,
}

impl BasicWordFilters {
    fn new(starts_with: &str, ends_with: &str, contains: &str) -> BasicWordFilters {
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

    #[test]
    fn test_new_basic_word_filters() {
        let filters = BasicWordFilters::new("p", "op", "o");

        assert_eq!(filters.contains, "o".to_owned());
        assert_eq!(filters.starts_with, "p".to_owned());
        assert_eq!(filters.ends_with, "op".to_owned());
    }

    #[test]
    fn test_passes_filter() {
        let filters = BasicWordFilters::new("app", "ion", "cat");

        assert!(filters.passes_filters("application"));
        assert!(!filters.passes_filters("apple"));
        assert!(!filters.passes_filters("caution"));
        assert!(!filters.passes_filters("bobcat"));
    }
}