use sodo::{Solver, Sudoku};

fn main() {
    let puzzle = Sudoku::from_string(
        "530070000600195000098000060800060003400803001700020006060000280000419005000080079",
        9,
    )
    .unwrap();

    println!("Puzzle:\n{puzzle}");

    let mut solver = Solver::new();
    let (solution, stats) = solver.solve_with_stats(puzzle).unwrap();

    println!("Solution:\n{solution}");
    println!(
        "Stats: {} iterations, {} backtracks",
        stats.iterations, stats.backtracks
    );
}
