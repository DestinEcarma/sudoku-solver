mod impls;

pub mod bitboard;
pub mod digit;
pub mod file_rank;
pub mod square;

pub use bitboard::Bitboard;
pub use digit::Digit;
pub use file_rank::{File, Rank};
pub use square::Square;

use bitboard::{BitboardFiles, BitboardOccupied, BitboardRanks, BitboardSquares, GetSubGrid};
use digit::DigitConsts;
use square::SquareLocation;

#[derive(Debug, Default)]
pub struct Board {
	pub digits: [Bitboard; usize::DIGIT_SIZE],
}

impl Board {
	pub fn add_digit(&mut self, digit: usize, square: Square) {
		for digit in usize::DIGIT_RANGE {
			self.digits[digit] &= !Bitboard::SQUARES[square];
		}

		self.square_is_safe(digit, square);

		self.digits[digit] |= Bitboard::SQUARES[square];
	}
}

impl Board {
	pub(crate) fn square_is_safe(&self, digit: usize, square: Square) -> bool {
		let (file, rank) = square.location();

		(Bitboard::FILES[file] | Bitboard::RANKS[rank] | Bitboard::get_sub_grid(square))
			& self.digits[digit]
			== 0
	}

	pub(crate) fn square_is_empty(&self, square: Square) -> bool {
		for bitboard in self.digits.iter() {
			if bitboard.occupied(square) {
				return false;
			}
		}

		true
	}

	pub(crate) fn set_digit(&mut self, digit: usize, square: Square) {
		self.digits[digit] |= Bitboard::SQUARES[square];
	}

	pub fn unset_digit(&mut self, digit: usize, square: Square) {
		self.digits[digit] &= !Bitboard::SQUARES[square];
	}

	pub fn is_solved(&self) -> bool {
		for bitboard in self.digits.iter() {
			if bitboard.count_ones() != usize::DIGIT_SIZE as u32 {
				return false;
			}
		}

		true
	}
}
