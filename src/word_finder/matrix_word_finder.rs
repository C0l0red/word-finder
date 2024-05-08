use std::collections::HashSet;
use std::rc::Rc;
use crate::word_finder::{WordFilters, WordFinder};
use crate::word_finder::matrix::{Direction, Matrix, Point};
use crate::word_finder::matrix::Direction::{Down, Left, LowerLeft, LowerRight, Right, Up, UpperLeft, UpperRight};
use crate::dictionaries::Dictionary;

struct WordFinderState {
    current_word: String,
    visited_points: HashSet<Point>,
    words_found: HashSet<String>,
}

pub(crate) struct MatrixWordFinder<T> {
    dictionary: Rc<T>,
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

impl<T: Dictionary> MatrixWordFinder<T> {
    pub(crate) fn new(dictionary: Rc<T>, diagonal: bool) -> MatrixWordFinder<T> {
        let directions: &[Direction] = if diagonal {
            &[
                Down, Right, Up, Left, LowerRight, LowerLeft, UpperRight, UpperLeft,
            ]
        } else {
            &[Down, Right, Up, Left]
        };

        MatrixWordFinder {
            dictionary,
            directions,
        }
    }

    fn traverse<F: WordFilters>(
        &self,
        point: Point,
        state: &mut WordFinderState,
        matrix: &Matrix,
        filters: &F,
    ) {
        if self.dictionary.starts_with(&state.current_word) {
            if filters.passes_filters(&state.current_word)
                && self.dictionary.search(&state.current_word)
            {
                state.words_found.insert(state.current_word.to_string());
            }

            state.visited_points.insert((point.0, point.1));
            state.current_word.push(matrix.get_letter(point));

            for direction in self.directions {
                if let Some(next_point) = matrix.next(point, direction) {
                    if state.visited_points.contains(&next_point) {
                    } else {
                        self.traverse(next_point, state, matrix, filters);
                    }
                }
            }

            state.current_word.pop();
            state.visited_points.remove(&(point.0, point.1));
        }
    }
}

impl<T: Dictionary> WordFinder<Matrix> for MatrixWordFinder<T> {
    fn search<F: WordFilters>(&self, matrix: &Matrix, filters: &F) -> HashSet<String> {
        let mut state = WordFinderState::new();

        for index in 0..matrix.get_capacity() {
            let point = matrix.get_point_from_index(index);
            self.traverse(point, &mut state, &matrix, filters);
        }
        state.words_found
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Instant;
    use crate::word_finder::BasicWordFilters;

    #[test]
    fn test_matrix_word_finder_search() {
        let dictionary = HashSet::from([
            "orals".to_string(),
            "clans".to_string(),
            "blair".to_string(),
            "scan".to_string(),
            "barons".to_string(),
            "poor".to_string(),
            "sabir".to_string(),
            "ribs".to_string()
        ]);

        let matrix = Matrix::new(&[
            &['p', 'i', 'i', 'e'],
            &['r', 'b', 'a', 'o'],
            &['n', 'l', 's', 'r'],
            &['c', 'a', 'n', 'o'],
        ]);
        let matrix_word_finder = MatrixWordFinder::new(Rc::new(dictionary), true);
        let filters: BasicWordFilters = Default::default();

        let start_time = Instant::now();
        let words = matrix_word_finder.search(&matrix, &filters);
        let end_time = Instant::now();

        println!("Time taken: {:?}", (end_time - start_time));
        println!("Length is {}", words.len());
        println!("Words found: {:?}", words);

        assert_eq!(words.len(), 6);
        assert!(words.contains("sabir"));
        assert!(words.contains("orals"));
        assert!(!words.contains("scan"));
        assert!(!words.contains("poor"));
    }
}
