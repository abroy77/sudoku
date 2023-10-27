extern crate sudoku_solver;
use std::path::PathBuf;
use sudoku_solver::board::{solve, Board};
#[test]
fn test_pass() {
    let mut board = Board::from_csv(&PathBuf::from("tests/test_board_pass.csv")).unwrap();
    solve(&mut board);
    assert_eq!(
        board,
        Board::new(&[
            [9, 7, 3, 8, 5, 4, 2, 1, 6],
            [1, 4, 6, 2, 7, 9, 5, 8, 3],
            [2, 8, 5, 6, 1, 3, 4, 9, 7],
            [8, 1, 2, 3, 9, 5, 6, 7, 4],
            [7, 5, 9, 4, 8, 6, 3, 2, 1],
            [6, 3, 4, 1, 2, 7, 9, 5, 8],
            [3, 9, 7, 5, 6, 1, 8, 4, 2],
            [5, 6, 8, 7, 4, 2, 1, 3, 9],
            [4, 2, 1, 9, 3, 8, 7, 6, 5]
        ])
    )
}
