use sodo::{Difficulty, Solver, Sudoku};
use wasm_bindgen::prelude::*;

/// Solves a Sudoku puzzle.
///
/// # Arguments
/// * `puzzle` - 81-char string (use '.' or '0' for empty cells)
/// * `size` - Grid size (default: 9)
///
/// # Returns
/// Solution as 81-char string, or error message.
#[wasm_bindgen]
pub fn solve(puzzle: &str, size: Option<usize>) -> Result<String, String> {
    let size = size.unwrap_or(9);
    let sudoku = Sudoku::from_string(puzzle, size)?;
    let mut solver = Solver::new();
    solver.solve(sudoku).map(|s| s.to_string_compact())
}

/// Generates a new Sudoku puzzle.
///
/// # Arguments
/// * `difficulty` - "easy", "medium", "hard", or "expert"
/// * `size` - Grid size (default: 9)
///
/// # Returns
/// Puzzle as 81-char string.
#[wasm_bindgen]
pub fn generate(difficulty: Option<String>, size: Option<usize>) -> Result<String, String> {
    let size = size.unwrap_or(9);
    let diff = match difficulty.as_deref().unwrap_or("medium") {
        "easy" => Difficulty::Easy,
        "medium" => Difficulty::Medium,
        "hard" => Difficulty::Hard,
        "expert" => Difficulty::Expert,
        other => return Err(format!("Invalid difficulty: {other}")),
    };
    let mut solver = Solver::new();
    solver.generate(size, diff).map(|s| s.to_string_compact())
}

/// Validates a Sudoku puzzle.
///
/// # Returns
/// `true` if puzzle has no constraint violations.
#[wasm_bindgen]
pub fn validate(puzzle: &str, size: Option<usize>) -> Result<bool, String> {
    let size = size.unwrap_or(9);
    let sudoku = Sudoku::from_string(puzzle, size)?;
    Ok(sudoku.is_valid())
}

/// Checks if a puzzle is solvable.
#[wasm_bindgen]
pub fn is_solvable(puzzle: &str, size: Option<usize>) -> Result<bool, String> {
    let size = size.unwrap_or(9);
    let sudoku = Sudoku::from_string(puzzle, size)?;
    let mut solver = Solver::new();
    Ok(solver.solve(sudoku).is_ok())
}

/// Gets a hint for the next move.
///
/// # Returns
/// JSON object `{row, col, value}` (1-indexed), or null if no hint.
#[wasm_bindgen]
pub fn hint(puzzle: &str, size: Option<usize>) -> Result<JsValue, String> {
    let size = size.unwrap_or(9);
    let sudoku = Sudoku::from_string(puzzle, size)?;
    let solver = Solver::new();

    match solver.hint(&sudoku) {
        Some((r, c, v)) => {
            let obj = js_sys::Object::new();
            js_sys::Reflect::set(&obj, &"row".into(), &(r as u32 + 1).into()).unwrap();
            js_sys::Reflect::set(&obj, &"col".into(), &(c as u32 + 1).into()).unwrap();
            js_sys::Reflect::set(&obj, &"value".into(), &(v as u32).into()).unwrap();
            Ok(obj.into())
        }
        None => Ok(JsValue::NULL),
    }
}

/// Formats a puzzle string as a readable grid.
#[wasm_bindgen]
pub fn format(puzzle: &str, size: Option<usize>) -> Result<String, String> {
    let size = size.unwrap_or(9);
    let sudoku = Sudoku::from_string(puzzle, size)?;
    Ok(sudoku.to_string())
}
