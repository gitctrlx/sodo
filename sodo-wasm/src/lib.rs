use serde::{Deserialize, Serialize};
use sodo::{Difficulty as SodoDifficulty, Solver, Sudoku};
use wasm_bindgen::prelude::*;

/// 9x9 grid: `number[][]` where 0 = empty, 1-9 = filled.
pub type Grid = Vec<Vec<u8>>;

/// Puzzle difficulty level.
#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    Easy = 0,
    Medium = 1,
    Hard = 2,
    Expert = 3,
}

/// Generation result containing puzzle and solution.
#[derive(Serialize, Deserialize)]
pub struct SudokuResult {
    pub puzzle: Grid,
    pub solution: Grid,
}

/// Generates a new puzzle with the specified difficulty.
/// @returns `{ puzzle: Grid, solution: Grid }`
#[wasm_bindgen(js_name = "generateSudoku")]
pub fn generate_sudoku(difficulty: Option<Difficulty>) -> Result<JsValue, String> {
    let diff: SodoDifficulty = difficulty.unwrap_or(Difficulty::Medium).into();
    let mut solver = Solver::new();
    let puzzle = solver.generate(9, diff)?;
    let solution = solver.solve(puzzle.clone())?;

    let result = SudokuResult {
        puzzle: to_grid(&puzzle),
        solution: to_grid(&solution),
    };
    serde_wasm_bindgen::to_value(&result).map_err(|e| e.to_string())
}

/// Solves a puzzle grid.
/// @returns Solution grid.
#[wasm_bindgen(js_name = "solveGrid")]
pub fn solve_grid(grid: JsValue) -> Result<JsValue, String> {
    let g = parse_grid(grid)?;
    let sudoku = from_grid(&g)?;
    let mut solver = Solver::new();
    let solution = solver.solve(sudoku)?;
    serde_wasm_bindgen::to_value(&to_grid(&solution)).map_err(|e| e.to_string())
}

/// Validates that a solution correctly solves a puzzle.
#[wasm_bindgen(js_name = "validateSolution")]
pub fn validate_solution(puzzle: JsValue, solution: JsValue) -> Result<bool, String> {
    let p = parse_grid(puzzle)?;
    let s = parse_grid(solution)?;

    // Check puzzle givens are preserved
    for row in 0..9 {
        for col in 0..9 {
            if p[row][col] != 0 && p[row][col] != s[row][col] {
                return Ok(false);
            }
        }
    }

    // Validate all rows, columns, boxes
    for i in 0..9 {
        let mut row_seen = [false; 10];
        let mut col_seen = [false; 10];
        for j in 0..9 {
            let row_val = s[i][j];
            let col_val = s[j][i];
            if row_val < 1 || row_val > 9 || row_seen[row_val as usize] {
                return Ok(false);
            }
            if col_seen[col_val as usize] {
                return Ok(false);
            }
            row_seen[row_val as usize] = true;
            col_seen[col_val as usize] = true;
        }
    }

    for br in 0..3 {
        for bc in 0..3 {
            let mut seen = [false; 10];
            for i in 0..3 {
                for j in 0..3 {
                    let v = s[br * 3 + i][bc * 3 + j];
                    if seen[v as usize] {
                        return Ok(false);
                    }
                    seen[v as usize] = true;
                }
            }
        }
    }

    Ok(true)
}

/// Validates a grid for constraint violations (partial puzzle check).
#[wasm_bindgen(js_name = "validateGrid")]
pub fn validate_grid(grid: JsValue) -> Result<bool, String> {
    let g = parse_grid(grid)?;
    Ok(from_grid(&g)?.is_valid())
}

/// Checks if a grid puzzle is solvable.
#[wasm_bindgen(js_name = "isSolvable")]
pub fn is_solvable_grid(grid: JsValue) -> Result<bool, String> {
    let g = parse_grid(grid)?;
    let sudoku = from_grid(&g)?;
    let mut solver = Solver::new();
    Ok(solver.solve(sudoku).is_ok())
}

/// Gets a hint for the next logical move.
/// @returns `{ row: number, col: number, value: number }` or `null`
#[wasm_bindgen(js_name = "getHint")]
pub fn get_hint(grid: JsValue) -> Result<JsValue, String> {
    let g = parse_grid(grid)?;
    let sudoku = from_grid(&g)?;
    match Solver::new().hint(&sudoku) {
        Some((r, c, v)) => Ok(make_hint_obj(r, c, v)),
        None => Ok(JsValue::NULL),
    }
}

/// Formats a grid as human-readable string with box separators.
#[wasm_bindgen(js_name = "formatGrid")]
pub fn format_grid(grid: JsValue) -> Result<String, String> {
    let g = parse_grid(grid)?;
    Ok(from_grid(&g)?.to_string())
}

/// Creates an empty 9x9 grid filled with zeros.
#[wasm_bindgen(js_name = "createEmptyGrid")]
pub fn create_empty_grid() -> JsValue {
    serde_wasm_bindgen::to_value(&vec![vec![0u8; 9]; 9]).unwrap()
}

/// Deep clones a grid.
#[wasm_bindgen(js_name = "cloneGrid")]
pub fn clone_grid(grid: JsValue) -> Result<JsValue, String> {
    let g = parse_grid(grid)?;
    serde_wasm_bindgen::to_value(&g).map_err(|e| e.to_string())
}

/// Converts grid to JSON string.
#[wasm_bindgen(js_name = "gridToJson")]
pub fn grid_to_json(grid: JsValue) -> Result<String, String> {
    let g = parse_grid(grid)?;
    serde_json::to_string_pretty(&g).map_err(|e| e.to_string())
}

/// Parses JSON string to grid. Returns `null` if invalid.
#[wasm_bindgen(js_name = "jsonToGrid")]
pub fn json_to_grid(json: &str) -> JsValue {
    serde_json::from_str::<Grid>(json)
        .ok()
        .filter(|g| check_grid_format(g).is_ok())
        .and_then(|g| serde_wasm_bindgen::to_value(&g).ok())
        .unwrap_or(JsValue::NULL)
}

/// Converts compact string (81 chars) to grid.
#[wasm_bindgen(js_name = "parseGrid")]
pub fn string_to_grid(s: &str) -> Result<JsValue, String> {
    let sudoku = Sudoku::from_string(s, 9)?;
    serde_wasm_bindgen::to_value(&to_grid(&sudoku)).map_err(|e| e.to_string())
}

/// Converts grid to compact string (81 chars).
#[wasm_bindgen(js_name = "stringifyGrid")]
pub fn grid_to_string(grid: JsValue) -> Result<String, String> {
    let g = parse_grid(grid)?;
    Ok(from_grid(&g)?.to_string_compact())
}

/// Generates puzzle as compact string.
#[wasm_bindgen]
pub fn generate(difficulty: Option<String>, size: Option<usize>) -> Result<String, String> {
    let size = size.unwrap_or(9);
    let diff: SodoDifficulty = parse_difficulty(difficulty.as_deref())?.into();
    let mut solver = Solver::new();
    solver.generate(size, diff).map(|s| s.to_string_compact())
}

/// Solves puzzle from compact string.
#[wasm_bindgen]
pub fn solve(puzzle: &str, size: Option<usize>) -> Result<String, String> {
    let size = size.unwrap_or(9);
    let sudoku = Sudoku::from_string(puzzle, size)?;
    let mut solver = Solver::new();
    solver.solve(sudoku).map(|s| s.to_string_compact())
}

/// Validates puzzle string for constraint violations.
#[wasm_bindgen]
pub fn validate(puzzle: &str, size: Option<usize>) -> Result<bool, String> {
    let size = size.unwrap_or(9);
    Ok(Sudoku::from_string(puzzle, size)?.is_valid())
}

/// Gets hint from puzzle string.
/// @returns `{ row, col, value }` or `null`
#[wasm_bindgen]
pub fn hint(puzzle: &str, size: Option<usize>) -> Result<JsValue, String> {
    let size = size.unwrap_or(9);
    let sudoku = Sudoku::from_string(puzzle, size)?;
    match Solver::new().hint(&sudoku) {
        Some((r, c, v)) => Ok(make_hint_obj(r, c, v)),
        None => Ok(JsValue::NULL),
    }
}

/// Formats puzzle string as human-readable grid.
#[wasm_bindgen]
pub fn format(puzzle: &str, size: Option<usize>) -> Result<String, String> {
    let size = size.unwrap_or(9);
    Ok(Sudoku::from_string(puzzle, size)?.to_string())
}

impl From<Difficulty> for SodoDifficulty {
    fn from(d: Difficulty) -> Self {
        match d {
            Difficulty::Easy => SodoDifficulty::Easy,
            Difficulty::Medium => SodoDifficulty::Medium,
            Difficulty::Hard => SodoDifficulty::Hard,
            Difficulty::Expert => SodoDifficulty::Expert,
        }
    }
}

fn parse_difficulty(s: Option<&str>) -> Result<Difficulty, String> {
    match s.unwrap_or("medium") {
        "easy" => Ok(Difficulty::Easy),
        "medium" => Ok(Difficulty::Medium),
        "hard" => Ok(Difficulty::Hard),
        "expert" => Ok(Difficulty::Expert),
        other => Err(format!("Invalid difficulty: {other}")),
    }
}

fn parse_grid(js: JsValue) -> Result<Grid, String> {
    let g: Grid = serde_wasm_bindgen::from_value(js).map_err(|e| e.to_string())?;
    check_grid_format(&g)?;
    Ok(g)
}

fn check_grid_format(grid: &Grid) -> Result<(), String> {
    if grid.len() != 9 {
        return Err(format!("Expected 9 rows, got {}", grid.len()));
    }
    for (i, row) in grid.iter().enumerate() {
        if row.len() != 9 {
            return Err(format!("Row {i}: expected 9 cols, got {}", row.len()));
        }
        if row.iter().any(|&v| v > 9) {
            return Err(format!("Row {i}: values must be 0-9"));
        }
    }
    Ok(())
}

fn to_grid(sudoku: &Sudoku) -> Grid {
    sudoku
        .grid
        .iter()
        .map(|row| row.iter().map(|c| c.value().unwrap_or(0)).collect())
        .collect()
}

fn from_grid(grid: &Grid) -> Result<Sudoku, String> {
    let s: String = grid
        .iter()
        .flatten()
        .map(|&v| if v == 0 { '.' } else { (b'0' + v) as char })
        .collect();
    Sudoku::from_string(&s, 9)
}

fn make_hint_obj(row: usize, col: usize, value: u8) -> JsValue {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &"row".into(), &(row as u32).into()).unwrap();
    js_sys::Reflect::set(&obj, &"col".into(), &(col as u32).into()).unwrap();
    js_sys::Reflect::set(&obj, &"value".into(), &(value as u32).into()).unwrap();
    obj.into()
}
