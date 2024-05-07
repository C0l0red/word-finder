use crate::grid::Direction::{Down, Left, Right, Up, UpperLeft, UpperRight, LowerLeft, LowerRight};

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

pub(crate) type Point = (u8, u8);

#[derive(PartialEq, Debug)]
struct Cell {
    letter: char,
}

pub(crate) struct Grid {
    cells: Vec<Cell>,
    height: u8,
    width: u8,
    capacity: u8,
}

impl Grid {
    pub(crate) fn new(nested_slice: &[&[char]]) -> Grid {
        let mut cell = Vec::new();
        let width: u8 = nested_slice.get(0).unwrap().len() as u8;
        let height = nested_slice.len() as u8;

        for row in nested_slice.iter() {
            if row.len() != width as usize {
                panic!("Rows are of different sizes");
            }

            for col in row.iter() {
                cell.push(Cell {
                    letter: *col,
                })
            }
        }

        Grid {
            cells: cell,
            height,
            width,
            capacity: width * height,
        }
    }
    pub(crate) fn get_letter(&self, point: Point) -> char {
        self.get_cell(point).unwrap().letter
    }

    pub(crate) fn get_capacity(&self) -> u8 {
        self.capacity
    }

    fn get_cell(&self, point: Point) -> Option<&Cell> {
        if point.0 < self.width && point.1 < self.height {
            let index = point.0 * self.width + point.1;
            self.cells.get(index as usize)
        } else {
            None
        }
    }

    pub(crate) fn get_point_from_index(&self, index: u8) -> Point {
        if index < self.capacity {
            (index / self.width, index % self.width)
        } else {
            panic!("Index out of bounds");
        }
    }

    fn validate_point(&self, row: u8, col: u8) -> bool {
        let point = (row, col);
        match self.get_cell(point) {
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
            LowerRight => (point.0 +1, point.1 + 1),
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
    use super::*;

    #[test]
    fn test_grid_new() {
        let grid = Grid::new(&[
            &['c', 'b', 'c'],
            &['a', 'e', 'e'],
            &['t', 't', 'l']
        ]);

        assert_eq!(grid.cells.get(0).unwrap().letter, 'c');
        assert_eq!(grid.cells.get(3).unwrap().letter, 'a');
        assert_eq!(grid.cells.get(4).unwrap().letter, 'e');
        assert_eq!(grid.cells.get(8).unwrap().letter, 'l');
    }

    #[test]
    #[should_panic(expected = "Rows are of different sizes")]
    fn test_grid_new_panics_for_uneven_matrix() {
        Grid::new(&[
            &['a', 'b'],
            &['d', 'e', 'f'],
            &['g', 'h', 'i']
        ]);
    }

    #[test]
    fn test_grid_get_cell() {
        let grid = Grid::new(&[
            &['a', 'b', 'c'],
            &['d', 'e', 'f'],
            &['g', 'h', 'i']
        ]);

        assert_eq!(grid.get_cell((0, 0)).unwrap().letter, 'a');
        assert_eq!(grid.get_cell((2, 2)).unwrap().letter, 'i');
    }

    #[test]
    fn test_grid_next() {
        let grid = Grid::new(&[&['a', 'b', 'c'], &['d', 'e', 'f'], &['g', 'h', 'i']]);

        assert_eq!(grid.next((0, 0), &Down).unwrap(), (1, 0));
        assert_eq!(grid.next((0, 1), &Left).unwrap(), (0, 0));
        assert_eq!(grid.next((2, 2), &Right), None);
        assert_eq!(grid.next((1, 1), &Up).unwrap(), (0, 1));
    }

    #[test]
    fn test_grid_get_point_from_index() {
        let grid = Grid::new(&[&['a', 'b', 'c'], &['d', 'e', 'f'], &['g', 'h', 'i']]);

        assert_eq!(grid.get_point_from_index(5), (1, 2));
        assert_eq!(grid.get_point_from_index(7), (2, 1));
        assert_eq!(grid.get_point_from_index(0), (0, 0));
        assert_eq!(grid.get_point_from_index(8), (2, 2));
    }
}
