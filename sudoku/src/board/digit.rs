pub type Digit = usize;

pub trait Digits {
	const ONE: Digit = 0;
	const TWO: Digit = 1;
	const THREE: Digit = 2;
	const FOUR: Digit = 3;
	const FIVE: Digit = 4;
	const SIX: Digit = 5;
	const SEVEN: Digit = 6;
	const EIGHT: Digit = 7;
	const NINE: Digit = 8;
}

pub trait DigitConsts {
	const DIGIT_SIZE: usize = 9;
	const DIGIT_RANGE: std::ops::Range<usize> = 0..Self::DIGIT_SIZE;
}

impl Digits for Digit {}
impl DigitConsts for usize {}

pub trait GetDigit<T> {
	fn get_digit(value: T) -> usize;
}

impl GetDigit<char> for Digit {
	fn get_digit(value: char) -> usize {
		match value {
			'1' => Self::ONE,
			'2' => Self::TWO,
			'3' => Self::THREE,
			'4' => Self::FOUR,
			'5' => Self::FIVE,
			'6' => Self::SIX,
			'7' => Self::SEVEN,
			'8' => Self::EIGHT,
			'9' => Self::NINE,
			_ => panic!("Invalid digit: {value}"),
		}
	}
}
