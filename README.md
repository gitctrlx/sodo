# sodo

Fast Sudoku solver and generator in Rust.

## Installation

```bash
cargo install sodo
```

## Usage

```bash
# Generate a puzzle
sodo g -d hard

# Solve a puzzle
sodo s 530070000600195000098000060800060003400803001700020006060000280000419005000080079

# Get a hint
sodo h <puzzle>

# Validate
sodo v <puzzle>
```

## Library

```rust
use sodo::{Sudoku, Solver};

let puzzle = Sudoku::from_string(
    "530070000600195000098000060800060003400803001700020006060000280000419005000080079",
    9
).unwrap();

let mut solver = Solver::new();
let solution = solver.solve(puzzle).unwrap();
println!("{}", solution);
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in kand by you, as defined in the Apache-2.0 license, shall be dually licensed as above, without any additional terms or conditions.
