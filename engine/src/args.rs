use clap::Parser;
use sudoku::{board::Board, solver::Solver};

#[derive(Parser)]
pub struct Args {
	/// The board state to solve
	///
	/// where each character is from 1-9, where blank spaces are represented as '.'
	///
	/// Example: "8..........36......7..9.2...5...7.......457.....1...3...1....68..85...1..9....4"
	pub input: String,
	/// Display the time taken to solve the board
	#[arg(short, long)]
	pub time: bool,
	/// Display the number of iterations taken to solve the board
	#[arg(short, long)]
	pub attempt: bool,
	/// Use heuristic search to solve the board
	#[arg(short = 'H', long)]
	pub heuristic: bool,
}

impl Args {
	pub fn solve(self) {
		let mut board = Board::from(self.input.as_str());

		println!("Initial State:\n{board}\n");

		let start = std::time::Instant::now();

		let (solved, attemtps) = match self.heuristic {
			true => Solver::heuristic_search(&mut board),
			false => Solver::solve(&mut board),
		};

		let elapsed = start.elapsed().as_micros();

		match solved {
			false => println!("No solution found"),
			true => {
				println!("Solved State:\n{board}");

				if self.time {
					println!("\nTime (μs): {:?}", elapsed);
				}

				if self.attempt {
					println!("Attempts: {}", attemtps);
				}
			}
		}
	}
}
