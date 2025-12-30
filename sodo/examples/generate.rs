use sodo::{Difficulty, Solver};

fn main() {
    let mut solver = Solver::new();

    for difficulty in [
        Difficulty::Easy,
        Difficulty::Medium,
        Difficulty::Hard,
        Difficulty::Expert,
    ] {
        let puzzle = solver.generate(9, difficulty).unwrap();
        println!("{difficulty:?}:\n{puzzle}");
    }
}
