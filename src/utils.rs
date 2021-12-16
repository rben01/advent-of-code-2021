pub(crate) fn to_decimal<V: AsRef<[bool]>>(binary_digits_msbf: V) -> usize {
	let binary_digits_msbf = binary_digits_msbf.as_ref();
	let n_digits = binary_digits_msbf.len() as u32;
	if n_digits == 0 {
		return 0;
	}

	let pow2s = num::range_step_inclusive(0, n_digits - 1, 1);
	pow2s
		.zip(binary_digits_msbf.iter().rev())
		.map(|(pow2, &is_on)| {
			let is_on = is_on as usize;
			is_on * 2usize.pow(pow2)
		})
		.reduce(|a, b| a + b)
		.unwrap_or(0)
}
