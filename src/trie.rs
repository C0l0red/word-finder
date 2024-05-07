macro_rules! search_for_chars {
    ($prefix: expr, $current: expr) => {
        for letter in $prefix.chars() {
            let index = SimpleTrie::char_to_index(letter);
            if let None = $current.children[index] {
                return false;
            }
            $current = $current.children[index].as_deref().unwrap();
        }
    };
}

pub(crate) trait Trie {
    fn new() -> impl Trie;
    fn insert(&mut self, word: &str);
    fn search(&self, word: &str) -> bool;
    fn starts_with(&self, prefix: &str) -> bool;
}

#[derive(Clone)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_end_of_word: bool,
}

#[derive(Clone)]
pub(crate) struct SimpleTrie {
    root: TrieNode,
}

impl TrieNode {
    fn new() -> TrieNode {
        TrieNode {
            children: Default::default(),
            is_end_of_word: false,
        }
    }
}

impl SimpleTrie {
    fn char_to_index(letter: char) -> usize {
        if !letter.is_alphabetic() {
            panic!("Character {} is not an alphabet", letter);
        }
        letter.to_lowercase().next().unwrap() as usize - 'a' as usize
    }
}

impl Trie for SimpleTrie {
    fn new() -> SimpleTrie {
        SimpleTrie {
            root: TrieNode::new(),
        }
    }
    fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;

        for letter in word.chars() {
            let index = SimpleTrie::char_to_index(letter);
            if let None = current.children[index] {
                current.children[index] = Some(Box::from(TrieNode::new()));
            }
            current = current.children[index].as_deref_mut().unwrap();
        }
        current.is_end_of_word = true;
    }

    fn search(&self, word: &str) -> bool {
        let mut current = &self.root;
        search_for_chars!(word, current);
        current.is_end_of_word
    }

    fn starts_with(&self, prefix: &str) -> bool {
        let mut current = &self.root;
        search_for_chars!(prefix, current);
        true
    }
}

mod test {
    use super::*;

    #[test]
    fn test_insert_into_trie() {
        let mut trie = SimpleTrie::new();
        let words = ["apple", "app", "banana", "bat", "ball"];

        for word in words {
            trie.insert(word);
        }

        assert!(trie.search("apple"));
        assert!(trie.search("app"));
        assert!(trie.search("banana"));
        assert!(!trie.search("ban"));
        assert!(!trie.search("b"));
        assert!(trie.search("ball"));
        assert!(trie.starts_with("b"));
        assert!(trie.starts_with("ba"));
        assert!(trie.starts_with("ball"));
        assert!(!trie.starts_with("ballistic"));
        assert!(!trie.starts_with("cackle"));
    }
}