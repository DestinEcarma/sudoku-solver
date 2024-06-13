pub type File = usize;
pub type Rank = usize;

pub trait Files {
	const A: File = 0;
	const B: File = 1;
	const C: File = 2;
	const D: File = 3;
	const E: File = 4;
	const F: File = 5;
	const G: File = 6;
	const H: File = 7;
	const I: File = 8;
}

pub trait Ranks {
	const R1: Rank = 0;
	const R2: Rank = 1;
	const R3: Rank = 2;
	const R4: Rank = 3;
	const R5: Rank = 4;
	const R6: Rank = 5;
	const R7: Rank = 6;
	const R8: Rank = 7;
	const R9: Rank = 8;
}

pub trait FileRankConsts {
	const FILE_RANK_SIZE: usize = 9;
	const FILE_RANK_RANGE: std::ops::Range<usize> = 0..Self::FILE_RANK_SIZE;
}

impl Files for File {}
impl Ranks for Rank {}
impl FileRankConsts for usize {}

pub trait FileString {
	fn file_string(&self) -> String;
}

impl FileString for File {
	fn file_string(&self) -> String {
		String::from(match *self {
			Self::A => 'A',
			Self::B => 'B',
			Self::C => 'C',
			Self::D => 'D',
			Self::E => 'E',
			Self::F => 'F',
			Self::G => 'G',
			Self::H => 'H',
			Self::I => 'I',
			_ => panic!("Invalid file: {self}"),
		})
	}
}

pub trait RankString {
	fn rank_string(&self) -> String;
}

impl RankString for Rank {
	fn rank_string(&self) -> String {
		String::from(match *self {
			Self::R1 => '1',
			Self::R2 => '2',
			Self::R3 => '3',
			Self::R4 => '4',
			Self::R5 => '5',
			Self::R6 => '6',
			Self::R7 => '7',
			Self::R8 => '8',
			Self::R9 => '9',
			_ => panic!("Invalid rank: {self}"),
		})
	}
}
