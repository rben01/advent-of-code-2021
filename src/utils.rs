#![allow(dead_code)]

#[macro_export]
macro_rules! test_input {
	($input:expr, day: $day:expr, ans: ($pt1:expr, $pt2:expr)) => {
		assert_eq!(ans_for_input($input), ($day, ($pt1, $pt2)).into());
	};
	($input:expr, pt1: $pt1:expr) => {
		assert_eq!(pt1($input), $pt1);
	};
	($input:expr, pt2: $pt2:expr) => {
		assert_eq!(pt2($input), $pt2);
	};
}

// tag::code[]
pub(crate) fn to_decimal<V: AsRef<[bool]>>(binary_digits_msbf: V) -> u32 {
	binary_digits_msbf
		.as_ref()
		.iter()
		.rev()
		.enumerate()
		.map(|(pow2, &is_on)| u32::from(is_on) * 2u32.pow(u32::try_from(pow2).unwrap()))
		.reduce(|a, b| a + b)
		.unwrap_or(0)
}

pub(crate) fn abs_diff(a: usize, b: usize) -> usize {
	if a > b {
		a - b
	} else {
		b - a
	}
}
// end::code[]
