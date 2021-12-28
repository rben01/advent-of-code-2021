use std::fmt::{Display, Write};

use crate::utils::to_decimal;
use crate::Answer;
use ndarray::prelude::*;

type Bit = bool;

#[derive(Debug, Clone)]
struct Image {
	mat: Array2<Bit>,
	surrounding: Bit,
	algo: Vec<Bit>,
}

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
		let new_surrounding = if self.surrounding {
			self.algo[to_decimal([true; 9])]
		} else {
			self.algo[to_decimal([false; 9])]
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
						let r = r as i32 + dr;
						let c = c as i32 + dc;

						let bit = if r < 0
							|| r >= (grown_mat.nrows() as i32)
							|| c < 0 || c >= (grown_mat.ncols() as i32)
						{
							self.surrounding
						} else {
							grown_mat[[r as usize, c as usize]]
						};
						surrounding_pixels.push(bit);
					}
				}

				let replacement = self.algo[to_decimal(surrounding_pixels)];
				new_mat[[r, c]] = replacement
			}
		}

		self.mat = new_mat;
		self.surrounding = new_surrounding;
	}

	fn tick_n_times(&mut self, n: usize) {
		for _ in 0..n {
			self.tick()
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

fn pt1(im: Image) -> usize {
	let mut im = im;
	im.tick_n_times(2);
	im.mat.map(|&bit| bit as usize).sum()
}

fn pt2(im: Image) -> usize {
	let mut im = im;
	im.tick_n_times(50);
	im.mat.map(|&bit| bit as usize).sum()
}
