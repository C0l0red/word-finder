use crate::word_finder::matrix::Direction::{Down, Left, LowerLeft, LowerRight, Right, Up, UpperLeft, UpperRight};

pub(crate) enum Direction {
    Up,
    UpperLeft,
    UpperRight,
    Down,
    LowerLeft,
    LowerRight,
    Left,
    Right,
}

pub(crate) type Point = (usize, usize);

#[derive(PartialEq, Debug)]
struct MatrixElement {
    letter: char,
}

pub(crate) struct Matrix {
    elements: Vec<MatrixElement>,
    height: usize,
    width: usize,
    capacity: usize,
}

impl Matrix {
    pub(crate) fn new(nested_slice: &[&[char]]) -> Matrix {
        let mut elements = Vec::new();
        let width = nested_slice.get(0).unwrap().len();
        let height = nested_slice.len();

        for row in nested_slice.iter() {
            if row.len() != width as usize {
                panic!("Rows are of different sizes");
            }

            for col in row.iter() {
                elements.push(MatrixElement { letter: *col })
            }
        }

        Matrix {
            elements,
            height,
            width,
            capacity: width * height,
        }
    }
    pub(crate) fn get_letter(&self, point: Point) -> char {
        self.get_element(point).unwrap().letter
    }

    pub(crate) fn get_capacity(&self) -> usize {
        self.capacity
    }

    fn get_element(&self, point: Point) -> Option<&MatrixElement> {
        if point.0 < self.width && point.1 < self.height {
            let index = point.0 * self.width + point.1;
            self.elements.get(index as usize)
        } else {
            None
        }
    }

    pub(crate) fn get_point_from_index(&self, index: usize) -> Point {
        if index < self.capacity {
            (index / self.width, index % self.width)
        } else {
            panic!("Index out of bounds");
        }
    }

    fn validate_point(&self, row: usize, col: usize) -> bool {
        let point = (row, col);
        match self.get_element(point) {
            None => false,
            Some(_) => true,
        }
    }

    pub(crate) fn next(&self, point: Point, direction: &Direction) -> Option<Point> {
        let (new_row, new_col) = match direction {
            Up => (point.0.checked_sub(1)?, point.1),
            Down => (point.0 + 1, point.1),
            Left => (point.0, point.1.checked_sub(1)?),
            Right => (point.0, point.1 + 1),
            UpperLeft => (point.0.checked_sub(1)?, point.1.checked_sub(1)?),
            UpperRight => (point.0.checked_sub(1)?, point.1 + 1),
            LowerLeft => (point.0 + 1, point.1.checked_sub(1)?),
            LowerRight => (point.0 + 1, point.1 + 1),
        };

        if self.validate_point(new_row, new_col) {
            Some((new_row, new_col))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use crate::word_finder::matrix::Direction::{Down, Left, Right, Up};
    use super::*;

    #[test]
    fn test_new_matrix() {
        let matrix = Matrix::new(&[&['c', 'b', 'c'], &['a', 'e', 'e'], &['t', 't', 'l']]);

        assert_eq!(matrix.elements.get(0).unwrap().letter, 'c');
        assert_eq!(matrix.elements.get(3).unwrap().letter, 'a');
        assert_eq!(matrix.elements.get(4).unwrap().letter, 'e');
        assert_eq!(matrix.elements.get(8).unwrap().letter, 'l');
    }

    #[test]
    #[should_panic(expected = "Rows are of different sizes")]
    fn test_new_matrix_panics_for_uneven_matrix() {
        Matrix::new(&[&['a', 'b'], &['d', 'e', 'f'], &['g', 'h', 'i']]);
    }

    #[test]
    fn test_get_matrix_element() {
        let matrix = Matrix::new(&[&['a', 'b', 'c'], &['d', 'e', 'f'], &['g', 'h', 'i']]);

        assert_eq!(matrix.get_element((0, 0)).unwrap().letter, 'a');
        assert_eq!(matrix.get_element((2, 2)).unwrap().letter, 'i');
    }

    #[test]
    fn test_matrix_next() {
        let matrix = Matrix::new(&[&['a', 'b', 'c'], &['d', 'e', 'f'], &['g', 'h', 'i']]);

        assert_eq!(matrix.next((0, 0), &Down).unwrap(), (1, 0));
        assert_eq!(matrix.next((0, 1), &Left).unwrap(), (0, 0));
        assert_eq!(matrix.next((2, 2), &Right), None);
        assert_eq!(matrix.next((1, 1), &Up).unwrap(), (0, 1));
    }

    #[test]
    fn test_get_point_from_index() {
        let matrix = Matrix::new(&[&['a', 'b', 'c'], &['d', 'e', 'f'], &['g', 'h', 'i']]);

        assert_eq!(matrix.get_point_from_index(5), (1, 2));
        assert_eq!(matrix.get_point_from_index(7), (2, 1));
        assert_eq!(matrix.get_point_from_index(0), (0, 0));
        assert_eq!(matrix.get_point_from_index(8), (2, 2));
    }
}
