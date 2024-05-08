use std::collections::HashSet;
use crate::dictionaries::Dictionary;

impl Dictionary for HashSet<String> {
    fn search(&self, word: &str) -> bool {
        self.contains(word)
    }

    fn starts_with(&self, prefix: &str) -> bool {
        self.iter().any(|word| word.starts_with(prefix))
    }
}