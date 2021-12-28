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
pub(crate) fn to_decimal<V: AsRef<[bool]>>(binary_digits_msbf: V) -> usize {
	let binary_digits_msbf = binary_digits_msbf.as_ref();
	let n_digits = binary_digits_msbf.len() as u32;
	if n_digits == 0 {
		return 0;
	}

	(0..n_digits)
		.zip(binary_digits_msbf.iter().rev())
		.map(|(pow2, &is_on)| (is_on as usize) * 2usize.pow(pow2))
		.reduce(|a, b| a + b)
		.unwrap_or(0)
}
// end::code[]
