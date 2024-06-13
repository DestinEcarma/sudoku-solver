use super::{
	bitboard::BitboardOccupied,
	file_rank::FileRankConsts,
	square::{GetSquare, SquareString},
	Board, Square,
};
use std::fmt;

impl fmt::Display for Board {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for rank in usize::FILE_RANK_RANGE {
			if rank % 3 == 0 {
				writeln!(f, "+-------+-------+-------+")?;
			}

			for file in usize::FILE_RANK_RANGE {
				if file % 3 == 0 {
					write!(f, "| ")?;
				}

				match self.get_digit(Square::get_square((file, rank))) {
					Some(digit) => write!(f, "{digit} ")?,
					None => write!(f, "  ")?,
				}
			}

			writeln!(f, "|")?;
		}

		write!(f, "+-------+-------+-------+")
	}
}

impl Board {
	pub fn display_squares() {
		for rank in usize::FILE_RANK_RANGE {
			if rank % 3 == 0 {
				println!("+----------+----------+----------+");
			}

			for file in usize::FILE_RANK_RANGE {
				if file % 3 == 0 {
					print!("| ");
				}

				print!("{} ", Square::get_square((file, rank)).square_string());
			}

			println!("|");
		}

		println!("+----------+----------+----------+");
	}
}

impl Board {
	fn get_digit(&self, square: Square) -> Option<u8> {
		for (digit, bitboard) in self.digits.iter().enumerate() {
			if bitboard.occupied(square) {
				return Some(digit as u8 + 1);
			}
		}

		None
	}
}
