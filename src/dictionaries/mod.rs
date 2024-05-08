pub(crate) mod tries;
mod sets;

pub(crate) trait Dictionary {
    fn search(&self, word: &str) -> bool;
    fn starts_with(&self, prefix: &str) -> bool;
}