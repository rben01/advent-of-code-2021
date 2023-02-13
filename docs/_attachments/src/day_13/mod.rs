// tag::setup[]
use crate::Answer;
use num::{CheckedAdd, Integer};
use std::{collections::BTreeSet as Set, fmt::Display, str::FromStr};

#[derive(Copy, Clone)]
enum Fold<T> {
	X(T),
	Y(T),
}

impl<T: FromStr> Fold<T> {
	fn from_str(s: &str) -> Option<Self> {
		let mut words = s.split_whitespace();
		words.next()?;
		words.next()?;
		let fold_eqn = words.next()?;

		let mut eqn_sides = fold_eqn.split('=');
		let var = eqn_sides.next()?;
		let value = eqn_sides.next()?.parse::<T>().ok()?;

		Some(match var {
			"x" => Self::X(value),
			"y" => Self::Y(value),
			_ => return None,
		})
	}
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point<T: Integer>(T, T);

impl<T: Integer + Copy> Point<T> {
	fn folded(&self, across: &Fold<T>) -> Self {
		let &Point(x, y) = self;
		match *across {
			Fold::X(fold_x) => {
				let new_x = if x > fold_x { fold_x - (x - fold_x) } else { x };
				Point(new_x, y)
			}
			Fold::Y(fold_y) => {
				let new_y = if y > fold_y { fold_y - (y - fold_y) } else { y };
				Point(x, new_y)
			}
		}
	}
}

#[derive(Clone, Debug)]
struct Paper<T: Integer> {
	dots: Set<Point<T>>,
}

impl<T: Integer + Copy> Paper<T> {
	fn from_dots(dots: impl IntoIterator<Item = Point<T>>) -> Paper<T> {
		let dots = dots.into_iter().collect();
		Self { dots }
	}

	fn folded_across(&self, fold: &Fold<T>) -> Paper<T> {
		let mut dots = Set::new();
		for p in &self.dots {
			dots.insert(p.folded(fold));
		}

		Paper::from_dots(dots)
	}

	fn do_folds<F: std::borrow::Borrow<Fold<T>>>(
		&self,
		folds: impl Iterator<Item = F>,
	) -> Paper<T> {
		let mut paper = self.clone();
		for fold in folds {
			let fold = fold.borrow();
			paper = paper.folded_across(fold);
		}
		paper
	}
}

fn read_input<T: Integer + FromStr + Copy>(input: &str) -> Option<(Paper<T>, Vec<Fold<T>>)> {
	let mut lines = input.lines();

	let points = lines
		.by_ref()
		.take_while(|line| !line.trim().is_empty())
		.map(|line| {
			let mut comps = line.split(',');
			let x = comps.next()?.parse::<T>().ok()?;
			let y = comps.next()?.parse::<T>().ok()?;

			Some(Point(x, y))
		})
		.collect::<Option<Vec<_>>>()?;
	let paper = Paper::<T>::from_dots(points);

	let folds = lines.map(Fold::<T>::from_str).collect::<Option<Vec<_>>>()?;

	Some((paper, folds))
}

fn ans_for_input(input: &str) -> Answer<usize, String> {
	let (paper, folds) = read_input::<i32>(input).unwrap();
	(13, (pt1(&paper, &folds[0]), pt2(&paper, folds.iter()))).into()
}

pub fn ans() -> Answer<usize, String> {
	ans_for_input(include_str!("input.txt"))
}
// end::setup[]

// tag::pt1[]
fn pt1<T: Integer + Copy>(paper: &Paper<T>, fold: &Fold<T>) -> usize {
	paper.folded_across(fold).dots.len()
}
// end::pt1[]

// tag::pt2[]
impl<T: Integer + CheckedAdd + Clone + Copy> Display for Paper<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let (max_x, max_y) = {
			let mut max_x = num::zero();
			let mut max_y = num::zero();
			for &Point(x, y) in &self.dots {
				if x > max_x {
					max_x = x;
				}
				if y > max_y {
					max_y = y;
				}
			}
			(max_x, max_y)
		};
		for y in num::range_step_inclusive(num::zero(), max_y, num::one()) {
			for x in num::range_step_inclusive(num::zero(), max_x, num::one()) {
				f.write_str(if self.dots.contains(&Point(x, y)) {
					"█" // unicode "full block" 0x2588
				} else {
					" "
				})?;
			}
			f.write_str("\n")?;
		}

		Ok(())
	}
}

fn pt2<T: Integer + CheckedAdd + Clone + Copy, F: std::borrow::Borrow<Fold<T>>>(
	paper: &Paper<T>,
	folds: impl Iterator<Item = F>,
) -> String {
	let ans = paper.do_folds(folds);
	println!("{}", ans);
	ans.to_string()
}
// end::pt2[]

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_input;

	#[test]
	fn test() {
		let (paper, folds) = read_input::<i32>(include_str!("sample_input.txt")).unwrap();
		assert_eq!(pt1(&paper, &folds[0]), 17);

		test_input!(
			include_str!("input.txt"),
			day: 13,
			ans: (
				790,
				concat!(
					"███   ██  █  █ ████ ███  ████   ██  ██ \n",
					"█  █ █  █ █  █    █ █  █ █       █ █  █\n",
					"█  █ █    ████   █  ███  ███     █ █   \n",
					"███  █ ██ █  █  █   █  █ █       █ █   \n",
					"█    █  █ █  █ █    █  █ █    █  █ █  █\n",
					"█     ███ █  █ ████ ███  █     ██   ██ \n"
				).to_owned()
			)
		);
	}
}
