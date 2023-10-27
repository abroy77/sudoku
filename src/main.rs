use std::env;
use std::path::Path;
use sudoku::board::{solve_mut, Board};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut board = Board::from_csv(Path::new(&args[1])).expect("Invalid Path");

    match solve_mut(&mut board) {
        Some(answer) => println!("{}", answer),
        None => println!("No solution found"),
    }
}
