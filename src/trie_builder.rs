use crate::trie::Trie;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub(crate) trait TrieBuilder {
    fn build<T: Trie>(&self, trie: &mut T);
}

pub(crate) struct TxtFileTrieBuilder<'a> {
    file_path: &'a Path,
}

impl TxtFileTrieBuilder<'_> {
    pub(crate) fn new(path: &'_ str) -> TxtFileTrieBuilder {
        let file_path = Path::new(path);
        if !file_path.exists() {
            panic!("File path does not exist: {}", path);
        }
        if file_path.extension().unwrap() != "txt" {
            panic!("File is not a txt file")
        }
        TxtFileTrieBuilder { file_path }
    }
}

impl TrieBuilder for TxtFileTrieBuilder<'_> {
    fn build<T: Trie>(&self, trie: &mut T) {
        let file = File::open(self.file_path).expect("Could not open file");
        let bufreader = BufReader::new(file);

        for line in bufreader.lines() {
            if let Ok(word) = line {
                trie.insert(&word);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::trie::SimpleTrie;

    #[test]
    #[should_panic(expected = "File path does not exist: inexistent.txt")]
    fn txt_trie_builder_validates_file_path() {
        TxtFileTrieBuilder::new("inexistent.txt");
    }

    #[test]
    #[should_panic(expected = "File is not a txt file")]
    fn txt_trie_new_validates_file_extension() {
        TxtFileTrieBuilder::new("README.md");
    }

    #[test]
    fn txt_trie_builder_builds_trie() {
        let txt_file_trie_builder = TxtFileTrieBuilder::new("scrabble-dictionary.txt");
        let mut simple_trie = SimpleTrie::new();
        txt_file_trie_builder.build(&mut simple_trie);

        assert!(simple_trie.search("apple"));
        assert!(simple_trie.search("wizard"));
        assert!(!simple_trie.search("pelem"));
        assert!(simple_trie.starts_with("zz"));
    }
}