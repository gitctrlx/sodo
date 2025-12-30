//! Sudoku solver and generator library.
//!
//! # Example
//!
//! ```
//! use sodo::{Sudoku, Solver};
//!
//! let puzzle = Sudoku::from_string(
//!     "530070000600195000098000060800060003400803001700020006060000280000419005000080079",
//!     9
//! ).unwrap();
//!
//! let mut solver = Solver::new();
//! let solution = solver.solve(puzzle).unwrap();
//! assert!(solution.is_solved());
//! ```

mod sodo;
mod solver;
mod strategy;

pub use sodo::{Cell, Sudoku};
pub use solver::{Difficulty, Solver, Stats};
pub use strategy::{Strategy, all as all_strategies};
