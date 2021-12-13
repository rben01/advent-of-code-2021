// tag::setup[]
use crate::Answer;
use num::Integer;
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
	fn from_dots<I: IntoIterator<Item = Point<T>>>(dots: I) -> Paper<T> {
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

	fn do_folds(&self, folds: &[Fold<T>]) -> Paper<T> {
		let mut paper = self.clone();
		for fold in folds {
			paper = paper.folded_across(fold);
		}
		paper
	}
}

impl Display for Paper<i32> {
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
		for y in num::zero()..=max_y {
			for x in num::zero()..=max_x {
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

fn read_input<T: Integer + FromStr + Copy>() -> Option<(Paper<T>, Vec<Fold<T>>)> {
	let s = include_str!("./input.txt");
	let mut lines = s.lines();

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
// end::setup[]

// tag::pt1[]
fn pt1<T: Integer + Copy>(paper: &Paper<T>, folds: &[Fold<T>]) -> usize {
	paper.do_folds(folds).dots.len()
}
// end::pt1[]

// tag::pt2[]
fn pt2(paper: &Paper<i32>, folds: &[Fold<i32>]) -> String {
	format!("{}", paper.do_folds(folds))
}
// end::pt2[]

// tag::setup[]

pub fn ans() -> Answer<usize, String> {
	let (paper, folds) = read_input::<i32>().unwrap();

	let p2 = pt2(&paper, &folds);
	println!("{}", p2);

	(pt1(&paper, &folds), p2).into()
}
// end::setup[]
