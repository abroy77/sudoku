# sudoku

A simple backtracking algorithm for solving sudoku puzzles written in rust.


## Usage

` cargo run --release -- "csv_path" `
where csv_path is the path to a csv file containing a sudoku puzzle.
eg: 
```
0,7,3,8,0,4,2,1,6
0,0,0,2,0,9,5,0,0
2,8,5,6,0,3,0,9,7
0,0,0,3,0,0,0,7,4
7,5,0,0,0,0,3,0,1
0,0,4,0,2,0,0,0,0
0,9,7,5,6,0,0,0,0
0,0,0,7,0,0,1,0,0
4,2,0,0,3,0,0,6,0
```

## Features

#### 1 - Stack only
All datastructures exist on the stack. No hashmaps or hashsets(which use heap memory)
are used. This is to increase speed.

#### 2 - Minimizing cloning
mutating a single board in memory instead of writing and storing cloned boards during backtracking.

#### 3 - csv deserialization
One can use a csv file to represent a sudoku puzzle. The csv file must be a 9x9 grid of numbers. Empty cells are represented by 0s.

#### 4 - pretty printing
the completed sudoku puzzle is pretty printed to the terminal.

#### 5 - Error Handling
The program will panic if the csv file is not a 9x9 grid of numbers. It will also panic if the puzzle is unsolvable. It will print helpful error messages

#### 6 - Docstrings
Added some docstrings and examples to the code!