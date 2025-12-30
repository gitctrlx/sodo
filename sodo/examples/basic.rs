use sodo::{Cell, Solver, Sudoku};

fn main() {
    // Create empty 9x9 sudoku
    let mut sudoku = Sudoku::new(9);

    // Set some values
    sudoku.set(0, 0, 5).unwrap();
    sudoku.set(0, 1, 3).unwrap();
    sudoku.set(0, 4, 7).unwrap();

    // Check validity
    println!("Valid: {}", sudoku.is_valid());
    println!("Complete: {}", sudoku.is_complete());

    // Get candidates for a cell
    let candidates = sudoku.candidates(0, 2);
    println!("Candidates at (0,2): {:?}", candidates);

    // Check cell state
    match sudoku.get(0, 0) {
        Some(Cell::Filled(v)) => println!("Cell (0,0) = {v}"),
        Some(Cell::Given(v)) => println!("Cell (0,0) = {v} (given)"),
        Some(Cell::Empty) => println!("Cell (0,0) is empty"),
        None => println!("Out of bounds"),
    }

    // Get hint
    let solver = Solver::new();
    if let Some((r, c, val)) = solver.hint(&sudoku) {
        println!("Hint: Place {val} at ({r}, {c})");
    }
}
