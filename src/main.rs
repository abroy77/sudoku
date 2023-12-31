use clap::Parser;
use std::path::PathBuf;
use sudoku_solver_by_roy::board::{solve, Board};

#[derive(Parser, Debug)]
#[command(author,version,about,long_about=None)]
struct Args {
    #[arg()]
    csv_path: PathBuf,
}

fn main() {
    let args = Args::parse();
    let csv_path = args.csv_path;

    let mut board = match Board::from_csv(&csv_path) {
        Ok(board) => board,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    match solve(&mut board) {
        Some(answer) => println!("{}", answer),
        None => println!("No solution found"),
    }
}
