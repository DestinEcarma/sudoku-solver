use crate::board::{
	digit::DigitConsts,
	file_rank::{Files, Ranks},
	square::GetSquare,
	Board, File, Rank, Square,
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
}
