use std::fmt::{Debug, Display};
// tag::mods[]
pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_12;
pub mod day_13;
pub mod day_14;
// end::mods[]

#[derive(Debug)]
pub struct Answer<T1, T2> {
	pt1: T1,
	pt2: T2,
}

impl<T1, T2> Answer<T1, T2> {
	fn new(pt1: T1, pt2: T2) -> Self {
		Self { pt1, pt2 }
	}
}

impl<T1: Debug, T2: Debug> Display for Answer<T1, T2> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Part 1: {:?} ; Part 2: {:?}", self.pt1, self.pt2)
	}
}

impl<T1, T2> From<(T1, T2)> for Answer<T1, T2> {
	fn from(other: (T1, T2)) -> Self {
		let (pt1, pt2) = other;
		Self::new(pt1, pt2)
	}
}

pub(crate) fn to_decimal<V: AsRef<[bool]>>(binary_digits_msbf: V) -> usize {
	let binary_digits_msbf = binary_digits_msbf.as_ref();
	let n_digits = binary_digits_msbf.len();
	let pow2s = num::range_step_inclusive((n_digits - 1) as i32, 0, -1);
	pow2s
		.zip(binary_digits_msbf)
		.map(|(pow2, is_on)| {
			let is_on: usize = (*is_on).into();
			is_on * 2usize.pow(pow2 as u32)
		})
		.reduce(|a, b| a + b)
		.unwrap_or(0) as usize
}
