// tag::setup[]
use crate::Answer;
use std::{collections::BTreeSet as Set, fmt::Write};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
	row: usize,
	col: usize,
}

#[derive(Debug, Clone)]
struct SeaGarden {
	width: usize,
	height: usize,
	rights: Set<Point>,
	downs: Set<Point>,
}

// tag::debugging[]
impl std::fmt::Display for SeaGarden {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "{} rows × {} cols", self.width, self.height)?;
		for row in 0..self.height {
			for col in 0..self.width {
				let p = &Point { row, col };
				f.write_char(if self.rights.contains(p) {
					'>'
				} else if self.downs.contains(p) {
					'v'
				} else {
					'.'
				})?;
			}
			f.write_char('\n')?;
		}
		Ok(())
	}
}

// end::debugging[]
impl SeaGarden {
	fn from_str(input: &str) -> Self {
		let mut rights = Set::new();
		let mut downs = Set::new();

		let mut width = 0;
		let mut height = 0;

		for (row, line) in input.lines().enumerate() {
			height += 1;
			width = line.chars().count();
			for (col, c) in line.chars().enumerate() {
				let loc = Point { row, col };
				match c {
					'>' => rights.insert(loc),
					'v' => downs.insert(loc),
					'.' => continue,
					_ => unreachable!(),
				};
			}
		}
		Self {
			width,
			height,
			rights,
			downs,
		}
	}

	fn tick(&mut self) -> bool {
		let mut any_cucumbers_did_move = false;

		let mut new_rights = Set::new();

		for old_loc @ &Point { row, col } in &self.rights {
			let new_loc = Point {
				row,
				col: (col + 1) % self.width,
			};
			if self.rights.contains(&new_loc) || self.downs.contains(&new_loc) {
				new_rights.insert(*old_loc);
			} else {
				new_rights.insert(new_loc);
				any_cucumbers_did_move = true;
			}
		}

		let new_rights = new_rights;
		let mut new_downs = Set::new();

		for old_loc @ &Point { row, col } in &self.downs {
			let new_loc = Point {
				row: (row + 1) % self.height,
				col,
			};
			if new_rights.contains(&new_loc) || self.downs.contains(&new_loc) {
				new_downs.insert(*old_loc);
			} else {
				new_downs.insert(new_loc);
				any_cucumbers_did_move = true;
			}
		}

		self.rights = new_rights;
		self.downs = new_downs;

		any_cucumbers_did_move
	}

	fn run_until_no_movement(&mut self) -> usize {
		let mut n = 1;
		while self.tick() {
			n += 1;
		}
		n
	}
}

fn ans_for_input(input: &str) -> Answer<usize, usize> {
	let mut garden = SeaGarden::from_str(input);
	(25, (pt1(&mut garden), 0)).into()
}
pub fn ans() -> Answer<usize, usize> {
	ans_for_input(include_str!("input.txt"))
}

// end::setup[]
// tag::pt1[]
fn pt1(garden: &mut SeaGarden) -> usize {
	garden.run_until_no_movement()
}
// end::pt1[]

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_input;

	#[test]
	fn test() {
		test_input!(include_str!("input.txt"), day: 25, ans: (557, 0));
	}
}
