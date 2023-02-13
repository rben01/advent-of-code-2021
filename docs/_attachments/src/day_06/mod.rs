// tag::setup[]
use crate::Answer;

const N_TIMERS: usize = 9;
type Timers = [usize; N_TIMERS];

fn read_input(input: &str) -> Option<Timers> {
	let mut timers = [0; N_TIMERS];

	let nums = input
		.trim()
		.split(',')
		.map(|s| s.parse().ok())
		.collect::<Option<Vec<usize>>>()?;

	for num in nums {
		timers[num] += 1;
	}

	Some(timers)
}

fn tick_in_place(timers: &mut Timers) {
	let initial = timers[0];
	for i in 0..(N_TIMERS - 1) {
		timers[i] = timers[i + 1];
	}
	timers[8] = initial;
	timers[6] += initial;
}

// [usize; 9] implements Copy
fn tick(n_times: usize, timers: &Timers) -> Timers {
	let mut timers = *timers;
	for _ in 0..n_times {
		tick_in_place(&mut timers);
	}
	timers
}

fn ans_for_input(input: &str) -> Answer<usize, usize> {
	let timers = read_input(input).unwrap();
	(6, (pt1(&timers), pt2(&timers))).into()
}

pub fn ans() -> Answer<usize, usize> {
	ans_for_input(include_str!("sample_input.txt"))
}
// end::setup[]

// tag::pt1[]
fn pt1(timers: &Timers) -> usize {
	tick(80, timers).iter().sum()
}
// end::pt1[]

// tag::pt2[]
fn pt2(timers: &Timers) -> usize {
	tick(256, timers).iter().sum()
}
// end::pt2[]

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_input;

	#[test]
	fn test() {
		test_input!(include_str!("sample_input.txt"), day: 6, ans: (5934, 26_984_457_539));
		test_input!(include_str!("input.txt"), day: 6, ans: (372_984, 1_681_503_251_694));
	}
}
