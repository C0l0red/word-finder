use std::collections::HashSet;
use crate::grid::{Direction, Grid, Point};
use crate::grid::Direction::{Down, Left, LowerLeft, LowerRight, Right, Up, UpperLeft, UpperRight};
use crate::trie::Trie;

type PointSet = HashSet<Point>;

pub(crate) trait WordFinder<T> {
    fn search(&self, structure: &T) -> HashSet<String>;
}

struct WordFinderState {
    current_word: String,
    visited_points: PointSet,
    words_found: HashSet<String>
}

pub(crate) struct GridWordFinder<T> {
    dictionary: T,
    directions: &'static [Direction],
}

impl WordFinderState {
    fn new() -> WordFinderState {
        WordFinderState {
            current_word: String::new(),
            visited_points: HashSet::new(),
            words_found: HashSet::new(),
        }
    }
}

impl<T: Trie> GridWordFinder<T> {
    pub(crate) fn new(dictionary: T, diagonal: bool) -> GridWordFinder<T> {
        let directions: &[Direction] = if diagonal {
            &[Down, Right, Up, Left, LowerRight, LowerLeft, UpperRight, UpperLeft]
        } else {
            &[Down, Right, Up, Left]
        };

        GridWordFinder {
            dictionary,
            directions,
        }
    }

    fn traverse(&self, point: Point, state: &mut WordFinderState, grid: &Grid) {
        if self.dictionary.starts_with(&state.current_word) {

            if self.dictionary.search(&state.current_word) {
                state.words_found.insert(state.current_word.to_string());
            }

            state.visited_points.insert((point.0, point.1));
            state.current_word.push(grid.get_letter(point));

            for direction in self.directions {
                if let Some(next_point) = grid.next(point, direction) {
                    if state.visited_points.contains(&next_point) {} else {
                        self.traverse(next_point, state, grid);
                    }
                }
            }

            state.current_word.pop();
            state.visited_points.remove(&(point.0, point.1));
        }
    }
}

impl<T: Trie> WordFinder<Grid> for GridWordFinder<T> {
    fn search(&self, grid: &Grid) -> HashSet<String> {
        let mut state = WordFinderState::new();

        for index in 0..grid.get_capacity() {
            let point = grid.get_point_from_index(index);
            self.traverse(point, &mut state, &grid);
        }
        state.words_found
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Instant;
    use crate::grid::Grid;
    use crate::trie::SimpleTrie;
    use crate::trie_builder::{TrieBuilder, TxtFileTrieBuilder};

    #[test]
    fn test_grid_word_finder_search() {

        let txt_file_trie_builder = TxtFileTrieBuilder::new("scrabble-dictionary.txt");
        let mut simple_trie = SimpleTrie::new();
        txt_file_trie_builder.build(&mut simple_trie);

        let grid = Grid::new(&[
            &['p', 'i', 'i', 'e'],
            &['r', 'b', 'a', 'o'],
            &['n', 'l', 's', 'r'],
            &['c', 'a', 'n', 'o'],
        ]);
        let grid_word_finder = GridWordFinder::new(simple_trie, true);

        let start_time = Instant::now();
        let words = grid_word_finder.search(&grid);
        let end_time = Instant::now();

        println!("Time taken: {:?}", (end_time - start_time));
        println!("Length is {}", words.len());
        println!("Words found: {:?}", words);
    }
}