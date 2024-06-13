use super::{
	file_rank::FileRankConsts,
	square::{SquareConsts, Squares},
	Square,
};

pub type Bitboard = u128;

pub trait BitboardFiles {
	#[rustfmt::skip]
	const FILES: [Bitboard; usize::FILE_RANK_SIZE] = [
		0x001008040201008040201, 0x002010080402010080402, 0x004020100804020100804,
		0x008040201008040201008, 0x010080402010080402010, 0x020100804020100804020,
		0x040201008040201008040, 0x080402010080402010080, 0x100804020100804020100,
	];
}

pub trait BitboardRanks {
	#[rustfmt::skip]
	const RANKS: [Bitboard; usize::FILE_RANK_SIZE] = [
		0x0000000000000000001ff, 0x00000000000000003fe00, 0x000000000000007fc0000,
		0x000000000000ff8000000, 0x0000000001ff000000000, 0x00000003fe00000000000,
		0x000007fc0000000000000, 0x000ff8000000000000000, 0x1ff000000000000000000,
	];
}

pub trait BitboardSubGrid {
	#[rustfmt::skip]
	const SUB_GRID: [Bitboard; usize::FILE_RANK_SIZE] = [
		0x0000000000000001c0e07, 0x000000000000000e07038, 0x0000000000000070381c0,
		0x000000000e07038000000, 0x0000000070381c0000000, 0x0000000381c0e00000000,
		0x0070381c0000000000000, 0x0381c0e00000000000000, 0x1c0e07000000000000000,
	];
}

pub trait BitboardSquares {
	#[rustfmt::skip]
	const SQUARES: [Bitboard; usize::SQUARE_SIZE] = [
		0x000000000000000000001, 0x000000000000000000002, 0x000000000000000000004, 0x000000000000000000008, 0x000000000000000000010, 0x000000000000000000020, 0x000000000000000000040, 0x000000000000000000080, 0x000000000000000000100,
		0x000000000000000000200, 0x000000000000000000400, 0x000000000000000000800, 0x000000000000000001000, 0x000000000000000002000, 0x000000000000000004000, 0x000000000000000008000, 0x000000000000000010000, 0x000000000000000020000,
		0x000000000000000040000, 0x000000000000000080000, 0x000000000000000100000, 0x000000000000000200000, 0x000000000000000400000, 0x000000000000000800000, 0x000000000000001000000, 0x000000000000002000000, 0x000000000000004000000,
		0x000000000000008000000, 0x000000000000010000000, 0x000000000000020000000, 0x000000000000040000000, 0x000000000000080000000, 0x000000000000100000000, 0x000000000000200000000, 0x000000000000400000000, 0x000000000000800000000,
		0x000000000001000000000, 0x000000000002000000000, 0x000000000004000000000, 0x000000000008000000000, 0x000000000010000000000, 0x000000000020000000000, 0x000000000040000000000, 0x000000000080000000000, 0x000000000100000000000,
		0x000000000200000000000, 0x000000000400000000000, 0x000000000800000000000, 0x000000001000000000000, 0x000000002000000000000, 0x000000004000000000000, 0x000000008000000000000, 0x000000010000000000000, 0x000000020000000000000,
		0x000000040000000000000, 0x000000080000000000000, 0x000000100000000000000, 0x000000200000000000000, 0x000000400000000000000, 0x000000800000000000000, 0x000001000000000000000, 0x000002000000000000000, 0x000004000000000000000,
		0x000008000000000000000, 0x000010000000000000000, 0x000020000000000000000, 0x000040000000000000000, 0x000080000000000000000, 0x000100000000000000000, 0x000200000000000000000, 0x000400000000000000000, 0x000800000000000000000,
		0x001000000000000000000, 0x002000000000000000000, 0x004000000000000000000, 0x008000000000000000000, 0x010000000000000000000, 0x020000000000000000000, 0x040000000000000000000, 0x080000000000000000000, 0x100000000000000000000,
	];
}

impl BitboardFiles for Bitboard {}
impl BitboardRanks for Bitboard {}
impl BitboardSubGrid for Bitboard {}
impl BitboardSquares for Bitboard {}

pub trait GetSubGrid {
	fn get_sub_grid(square: Square) -> Bitboard;
}

impl GetSubGrid for Bitboard {
	#[rustfmt::skip]
	fn get_sub_grid(square: Square) -> Bitboard {
		match square {
			Square::A1..=Square::C1 | Square::A2..=Square::C2 | Square::A3..=Square::C3 => Self::SUB_GRID[0],
			Square::D1..=Square::F1 | Square::D2..=Square::F2 | Square::D3..=Square::F3 => Self::SUB_GRID[1],
			Square::G1..=Square::I1 | Square::G2..=Square::I2 | Square::G3..=Square::I3 => Self::SUB_GRID[2],
			Square::A4..=Square::C4 | Square::A5..=Square::C5 | Square::A6..=Square::C6 => Self::SUB_GRID[3],
			Square::D4..=Square::F4 | Square::D5..=Square::F5 | Square::D6..=Square::F6 => Self::SUB_GRID[4],
			Square::G4..=Square::I4 | Square::G5..=Square::I5 | Square::G6..=Square::I6 => Self::SUB_GRID[5],
			Square::A7..=Square::C7 | Square::A8..=Square::C8 | Square::A9..=Square::C9 => Self::SUB_GRID[6],
			Square::D7..=Square::F7 | Square::D8..=Square::F8 | Square::D9..=Square::F9 => Self::SUB_GRID[7],
			Square::G7..=Square::I7 | Square::G8..=Square::I8 | Square::G9..=Square::I9 => Self::SUB_GRID[8],
			_ => panic!("Invalid Square: {square}"),
		}
	}
}

pub trait BitboardOccupied<T: std::ops::BitAnd> {
	fn occupied(&self, other: T) -> bool;
}

impl BitboardOccupied<Square> for Bitboard {
	fn occupied(&self, other: Square) -> bool {
		self & Self::SQUARES[other] > 0
	}
}

pub trait BitboardString {
	fn bitboard_string(&self) -> String;
}

impl BitboardString for Bitboard {
	fn bitboard_string(&self) -> String {
		let mut result = String::new();

		for square in usize::SQUARE_RANGE {
			if square % 9 == 0 {
				result.push('\n');
			}

			match self.occupied(square) {
				true => result.push_str("1 "),
				false => result.push_str("0 "),
			}
		}

		result
	}
}