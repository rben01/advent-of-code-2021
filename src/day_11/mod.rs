// tag::setup[]
use crate::Answer;
use ndarray::prelude::*;

#[derive(Clone, Debug)]
struct Octopi {
	arr: Array2<usize>,
}

impl Octopi {
	fn from_str(s: &str) -> Option<Self> {
		let mut data = Vec::new();
		let width = s.lines().next()?.len();
		let height = s.lines().count();
		for line in s.lines() {
			data.extend(
				line.trim()
					.chars()
					.map(|c| c.to_digit(10).and_then(|d| usize::try_from(d).ok())),
			);
		}
		let data = data.into_iter().collect::<Option<Vec<_>>>()?;
		let arr = Array2::from_shape_vec((width, height), data).ok()?;
		Some(Self { arr })
	}

	fn tick_in_place_and_count_flashes(&mut self) -> usize {
		// We use equality with FLASH_THRESH to mean "will flash right now", whereas being
		// greater than FLASH_THRESH means "has already flashed (and won't flash again)"
		let flash_thresh = 10;

		let (n_rows, n_cols) = self.arr.dim();

		self.arr.mapv_inplace(|x| x + 1);
		let mut n_flashes = 0;

		loop {
			let flashing_octopi_idxs = (0..n_rows)
				.flat_map(|r| {
					let arr = &self.arr;
					(0..n_cols).filter_map(move |c| {
						let idx = [r, c];
						let val = &arr[idx];
						(*val == flash_thresh).then_some(idx)
					})
				})
				.collect::<Vec<_>>();

			if flashing_octopi_idxs.is_empty() {
				break;
			}

			n_flashes += flashing_octopi_idxs.len();

			for [base_row, base_col] in flashing_octopi_idxs {
				self.arr[[base_row, base_col]] += 1;

				let rows = [
					base_row.checked_sub(1),
					Some(base_row),
					(base_row + 1 < n_rows).then_some(base_row + 1),
				];
				let cols = [
					base_col.checked_sub(1),
					Some(base_col),
					(base_col + 1 < n_rows).then_some(base_col + 1),
				];

				for row in rows.into_iter().flatten() {
					for col in cols.into_iter().flatten() {
						if self.arr[[row, col]] < flash_thresh {
							self.arr[[row, col]] += 1;
						}
					}
				}
			}
		}

		self.arr
			.mapv_inplace(|val| if val >= flash_thresh { 0 } else { val });

		n_flashes
	}
}

fn ans_for_input(input: &str) -> Answer<usize, usize> {
	let octopi = Octopi::from_str(input).unwrap();
	(11, (pt1(octopi.clone()), pt2(octopi))).into()
}

pub fn ans() -> Answer<usize, usize> {
	ans_for_input(include_str!("input.txt"))
}
// end::setup[]

// tag::pt1[]
fn tick(mut octopi: Octopi, n: usize) -> usize {
	let mut n_flashes = 0;
	for _ in 0..n {
		n_flashes += octopi.tick_in_place_and_count_flashes();
	}
	n_flashes
}

fn pt1(octopi: Octopi) -> usize {
	tick(octopi, 100)
}
// end::pt1[]

// tag::pt2[]
fn pt2(mut octopi: Octopi) -> usize {
	let mut n = 0;
	while octopi.arr.iter().any(|&val| val != 0) {
		octopi.tick_in_place_and_count_flashes();
		n += 1;
	}
	n
}
// end::pt2[]
