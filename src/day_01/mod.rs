// tag::setup[]
use crate::Answer;
use std::collections::VecDeque;

fn get_n_increasing_running_sum_of_depths(n: usize) -> Option<usize> {
	let mut depth_buf = VecDeque::with_capacity(n);
	let mut depths = include_str!("./input.txt")
		.lines()
		.map(|line| line.parse::<i32>().unwrap());

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

pub fn ans() -> Answer<usize, usize> {
	(1, (pt1(), pt2())).into()
}
// end::setup[]

// tag::pt1[]
pub fn pt1() -> usize {
	get_n_increasing_running_sum_of_depths(1).unwrap()
}
// end::pt1[]

// tag::pt2[]
pub fn pt2() -> usize {
	get_n_increasing_running_sum_of_depths(3).unwrap()
}
// end::pt2[]
