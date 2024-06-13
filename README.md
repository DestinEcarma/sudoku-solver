# Sudoku Solver

This project is a Sudoku solver implemented in Rust, utilizing the backtracking algorithm. To enhance efficiency, the solver employs a bitboard representation for each digit within the domain `{1, 2, 3, 4, 5, 6, 7, 8, 9}`. Each bitboard is stored in an array, where advanced bitwise operations are used to check the safety of placing a digit in a square.

## Features

- **Backtracking Algorithm**: A recursive approach to solve the Sudoku puzzle by trying each digit and backtracking when a conflict is detected.
- **Bitboard Representation**: Efficiently stores and manipulates the presence of digits using bitwise operations.

## Usage

First, build the executable file or run it directly in release mode for optimal performance:

```sh
cargo build --release
```

or

```sh
cargo run --release
```

Finally, you can display all available options and commands by running the program with the `-h` or `--help` argument:

```sh
cargo run --release -- -h
```

Note: The solver expects a Sudoku puzzle in a string format. Each character in the string represents a cell in the Sudoku grid, where digits `1-9` denote filled cells and `.` represents empty cells.

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE.md) file for details.
