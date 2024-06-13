use super::{digit::GetDigit, Board, Digit};

impl From<&str> for Board {
	fn from(value: &str) -> Self {
		let mut board = Board::default();
		let mut square = 0;

		for ch in value.chars() {
			match ch {
				'.' => square += 1,
				_ => {
					board.set_digit(Digit::get_digit(ch), square);
					square += 1;
				}
			}
		}

		board
	}
}
