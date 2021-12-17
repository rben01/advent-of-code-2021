// tag::setup[]
use crate::Answer;
use num::Integer;
use regex::Regex;
use std::collections::BTreeMap as Map;
use std::str::FromStr;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Point<T>(T, T);

struct EndpointPair<T: Integer>(Point<T>, Point<T>);

type PointCounter<T> = Map<Point<T>, usize>;

fn get_lines<T: Integer + FromStr>(input: &str) -> Option<Vec<EndpointPair<T>>> {
	let line_re = Regex::new(r"(\d+),(\d+)\s*->\s*(\d+),(\d+)").ok()?;
	input
		.lines()
		.map(|line| {
			let caps = line_re.captures(line)?;
			let [x1, y1, x2, y2] = [1, 2, 3, 4].map(|i| caps.get(i)?.as_str().parse::<T>().ok());
			Some(EndpointPair(Point(x1?, y1?), Point(x2?, y2?)))
		})
		.collect::<Option<Vec<_>>>()
}

fn range_between(a: i32, b: i32) -> num::iter::RangeStepInclusive<i32> {
	let step = if a < b { 1 } else { -1 };
	num::range_step_inclusive(a, b, step)
}

fn get_ans<T>(counter: &PointCounter<T>) -> usize {
	counter
		.values()
		.map(|count| if *count >= 2 { 1 } else { 0 })
		.sum()
}

fn ans_for_input(input: &str) -> Answer<usize, usize> {
	let endpoints = get_lines(input).unwrap();
	(5, (pt1(&endpoints), pt2(&endpoints))).into()
}

pub fn ans() -> Answer<usize, usize> {
	let input = include_str!("input.txt");
	ans_for_input(input)
}
// end::setup[]

// tag::pt1[]
fn get_hv_point_counts(endpoints: &[EndpointPair<i32>]) -> PointCounter<i32> {
	let mut counter = Map::new();
	for &EndpointPair(Point(x1, y1), Point(x2, y2)) in endpoints {
		if x1 != x2 && y1 != y2 {
			continue;
		}
		for x in range_between(x1, x2) {
			for y in range_between(y1, y2) {
				*counter.entry(Point(x, y)).or_default() += 1;
			}
		}
	}
	counter
}

fn pt1(endpoints: &[EndpointPair<i32>]) -> usize {
	get_ans(&get_hv_point_counts(endpoints))
}
// end::pt1[]

// tag::pt2[]
fn get_diag_point_counts(endpoints: &[EndpointPair<i32>]) -> PointCounter<i32> {
	let mut counter = Map::new();
	for &EndpointPair(Point(x1, y1), Point(x2, y2)) in endpoints {
		if (x1 - x2).abs() != (y1 - y2).abs() {
			continue;
		}

		for (x, y) in range_between(x1, x2).zip(range_between(y1, y2)) {
			*counter.entry(Point(x, y)).or_default() += 1;
		}
	}

	counter
}

fn pt2(endpoints: &[EndpointPair<i32>]) -> usize {
	let hv_counter = get_hv_point_counts(endpoints);

	let all_counter = {
		let mut diag_counter = get_diag_point_counts(endpoints);

		for (k, v) in hv_counter {
			*diag_counter.entry(k).or_default() += v;
		}

		diag_counter
	};
	get_ans(&all_counter)
}
// end::pt2[]

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_input;

	#[test]
	fn test() {
		test_input!(include_str!("sample_input.txt"), day: 5, ans: (5, 12));
		test_input!(include_str!("input.txt"), day: 5, ans: (5576, 18144));
	}
}
