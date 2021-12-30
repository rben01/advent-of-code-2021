// tag::setup[]
use crate::{utils::to_decimal, Answer};
use ndarray::prelude::*;
use std::fmt::{Display, Write};

type Bit = bool;

#[derive(Debug, Clone)]
struct Image {
	mat: Array2<Bit>,
	surrounding: Bit,
	algo: Vec<Bit>,
}

// tag::debugging[]
impl Display for Image {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		writeln!(f, "outer: {}", self.surrounding)?;
		for r in 0..self.mat.nrows() {
			for c in 0..self.mat.ncols() {
				let bit = if self.mat[[r, c]] { '#' } else { '.' };
				f.write_char(bit)?;
			}
			f.write_char('\n')?;
		}
		Ok(())
	}
}
// end::debugging[]
impl Image {
	fn from_str(s: &str) -> Option<Self> {
		let mut lines = s.lines();
		let algo = lines
			.next()?
			.chars()
			.map(|c| match c {
				'#' => true,
				'.' => false,
				_ => unreachable!(),
			})
			.collect();

		let data = lines
			.flat_map(|line| {
				line.trim().chars().map(|c| match c {
					'#' => true,
					'.' => false,
					_ => unreachable!(),
				})
			})
			.collect::<Vec<_>>();

		let n_cols = s.lines().nth_back(0)?.trim().len();
		let n_rows = data.len() / n_cols;

		assert_eq!(n_rows * n_cols, data.len());

		Some(Self {
			mat: Array2::from_shape_vec((n_rows, n_cols), data).unwrap(),
			surrounding: false,
			algo,
		})
	}

	fn tick(&mut self) {
		let new_surrounding = {
			let index = usize::try_from(to_decimal([self.surrounding; 9])).unwrap();
			self.algo[index]
		};

		let grown_mat =
			Array2::from_shape_fn((self.mat.nrows() + 2, self.mat.ncols() + 2), |(r, c)| {
				if r < 1 || r > self.mat.nrows() || c < 1 || c > self.mat.ncols() {
					self.surrounding
				} else {
					self.mat[[r - 1, c - 1]]
				}
			});

		let mut new_mat = Array2::from_shape_simple_fn(grown_mat.dim(), || false);

		for r in 0..grown_mat.nrows() {
			for c in 0..grown_mat.ncols() {
				let mut surrounding_pixels = vec![];
				for dr in [-1, 0, 1] {
					for dc in [-1, 0, 1] {
						let r = i32::try_from(r).unwrap() + dr;
						let c = i32::try_from(c).unwrap() + dc;

						let bit = if r < 0
							|| r >= i32::try_from(grown_mat.nrows()).unwrap()
							|| c < 0 || c >= i32::try_from(grown_mat.ncols()).unwrap()
						{
							self.surrounding
						} else {
							grown_mat[[r, c].map(|m| usize::try_from(m).unwrap())]
						};
						surrounding_pixels.push(bit);
					}
				}

				let replacement = self.algo[to_decimal(surrounding_pixels) as usize];
				new_mat[[r, c]] = replacement;
			}
		}

		self.mat = new_mat;
		self.surrounding = new_surrounding;
	}

	fn tick_n_times(&mut self, n: usize) {
		for _ in 0..n {
			self.tick();
		}
	}
}

fn ans_for_input(input: &str) -> Answer<usize, usize> {
	let im1 = Image::from_str(input).unwrap();
	let im2 = im1.clone();
	(20, (pt1(im1), pt2(im2))).into()
}

pub fn ans() -> Answer<usize, usize> {
	ans_for_input(include_str!("input.txt"))
}
// end::setup[]
// tag::pt1[]
fn pt1(im: Image) -> usize {
	let mut im = im;
	im.tick_n_times(2);
	im.mat.map(|&bit| usize::from(bit)).sum()
}
// end::pt1[]
// tag::pt2[]
fn pt2(im: Image) -> usize {
	let mut im = im;
	im.tick_n_times(50);
	im.mat.map(|&bit| usize::from(bit)).sum()
}
//end::pt2[]
