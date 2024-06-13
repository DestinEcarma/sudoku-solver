use super::{
	file_rank::{FileRankConsts, FileString, RankString},
	File, Rank,
};

pub type Square = usize;

pub trait Squares {
	const A1: Square = 0;
	const B1: Square = 1;
	const C1: Square = 2;
	const D1: Square = 3;
	const E1: Square = 4;
	const F1: Square = 5;
	const G1: Square = 6;
	const H1: Square = 7;
	const I1: Square = 8;
	const A2: Square = 9;
	const B2: Square = 10;
	const C2: Square = 11;
	const D2: Square = 12;
	const E2: Square = 13;
	const F2: Square = 14;
	const G2: Square = 15;
	const H2: Square = 16;
	const I2: Square = 17;
	const A3: Square = 18;
	const B3: Square = 19;
	const C3: Square = 20;
	const D3: Square = 21;
	const E3: Square = 22;
	const F3: Square = 23;
	const G3: Square = 24;
	const H3: Square = 25;
	const I3: Square = 26;
	const A4: Square = 27;
	const B4: Square = 28;
	const C4: Square = 29;
	const D4: Square = 30;
	const E4: Square = 31;
	const F4: Square = 32;
	const G4: Square = 33;
	const H4: Square = 34;
	const I4: Square = 35;
	const A5: Square = 36;
	const B5: Square = 37;
	const C5: Square = 38;
	const D5: Square = 39;
	const E5: Square = 40;
	const F5: Square = 41;
	const G5: Square = 42;
	const H5: Square = 43;
	const I5: Square = 44;
	const A6: Square = 45;
	const B6: Square = 46;
	const C6: Square = 47;
	const D6: Square = 48;
	const E6: Square = 49;
	const F6: Square = 50;
	const G6: Square = 51;
	const H6: Square = 52;
	const I6: Square = 53;
	const A7: Square = 54;
	const B7: Square = 55;
	const C7: Square = 56;
	const D7: Square = 57;
	const E7: Square = 58;
	const F7: Square = 59;
	const G7: Square = 60;
	const H7: Square = 61;
	const I7: Square = 62;
	const A8: Square = 63;
	const B8: Square = 64;
	const C8: Square = 65;
	const D8: Square = 66;
	const E8: Square = 67;
	const F8: Square = 68;
	const G8: Square = 69;
	const H8: Square = 70;
	const I8: Square = 71;
	const A9: Square = 72;
	const B9: Square = 73;
	const C9: Square = 74;
	const D9: Square = 75;
	const E9: Square = 76;
	const F9: Square = 77;
	const G9: Square = 78;
	const H9: Square = 79;
	const I9: Square = 80;
}

pub trait SquareConsts {
	const SQUARE_SIZE: usize = 81;
	const SQUARE_RANGE: std::ops::Range<usize> = 0..Self::SQUARE_SIZE;
}

impl Squares for Square {}
impl SquareConsts for Square {}

pub trait GetSquare<T> {
	fn get_square(value: T) -> Square;
}

impl GetSquare<(File, Rank)> for Square {
	fn get_square((file, rank): (File, Rank)) -> Square {
		(rank * usize::FILE_RANK_SIZE + file) as Square
	}
}

pub trait SquareLocation {
	fn location(&self) -> (File, Rank);
}

impl SquareLocation for Square {
	fn location(&self) -> (File, Rank) {
		(*self % usize::FILE_RANK_SIZE, *self / usize::FILE_RANK_SIZE)
	}
}

pub trait SquareString {
	fn square_string(&self) -> String;
}

impl SquareString for Square {
	fn square_string(&self) -> String {
		let rank = *self / usize::FILE_RANK_SIZE;
		let file = *self % usize::FILE_RANK_SIZE;

		format!("{}{}", file.file_string(), rank.rank_string())
	}
}
