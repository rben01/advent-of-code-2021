use crate::Answer;

// tag::setup[]
struct Position {
	h: i32,
	v: i32,
}

impl Position {
	fn get_ans(&self) -> i32 {
		self.h * self.v
	}
}

enum Direction {
	Forward,
	Up,
	Down,
}

impl Direction {
	fn from_str(s: &str) -> Option<Self> {
		use Direction::*;
		Some(match s {
			"forward" => Forward,
			"up" => Up,
			"down" => Down,
			_ => return None,
		})
	}
}

struct Step {
	direction: Direction,
	dist: i32,
}

fn read_input(s: &str) -> Option<Vec<Step>> {
	s.lines()
		.map(|line| {
			let mut tokens_iter = line.split_whitespace();
			let direction = Direction::from_str(tokens_iter.next()?)?;
			let dist = tokens_iter.next()?.parse().ok()?;

			Some(Step { direction, dist })
		})
		.collect()
}

fn ans_for_input(input: &str) -> Answer<i32, i32> {
	let directions = read_input(input).unwrap();
	(2, (pt1(directions.iter()), pt2(directions.iter()))).into()
}

pub fn ans() -> Answer<i32, i32> {
	ans_for_input(include_str!("input.txt"))
}
// end::setup[]

// tag::pt1[]
fn pt1<T: std::ops::Deref<Target = Step>>(steps: impl Iterator<Item = T>) -> i32 {
	use Direction::*;
	let mut h = 0;
	let mut v = 0;
	for step in steps {
		let Step { direction, dist } = &*step;
		match direction {
			Forward => h += dist,
			Up => v -= dist,
			Down => v += dist,
		};
	}

	Position { h, v }.get_ans()
}
// end::pt1[]

// tag::pt2[]
fn pt2<T: std::borrow::Borrow<Step>>(steps: impl Iterator<Item = T>) -> i32 {
	use Direction::*;
	let mut h = 0;
	let mut v = 0;
	let mut aim = 0;
	for step in steps {
		let Step { direction, dist } = step.borrow();
		let dist = *dist;
		match direction {
			Forward => {
				h += dist;
				v += aim * dist;
			}
			Up => aim -= dist,
			Down => aim += dist,
		};
	}

	Position { h, v }.get_ans()
}
// end::pt2[]

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_input;

	#[test]
	fn test() {
		test_input!(include_str!("sample_input.txt"), day: 2, ans: (150, 900));
		test_input!(include_str!("input.txt"), day: 2, ans: (1_459_206, 1_320_534_480));
	}
}
