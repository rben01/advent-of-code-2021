// tag::setup[]
use crate::Answer;
use ndarray::prelude::*;

type Cost = u32;
type Grid = Array2<Cost>;
type Coords = (usize, usize);

fn read_input(input: &str) -> Option<Grid> {
	let mut lines = input.lines();

	let mut grid = vec![];

	let first_line = lines.next()?;
	for c in first_line.chars() {
		grid.push(c.to_digit(10)? as Cost);
	}
	let n_cols = grid.len();

	for line in lines {
		for c in line.chars() {
			grid.push(c.to_digit(10)? as Cost);
		}
	}

	let n_rows = grid.len() / n_cols;
	assert_eq!(n_rows * n_cols, grid.len());

	Array2::from_shape_vec((n_rows, n_cols), grid).ok()
}

enum Direction {
	N,
	S,
	E,
	W,
}

impl Direction {
	fn stepping_from(&self, grid: &Grid, (from_row, from_col): Coords) -> Option<Coords> {
		use Direction::*;

		let (min_row, min_col) = (0, 0);
		let (n_rows, n_cols) = grid.dim();
		let max_row = n_rows - 1;
		let max_col = n_cols - 1;

		let new_row = match self {
			N if from_row == min_row => return None,
			N => from_row - 1,
			S if from_row == max_row => return None,
			S => from_row + 1,
			_ => from_row,
		};

		let new_col = match self {
			W if from_col == min_col => return None,
			W => from_col - 1,
			E if from_col == max_col => return None,
			E => from_col + 1,
			_ => from_col,
		};

		Some((new_row, new_col))
	}
}

fn traversal_cost(entry_costs: &Grid) -> Cost {
	use Direction::*;

	let (n_rows, n_cols) = entry_costs.dim();
	let max_row = n_rows - 1;
	let max_col = n_cols - 1;

	let mut net_travel_costs = Grid::from_shape_simple_fn((n_rows, n_cols), || Cost::MAX);
	net_travel_costs[(0, 0)] = 0;

	let max_dist = max_row + max_col;
	loop {
		let mut did_modify = false;

		for dist in 0..=max_dist {
			let r_min = if dist < max_col { 0 } else { dist - max_col };
			let r_max = dist.min(max_row);

			for r in r_min..=r_max {
				let c = dist - r;

				let net_cost_to_travel_here = net_travel_costs[(r, c)];

				// One of the perks of moving diagonally, down and to the right, is that
				// this assertion holds (which means the following (necessary) loop isn't
				// pointless)
				assert_ne!(net_cost_to_travel_here, Cost::MAX);

				for dir in [N, S, E, W] {
					let nghbr_coords = match dir.stepping_from(entry_costs, (r, c)) {
						Some(nghbr_coords) => nghbr_coords,
						None => continue,
					};

					let net_cost_to_travel_to_nghbr_thru_here =
						net_cost_to_travel_here + entry_costs[nghbr_coords];

					if net_cost_to_travel_to_nghbr_thru_here < net_travel_costs[nghbr_coords] {
						net_travel_costs[nghbr_coords] = net_cost_to_travel_to_nghbr_thru_here;
						did_modify = true;
					}
				}
			}
		}

		if !did_modify {
			return net_travel_costs[(max_row, max_col)];
		}
	}
}

fn ans_for_input(input: &str) -> Answer<Cost, Cost> {
	let grid = read_input(input).unwrap();
	(15, (pt1(&grid), pt2(&grid))).into()
}

pub fn ans() -> Answer<Cost, Cost> {
	ans_for_input(include_str!("input.txt"))
}
// end::setup[]

// tag::pt1[]
fn pt1(grid: &Grid) -> Cost {
	traversal_cost(grid)
}
// end::pt1[]

// tag::pt2[]
fn expand_grid(grid: &Grid, k: usize) -> Grid {
	let (n_rows, n_cols) = grid.dim();
	let mut new_grid = Array2::from_shape_simple_fn((k * n_rows, k * n_cols), || 0);

	// Arrays are in row major order, so that's the order we iterate in (for cache-friendliness)
	for outer_r in 0..k {
		for inner_r in 0..n_rows {
			for outer_c in 0..k {
				for inner_c in 0..n_cols {
					let old_cost = grid[(inner_r, inner_c)];

					let d_cost = Cost::try_from(outer_r + outer_c).unwrap();
					let new_cost = (old_cost + d_cost - 1) % 9 + 1;

					let new_grid_r = outer_r * n_rows + inner_r;
					let new_grid_c = outer_c * n_cols + inner_c;

					new_grid[(new_grid_r, new_grid_c)] = new_cost;
				}
			}
		}
	}

	new_grid
}

fn pt2(grid: &Grid) -> Cost {
	let grid = expand_grid(grid, 5);
	traversal_cost(&grid)
}
// end::pt2[]

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_input;

	#[test]
	fn test() {
		test_input!(include_str!("sample_input.txt"), day: 15, ans: (40, 315));
		test_input!(include_str!("input.txt"), day: 15, ans: (739, 3040));
	}
}
