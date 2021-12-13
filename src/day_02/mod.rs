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
// end::setup[]

// tag::pt1[]
fn pt1<S: AsRef<str>>(s: S) -> Option<i32> {
	let mut h = 0;
	let mut v = 0;
	for line in s.as_ref().lines() {
		let mut tokens_iter = line.split_whitespace();
		let dir = tokens_iter.next()?;
		let dist = tokens_iter.next()?.parse::<i32>().ok()?;

		match dir {
			"forward" => h += dist,
			"up" => v -= dist,
			"down" => v += dist,
			_ => return None,
		}
	}

	Some(Position { h, v }.get_ans())
}

// end::pt1[]

// tag::pt2[]
fn pt2<S: AsRef<str>>(s: S) -> Option<i32> {
	let mut h = 0;
	let mut v = 0;
	let mut aim = 0;
	for line in s.as_ref().lines() {
		let mut tokens_iter = line.split_whitespace();
		let dir = tokens_iter.next()?;
		let dist = tokens_iter.next()?.parse::<i32>().ok()?;

		match dir {
			"forward" => {
				h += dist;
				v += aim * dist
			}
			"up" => aim -= dist,
			"down" => aim += dist,
			_ => return None,
		}
	}

	Some(Position { h, v }.get_ans())
}

// end::pt2[]

// tag::setup[]

pub fn ans() -> Answer<i32, i32> {
	let s = include_str!("./input.txt");
	(pt1(s).unwrap(), pt2(s).unwrap()).into()
}
// end::setup[]
