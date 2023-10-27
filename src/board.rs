//! This module contains the board struct and associated functions
//! The board struct is a 9x9 array of options
//! Empty cells are represented by None
//! Filled cells are represented by Some(u8)
//!
//! The module also contains a function to solve the board by backtracking

use std::fmt::Display;
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Board {
    board: [[Option<u8>; 9]; 9],
}
struct Index(usize, usize);

impl Board {
    /// Create a new board from a 9x9 array of u8
    /// 0 represents an empty cell
    /// Any other number represents a filled cell
    /// # Example
    /// ```
    /// use sudoku_solver::board::{Board, Index};
    /// let board = Board::new(&[
    /// [0, 0, 3, 4, 0, 7, 0, 6, 0],
    /// [7, 0, 0, 0, 0, 0, 0, 4, 0],
    /// [0, 0, 0, 0, 1, 0, 2, 5, 0],
    /// [4, 8, 0, 3, 0, 0, 1, 0, 0],
    /// [0, 5, 0, 0, 0, 0, 0, 0, 2],
    /// [0, 6, 0, 0, 2, 0, 0, 0, 0],
    /// [0, 9, 0, 1, 0, 5, 0, 0, 8],
    /// [1, 0, 0, 6, 0, 0, 0, 0, 5],
    /// [0, 0, 0, 0, 0, 0, 4, 0, 0],
    /// ]);
    ///
    /// ```
    /// # Panics
    /// 1. Array not 9x9.
    /// 2. Array contains numbers other than 0-9.
    /// 3. Array contains non integers.
    pub fn new(board: &[[u8; 9]; 9]) -> Self {
        // convert to options
        let mut new_board = [[None; 9]; 9];
        for (i, row) in board.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                new_board[i][j] = match cell {
                    0 => None,
                    _ => Some(*cell),
                };
            }
        }
        Board { board: new_board }
    }
    /// Get the value of a cell at a given index
    fn get_cell(&self, index: &Index) -> Option<u8> {
        self.board[index.0][index.1]
    }
    /// Update the value of a cell at a given index
    fn update_cell(&mut self, index: &Index, value: u8) {
        self.board[index.0][index.1] = match value {
            0 => None,
            _ => Some(value),
        };
    }
    /// Get a row of the board
    fn get_row(&self, row: usize) -> [Option<u8>; 9] {
        self.board[row]
    }
    /// Get a column of the board
    fn get_column(&self, column: usize) -> [Option<u8>; 9] {
        let mut column_array = [None; 9];
        for (i, row) in self.board.iter().enumerate() {
            column_array[i] = row[column];
        }
        column_array
    }
    /// Get elements in the 3x3 subgrid that contains the given index
    fn get_subgrid(&self, index: &Index) -> [Option<u8>; 9] {
        let gridindex: Index = Index(index.0 / 3, index.1 / 3);
        let mut subgrid = [None; 9];
        for i in 0..3 {
            for j in 0..3 {
                let cell_index = Index(gridindex.0 * 3 + i, gridindex.1 * 3 + j);
                subgrid[i * 3 + j] = self.get_cell(&cell_index);
            }
        }
        subgrid
    }
    /// Get the possible valid entries for a given index
    fn valid_entries(&self, index: &Index) -> [bool; 9] {
        let mut possible_entries = [true; 9];
        self.get_row(index.0).iter().for_each(|x| match x {
            None => {}
            Some(x) => possible_entries[(x - 1) as usize] = false,
        });

        self.get_column(index.1).iter().for_each(|x| match x {
            None => {}
            Some(x) => possible_entries[(x - 1) as usize] = false,
        });

        self.get_subgrid(index).iter().for_each(|x| match x {
            None => {}
            Some(x) => possible_entries[(x - 1) as usize] = false,
        });

        possible_entries
    }
    /// Check if a given entry is valid
    fn is_valid_entry(&self, index: &Index) -> bool {
        let element = self.get_cell(index);
        if element.is_none() {
            // you can have none anywhere
            return true;
        }
        let elements = [
            self.get_row(index.0),
            self.get_column(index.1),
            self.get_subgrid(index),
        ]
        .concat();

        if elements.iter().filter(|x| *x == &element).count() > 3 {
            return false;
        }
        return true;
    }
    /// Check if the board is valid
    pub fn is_valid_board(&self) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                if !self.is_valid_entry(&Index(i, j)) {
                    return false;
                }
            }
        }
        return true;
    }
    /// Check if the board is complete
    fn is_complete(&self) -> bool {
        self.is_valid_board() && (self.next_empty().is_none())
    }
    /// Get the next empty cell
    fn next_empty(&self) -> Option<Index> {
        for (i, row) in self.board.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if cell == &None {
                    return Some(Index(i, j));
                }
            }
        }
        return None;
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board_string = String::new();
        for (i, row) in self.board.iter().enumerate() {
            if i % 3 == 0 {
                board_string.push_str("-------------------------\n");
            }
            for (j, cell) in row.iter().enumerate() {
                if j % 3 == 0 {
                    board_string.push_str("| ");
                }
                match cell {
                    Some(x) => board_string.push_str(&format!("{} ", x)),
                    None => board_string.push_str("  "),
                }
            }
            board_string.push_str("|\n");
        }
        board_string.push_str("-------------------------\n");
        write!(f, "{}", board_string)
    }
}

/// Solve the board by backtracking
pub fn solve(board: &mut Board) -> Option<Board> {
    if board.is_complete() {
        return Some(board.clone());
    }

    let next_empty = board.next_empty().unwrap_or_else(|| {
        panic!(" this should not happen because we checked completeness earlier")
    }); // we know this is not none because we checked in is_complete()

    let possible_entries = board.valid_entries(&next_empty);

    for (i, is_valid) in possible_entries.iter().enumerate() {
        if !is_valid {
            continue;
        }
        board.update_cell(&next_empty, (i + 1) as u8);
        if board.is_valid_entry(&next_empty) {
            if let Some(board) = solve(board) {
                return Some(board.clone());
            }
        }
    }
    board.update_cell(&next_empty, 0);
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_board() -> Board {
        Board::new(&[
            [0, 0, 3, 4, 0, 7, 0, 6, 0],
            [7, 0, 0, 0, 0, 0, 0, 4, 0],
            [0, 0, 0, 0, 1, 0, 2, 5, 0],
            [4, 8, 0, 3, 0, 0, 1, 0, 0],
            [0, 5, 0, 0, 0, 0, 0, 0, 2],
            [0, 6, 0, 0, 2, 0, 0, 0, 0],
            [0, 9, 0, 1, 0, 5, 0, 0, 8],
            [1, 0, 0, 6, 0, 0, 0, 0, 5],
            [0, 0, 0, 0, 0, 0, 4, 0, 0],
        ])
    }
    fn make_solved_board() -> Board {
        Board::new(&[
            [5, 2, 3, 4, 8, 7, 9, 6, 1],
            [7, 1, 9, 5, 6, 2, 8, 4, 3],
            [8, 4, 6, 9, 1, 3, 2, 5, 7],
            [4, 8, 2, 3, 5, 9, 1, 7, 6],
            [9, 5, 1, 7, 4, 6, 3, 8, 2],
            [3, 6, 7, 8, 2, 1, 5, 9, 4],
            [2, 9, 4, 1, 7, 5, 6, 3, 8],
            [1, 3, 8, 6, 9, 4, 7, 2, 5],
            [6, 7, 5, 2, 3, 8, 4, 1, 9],
        ])
    }

    #[test]
    fn test_get_cell() {
        let board = make_board();
        assert_eq!(board.get_cell(&Index(3, 6)), Some(1));
        assert_eq!(board.get_cell(&Index(0, 2)), Some(3));
        assert_eq!(board.get_cell(&Index(8, 8)), None);
    }

    #[test]
    fn test_get_row() {
        let board = make_board();
        assert_eq!(
            board.get_row(3),
            [
                Some(4),
                Some(8),
                None,
                Some(3),
                None,
                None,
                Some(1),
                None,
                None
            ]
        );
    }

    #[test]
    fn test_get_column() {
        let board = make_board();
        assert_eq!(
            board.get_column(5),
            [
                Some(7), //
                None,
                None,
                None,
                None,
                None,
                Some(5),
                None,
                None
            ]
        )
    }

    #[test]
    fn test_get_subgrid() {
        let board = make_board();

        assert_eq!(
            board.get_subgrid(&Index(5, 5)),
            [
                Some(3), //
                None,
                None,
                None,
                None,
                None,
                None,
                Some(2),
                None
            ]
        )
    }
    #[test]
    fn test_valid_entry() {
        let mut board = make_board();
        assert!(board.is_valid_entry(&Index(0, 2)));
        board.update_cell(&Index(0, 0), 3);
        assert!(!board.is_valid_entry(&Index(0, 0)));
    }

    #[test]
    fn test_valid_board() {
        let board = make_board();
        assert!(board.is_valid_board());
    }

    #[test]
    fn test_complete() {
        let complete = make_solved_board();
        assert!(complete.is_complete());
    }

    #[test]
    fn test_solve_mut() {
        let mut board = make_board();
        let solved_board = make_solved_board();
        let experimental_solution = solve(&mut board).unwrap_or_else(|| {
            panic!("this should not happen because we know the board is solvable")
        });

        assert_eq!(experimental_solution, solved_board);
    }
}
