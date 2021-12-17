// tag::setup[]
use crate::{to_decimal, Answer};
use ndarray::prelude::*;

fn read_input(input: &str) -> Option<ndarray::Array2<bool>> {
	let mut lines = input.lines();
	let mut bit_vec = Vec::new();

	let first_line = lines.next()?;
	let line_length = first_line.len();

	for line in std::iter::once(first_line).chain(lines) {
		for c in line.bytes() {
			bit_vec.push(c == b'1');
		}
	}

	let n_lines = bit_vec.len() / line_length;
	Array2::from_shape_vec((n_lines, line_length), bit_vec).ok()
}

fn ans_for_input(input: &str) -> Answer<usize, usize> {
	let mat = read_input(input).unwrap();
	(3, (pt1(&mat), pt2(&mat))).into()
}

pub fn ans() -> Answer<usize, usize> {
	ans_for_input(include_str!("input.txt"))
}
// end::setup[]

// tag::pt1[]
fn pt1(mat: &Array2<bool>) -> usize {
	let (n_rows, n_cols) = mat.dim();

	let n_ones = mat.map(|x| *x as usize).sum_axis(Axis(0));
	let n_zeros = n_ones.map(|n| n_rows - n);

	let col_has_more_ones_than_zeros = ndarray::Zip::from(&n_ones)
		.and(&n_zeros)
		.map_collect(|n_o, n_z| n_o > n_z)
		.into_shape((n_cols,))
		.unwrap();

	let gamma_rate = to_decimal(col_has_more_ones_than_zeros.to_vec());
	let epsilon_rate = (2usize.pow(n_cols as u32) - 1) - gamma_rate;

	gamma_rate * epsilon_rate
}
// end::pt1[]

// tag::pt2[]
fn value_of_line_chosen_by_criterion(
	mat: &Array2<bool>,
	cmp_predicate: impl Fn(usize, usize) -> bool,
) -> usize {
	let (n_rows, n_cols) = mat.dim();
	let mut candidates = Array1::<_>::from_shape_simple_fn((n_rows,), || true);
	for i in 0..n_cols {
		let n_candidates_remaining = candidates.map(|c| if *c { 1usize } else { 0 }).sum();

		if n_candidates_remaining == 1 {
			break;
		}

		let column = mat.index_axis(Axis(1), i);
		let digit_sum = column
			.iter()
			.enumerate()
			.filter_map(|(i, &x)| {
				if candidates[[i]] {
					Some(x as usize)
				} else {
					None
				}
			})
			.sum::<usize>();

		let most_common_digit = cmp_predicate(2 * digit_sum, n_candidates_remaining);

		candidates = ndarray::Zip::from(&candidates)
			.and(&column)
			.map_collect(|&candidate, &digit| candidate && digit == most_common_digit);
	}

	let index = candidates
		.into_iter()
		.enumerate()
		.filter_map(|(i, x)| if x { Some(i) } else { None })
		.next()
		.unwrap();
	let line = mat.index_axis(Axis(0), index);

	to_decimal(line.to_vec())
}

fn pt2(mat: &Array2<bool>) -> usize {
	let [oxy_rate, co2_rate] =
		[|x, y| x >= y, |x, y| x < y].map(|op| value_of_line_chosen_by_criterion(mat, op));

	oxy_rate * co2_rate
}
// end::pt2[]

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_input;

	#[test]
	fn test() {
		test_input!(include_str!("sample_input.txt"), day: 3, ans: (198, 230));
		test_input!(include_str!("input.txt"), day: 3, ans: (2743844, 6677951));
	}
}
