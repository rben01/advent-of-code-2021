// tag::setup[]
use crate::{utils::abs_diff, Answer};

fn read_input(s: &str) -> Option<Vec<usize>> {
	s.trim()
		.split(',')
		.map(|n| n.parse().ok())
		.collect::<Option<Vec<_>>>()
}

fn ans_for_input(input: &str) -> Answer<usize, usize> {
	let nums = read_input(input).unwrap();
	(7, (pt1(&nums), pt2(&nums))).into()
}

pub fn ans() -> Answer<usize, usize> {
	ans_for_input(include_str!("input.txt"))
}
// end::setup[]
// tag::pt1[]
fn pt1<V: AsRef<[usize]>>(nums: V) -> usize {
	let mut nums = nums.as_ref().to_vec();
	nums.sort_unstable();
	let datum_below = nums[nums.len() / 2];
	let datum_above = nums[1 + (nums.len() - 1) / 2];
	let median = (datum_below + datum_above) / 2;
	nums.iter().map(|&n| abs_diff(n, median)).sum()
}
// end::pt1[]
// tag::pt2[]
fn pt2<V: AsRef<[usize]>>(nums: V) -> usize {
	fn cost(mean: usize, nums: &[usize]) -> usize {
		nums.iter()
			.map(|&n| {
				let diff = abs_diff(n, mean);
				diff * (diff + 1) / 2
			})
			.sum()
	}

	let nums = nums.as_ref();
	let sum = nums.iter().sum::<usize>();
	let len = nums.len();

	let mean_rounded_down = sum / len;

	if sum % len == 0 {
		cost(mean_rounded_down, nums)
	} else {
		let mean_rounded_up = (sum - 1) / len + 1;
		cost(mean_rounded_down, nums).min(cost(mean_rounded_up, nums))
	}
}
// end::pt2[]

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_input;

	#[test]
	fn test() {
		test_input!(include_str!("input.txt"), day: 7, ans: (328_187, 91_257_582));
	}
}
