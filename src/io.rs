// scripts to read csv files with a sudoku puzzle
use crate::board::Board;
use csv::ReaderBuilder;
use std::path::PathBuf;

impl Board {
    pub fn from_csv(path: &PathBuf) -> Result<Board, &'static str> {
        let mut board = [[0; 9]; 9];
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .from_path(path)
            .expect("Could not open csv file");

        let mut line_count = 0;
        for (i, row) in reader.records().enumerate() {
            let row = row.map_err(|_| "Unreadable csv")?;
            if row.len() != 9 {
                return Err("Invalid csv file. Only 9x9 boards allowed");
            }

            for (j, cell) in row.iter().enumerate() {
                let input_value = match cell.trim().parse::<u8>() {
                    Ok(value) => value,
                    Err(_) => return Err("Invalid csv file. Only int numbers allowed"),
                };
                if input_value > 9 {
                    return Err("Invalid csv file. Only numbers between 0 and 9 allowed");
                }

                board[i][j] = input_value;
            }
            line_count += 1;
        }
        if line_count != 9 {
            return Err("Invalid csv file. Only 9x9 boards allowed");
        }

        // check if the board is valid
        let board = Board::new(&board);
        if !board.is_valid_board() {
            return Err("Invalid Board: Board does not satisfy sudoku rules");
        }

        Ok(board)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_from_csv() {
        let board = Board::from_csv(&PathBuf::from("tests/test_board_pass.csv")).unwrap();
        assert_eq!(
            board,
            Board::new(&[
                [0, 7, 3, 8, 0, 4, 2, 1, 6], //
                [0, 0, 0, 2, 0, 9, 5, 0, 0], //
                [2, 8, 5, 6, 0, 3, 0, 9, 7],
                [0, 0, 0, 3, 0, 0, 0, 7, 4],
                [7, 5, 0, 0, 0, 0, 3, 0, 1],
                [0, 0, 4, 0, 2, 0, 0, 0, 0],
                [0, 9, 7, 5, 6, 0, 0, 0, 0],
                [0, 0, 0, 7, 0, 0, 1, 0, 0],
                [4, 2, 0, 0, 3, 0, 0, 6, 0]
            ])
        );
    }

    #[test]
    fn test_invalid_non_int() {
        let board = Board::from_csv(&PathBuf::from("tests/test_invalid_non_int.csv"));
        assert_eq!(board, Err("Invalid csv file. Only int numbers allowed"));
    }
    #[test]
    fn test_invalid_8_lines() {
        let board = Board::from_csv(&PathBuf::from("tests/test_invalid_8_lines.csv"));
        assert_eq!(board, Err("Invalid csv file. Only 9x9 boards allowed"));
    }

    #[test]
    fn test_invalid_column() {
        let board = Board::from_csv(&PathBuf::from("tests/test_invalid_column.csv"));
        assert_eq!(board, Err("Invalid csv file. Only 9x9 boards allowed"));
    }

    #[test]
    fn test_invalid_sudoku() {
        let board = Board::from_csv(&PathBuf::from("tests/test_board_invalid_sudoku.csv"));
        assert_eq!(
            board,
            Err("Invalid Board: Board does not satisfy sudoku rules")
        );
    }
}
