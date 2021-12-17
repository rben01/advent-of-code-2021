use crate::Answer;

// tag::setup[]
type Num = i32;
type Directions = Vec<Direction>;

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
	Forward(Num),
	Up(Num),
	Down(Num),
}

fn read_input(s: &str) -> Option<Directions> {
	use Direction::*;
	s.lines()
		.map(|line| {
			let mut tokens_iter = line.split_whitespace();
			let dir = tokens_iter.next()?;
			let dist = tokens_iter.next()?.parse().ok()?;

			Some(match dir {
				"forward" => Forward(dist),
				"up" => Up(dist),
				"down" => Down(dist),
				_ => return None,
			})
		})
		.collect()
}

fn ans_for_input(input: &str) -> Answer<Num, Num> {
	let directions = read_input(input).unwrap();
	(2, (pt1(&directions), pt2(&directions))).into()
}

pub fn ans() -> Answer<Num, Num> {
	ans_for_input(include_str!("input.txt"))
}
// end::setup[]

// tag::pt1[]
fn pt1(directions: &Directions) -> Num {
	use Direction::*;
	let mut h = 0;
	let mut v = 0;
	for dir in directions.iter() {
		match dir {
			Forward(dist) => h += dist,
			Up(dist) => v -= dist,
			Down(dist) => v += dist,
		};
	}

	Position { h, v }.get_ans()
}
// end::pt1[]

// tag::pt2[]
fn pt2(directions: &Directions) -> Num {
	use Direction::*;
	let mut h = 0;
	let mut v = 0;
	let mut aim = 0;
	for dir in directions.iter() {
		match dir {
			Forward(dist) => {
				h += dist;
				v += aim * dist
			}
			Up(dist) => aim -= dist,
			Down(dist) => aim += dist,
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
		test_input!(include_str!("input.txt"), day: 2, ans: (1459206, 1320534480));
	}
}
