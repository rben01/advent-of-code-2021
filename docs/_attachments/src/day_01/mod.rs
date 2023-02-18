// tag::setup[]
use crate::Answer;
use std::collections::VecDeque;

fn get_n_increasing_running_sum_of_depths(input: &str, n: usize) -> Option<usize> {
	let mut depth_buf = VecDeque::with_capacity(n);
	let mut depths = input.lines().map(|line| line.parse::<i32>().unwrap());

	depth_buf.extend(depths.by_ref().take(n));
	if depth_buf.len() < n {
		return None;
	}

	let mut n_increasing = 0;

	for new_depth in depths {
		let old_depth = depth_buf.pop_front().unwrap();
		depth_buf.push_back(new_depth);

		if new_depth > old_depth {
			n_increasing += 1;
		}
	}

	Some(n_increasing)
}

fn ans_for_input(input: &str) -> Answer<usize, usize> {
	(1, (pt1(input), pt2(input))).into()
}

pub fn ans() -> Answer<usize, usize> {
	ans_for_input(include_str!("input.txt"))
}
// end::setup[]

// tag::pt1[]
fn pt1(input: &str) -> usize {
	get_n_increasing_running_sum_of_depths(input, 1).unwrap()
}
// end::pt1[]

// tag::pt2[]
fn pt2(input: &str) -> usize {
	get_n_increasing_running_sum_of_depths(input, 3).unwrap()
}
// end::pt2[]

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_input;

	#[test]
	fn test() {
		test_input!(include_str!("sample_input.txt"), day: 1, ans: (7, 5));
		test_input!(include_str!("input.txt"), day: 1, ans: (1681, 1704));
	}
}
