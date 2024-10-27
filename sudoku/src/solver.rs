use crate::board::{
	digit::DigitConsts,
	file_rank::{Files, Ranks},
	square::{GetSquare, SquareConsts, Squares},
	Board, Digit, File, Rank, Square,
};

pub struct Solver;

impl Solver {
	pub fn solve(board: &mut Board) -> (bool, usize) {
		let mut attempts = 0;

		(Self::solve_driver(board, &mut attempts, 0, 0), attempts)
	}

	fn solve_driver(
		board: &mut Board,
		attempts: &mut usize,
		mut rank: Rank,
		mut file: File,
	) -> bool {
		if rank == Rank::R9 && file > File::I {
			return true;
		}

		if file > File::I {
			rank += 1;
			file = File::A;
		};

		let square = Square::get_square((file, rank));

		if !board.square_is_empty(square) {
			return Self::solve_driver(board, attempts, rank, file + 1);
		}

		for digit in usize::DIGIT_RANGE {
			if board.square_is_safe(digit, square) {
				board.set_digit(digit, square);
				*attempts += 1;

				if Self::solve_driver(board, attempts, rank, file + 1) {
					return true;
				}

				board.unset_digit(digit, square);
			}
		}

		false
	}

	pub fn heuristic_search(board: &mut Board) -> (bool, usize) {
		let mut attempts = 0;

		Self::_heuristic_search(board, &mut attempts);

		(board.is_solved(), attempts)
	}

	fn _heuristic_search(board: &mut Board, attempts: &mut usize) {
		let mut heuristic: Option<Vec<Digit>> = None;
		let mut heuristic_count = usize::MAX;
		let mut heuristic_square = Square::A1;

		for square in usize::SQUARE_RANGE {
			if !board.square_is_empty(square) {
				continue;
			}

			let mut digits: Vec<Digit> = Default::default();
			let mut digits_count = 0;

			for digit in usize::DIGIT_RANGE {
				if board.square_is_safe(digit, square) {
					digits.push(digit);
					digits_count += 1;
				}
			}

			if digits_count > 0 && digits_count < heuristic_count {
				heuristic = Some(digits);
				heuristic_count = digits_count;
				heuristic_square = square;
			}
		}

		if let Some(heuristic) = heuristic {
			for digit in heuristic {
				board.set_digit(digit, heuristic_square);
				*attempts += 1;

				Self::_heuristic_search(board, attempts);

				if board.is_solved() {
					break;
				}

				board.unset_digit(digit, heuristic_square);
			}
		}
	}
}
