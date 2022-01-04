// tag::setup[]
use crate::Answer;
use ndarray::prelude::*;
use std::collections::BTreeSet as Set;

#[derive(Debug)]
struct Heightmap {
	arr: Array2<usize>,
}

fn get_neighbor_idxs(
	[row, col]: [usize; 2],
	n_rows: usize,
	n_cols: usize,
) -> [[Option<usize>; 2]; 4] {
	[
		[(row + 1 < n_rows).then_some(row + 1), Some(col)],
		[row.checked_sub(1), Some(col)],
		[Some(row), (col + 1 < n_cols).then_some(col + 1)],
		[Some(row), col.checked_sub(1)],
	]
}

impl Heightmap {
	fn from_str(s: &str) -> Option<Self> {
		let width = s.lines().next()?.chars().count();
		let height = s.lines().count();

		let mut data = Vec::new();
		for line in s.lines() {
			data.extend(
				line.trim()
					.chars()
					.map(|c| c.to_digit(10).and_then(|d| usize::try_from(d).ok())),
			);
		}
		let data = data.iter().copied().collect::<Option<Vec<_>>>()?;

		let arr = Array2::from_shape_vec((width, height), data).ok()?;
		Some(Self { arr })
	}
}

fn ans_for_input(input: &str) -> Answer<usize, usize> {
	let hm = Heightmap::from_str(input).unwrap();
	(9, (pt1(&hm), pt2(&hm))).into()
}

pub fn ans() -> Answer<usize, usize> {
	ans_for_input(include_str!("input.txt"))
}
// end::setup[]

// tag::pt1[]
impl Heightmap {
	fn is_lower_than_neighbors(&self, idx: [usize; 2]) -> bool {
		let val = self.arr[idx];
		let (n_rows, n_cols) = self.arr.dim();

		let neighbors = get_neighbor_idxs(idx, n_rows, n_cols);
		for idx_pair in neighbors {
			let idxs = match idx_pair {
				[Some(r), Some(c)] => [r, c],
				_ => continue,
			};
			if self.arr[idxs] <= val {
				return false;
			}
		}

		true
	}

	fn idxs_where_lower_than_neighbors(&self) -> Vec<[usize; 2]> {
		let (n_rows, n_cols) = self.arr.dim();
		let mut idxs = Vec::new();
		for r in 0..n_rows {
			for c in 0..n_cols {
				if self.is_lower_than_neighbors([r, c]) {
					idxs.push([r, c]);
				}
			}
		}
		idxs
	}
}

fn pt1(hm: &Heightmap) -> usize {
	hm.idxs_where_lower_than_neighbors()
		.into_iter()
		.map(|idx| hm.arr[idx] + 1)
		.sum()
}
// end::pt1[]

// tag::pt2[]
impl Heightmap {
	fn basin_sizes(&self) -> Vec<usize> {
		let (n_rows, n_cols) = self.arr.dim();
		let mut basin_sizes = Vec::new();

		let mut not_yet_visited_idxs = (0..n_rows)
			.flat_map(|r| {
				(0..n_cols).filter_map(move |c| {
					let idx = [r, c];
					(self.arr[idx] != 9).then_some(idx)
				})
			})
			.collect::<Set<_>>();

		while let Some(first_idx) = not_yet_visited_idxs.pop_first() {
			let mut visited_idxs = Set::new();
			let mut coords_stack = vec![first_idx];
			while let Some(idx) = coords_stack.pop() {
				if !visited_idxs.insert(idx) {
					continue;
				}

				let neighbor_idxs = get_neighbor_idxs(idx, n_rows, n_cols);

				for nghbr_idx in neighbor_idxs {
					let nghbr_idx = match nghbr_idx {
						[Some(r), Some(c)] => [r, c],
						_ => continue,
					};
					if self.arr[nghbr_idx] != 9 && !visited_idxs.contains(&nghbr_idx) {
						coords_stack.push(nghbr_idx);
					}
				}

				for idx in &visited_idxs {
					not_yet_visited_idxs.remove(idx);
				}
			}
			basin_sizes.push(visited_idxs.len());
		}

		basin_sizes
	}
}

fn pt2(hm: &Heightmap) -> usize {
	let mut basin_sizes = hm.basin_sizes();
	basin_sizes.sort_unstable_by_key(|&size| std::cmp::Reverse(size));
	basin_sizes.into_iter().take(3).product()
}
// end::pt2[]
