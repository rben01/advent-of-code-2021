// tag::amphipods[]
use crate::{utils::abs_diff, Answer};
use hashbrown::hash_map::DefaultHashBuilder;
use ndarray::prelude::*;
use priority_queue::PriorityQueue;
use std::{
	collections::{BTreeMap as Map, BTreeSet},
	fmt::Write,
};

type Point = [usize; 2];
const ROW: usize = 0;
const COL: usize = 1;

#[derive(Eq, PartialEq, Debug)]
struct Cost(usize);

impl std::cmp::Ord for Cost {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		std::cmp::Reverse(self.0).cmp(&std::cmp::Reverse(other.0))
	}
}

impl std::cmp::PartialOrd for Cost {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
#[repr(usize)]
enum AmphipodKind {
	A = 0,
	B,
	C,
	D,
}

impl AmphipodKind {
	const fn n_kinds() -> usize {
		4
	}

	const fn from_usize(n: usize) -> Self {
		use AmphipodKind::*;
		match n {
			0 => A,
			1 => B,
			2 => C,
			3 => D,
			_ => panic!("Cannot construct an AmphipodKind from n >= 4"),
		}
	}

	const fn from_char(c: char) -> Self {
		use AmphipodKind::*;
		match c {
			'A' => A,
			'B' => B,
			'C' => C,
			'D' => D,
			_ => panic!("Cannot construct an AmphipodKind from c not in [A, B, C, D]"),
		}
	}

	const fn for_col(col: usize) -> Option<Self> {
		use AmphipodKind::*;
		Some(match col {
			3 => A,
			5 => B,
			7 => C,
			9 => D,
			_ => return None,
		})
	}

	const fn dest_col(self) -> usize {
		use AmphipodKind::*;
		match self {
			A => 3,
			B => 5,
			C => 7,
			D => 9,
		}
	}

	// tag::debugging[]
	fn to_char(self) -> char {
		use AmphipodKind::*;
		match self {
			A => 'A',
			B => 'B',
			C => 'C',
			D => 'D',
		}
	}
	// end::debugging[]
	fn energy(self) -> usize {
		use AmphipodKind::*;
		match self {
			A => 1,
			B => 10,
			C => 100,
			D => 1000,
		}
	}
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Amphipod {
	kind: AmphipodKind,
	index: usize,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct AmphipodIndexed<T, const N: usize>([T; N]);

impl<T, const N: usize> AmphipodIndexed<T, N> {
	const fn n_amphipods_per_kind() -> usize {
		N / AmphipodKind::n_kinds()
	}

	const fn index_for(am: Amphipod) -> usize {
		Self::n_amphipods_per_kind() * (am.kind as usize) + am.index
	}

	fn iter_items(&self) -> impl Iterator<Item = (Amphipod, &T)> {
		self.0.iter().enumerate().map(|(i, x)| {
			(
				Amphipod {
					kind: AmphipodKind::from_usize(i / Self::n_amphipods_per_kind()),
					index: i % Self::n_amphipods_per_kind(),
				},
				x,
			)
		})
	}
}

impl<T, const N: usize> std::ops::Index<Amphipod> for AmphipodIndexed<T, N> {
	type Output = T;

	fn index(&self, am: Amphipod) -> &Self::Output {
		&self.0[Self::index_for(am)]
	}
}

impl<T, const N: usize> std::ops::IndexMut<Amphipod> for AmphipodIndexed<T, N> {
	fn index_mut(&mut self, am: Amphipod) -> &mut Self::Output {
		&mut self.0[Self::index_for(am)]
	}
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Tile {
	Wall,
	Hallway,
	Doorway,
	SideRoom(AmphipodKind),
}

// tag::debugging[]
impl Tile {
	fn to_char(self) -> char {
		use Tile::*;
		match self {
			Hallway | SideRoom(_) => '•',
			Doorway => '□',
			Wall => '#',
		}
	}
}
// end::debugging[]
#[derive(Debug)]
struct InstantiatedBurrow<const N: usize> {
	burrow: Burrow,
	amphipod_locs: AmphipodIndexed<Point, N>,
}

impl<const N: usize> InstantiatedBurrow<N> {
	fn from_str(s: &str) -> Option<Self> {
		use Tile::*;

		let mut width = 0_usize;
		let mut height = 0_usize;

		let mut tiles_map = Map::new();
		let mut amphipod_locs_map = Map::new();
		let mut amphipod_kind_counts = [0_usize; 4];

		for (row, line) in s.lines().enumerate() {
			height += 1;
			for (col, c) in line.chars().enumerate() {
				if row == 0 {
					width += 1;
				}
				let tile = match c {
					'#' | ' ' => Wall,
					'.' => Hallway,
					'A' | 'B' | 'C' | 'D' => {
						let amphipod_kind = AmphipodKind::from_char(c);

						amphipod_locs_map.insert(
							Amphipod {
								kind: amphipod_kind,
								index: amphipod_kind_counts[amphipod_kind as usize],
							},
							[row, col],
						);

						amphipod_kind_counts[amphipod_kind as usize] += 1;

						SideRoom(AmphipodKind::for_col(col)?)
					}
					_ => return None,
				};

				tiles_map.insert((row, col), tile);
			}
		}

		let mut tiles = Array2::from_shape_fn((height, width), |(row, col)| {
			*tiles_map.get(&(row, col)).unwrap_or(&Wall)
		});

		for row in 0..tiles.nrows() {
			for col in 0..tiles.ncols() {
				if matches!(tiles[[row, col]], Hallway)
					&& matches!(tiles[[row + 1, col]], SideRoom(_))
				{
					tiles[[row, col]] = Doorway;
				}
			}
		}

		let amphipod_locs = {
			let mut locs = AmphipodIndexed(
				vec![[usize::MAX; 2]; amphipod_locs_map.len()]
					.try_into()
					.unwrap(),
			);
			for (&am, &loc) in &amphipod_locs_map {
				locs[am] = loc;
			}
			locs
		};

		Some(Self {
			burrow: Burrow { tiles },
			amphipod_locs,
		})
	}
}

// tag::debugging[]
impl<const N: usize> std::fmt::Display for InstantiatedBurrow<N> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let tiles = &self.burrow.tiles;
		let locs_amphipods = self
			.amphipod_locs
			.iter_items()
			.map(|(k, &v)| (v, k))
			.collect::<Map<_, _>>();
		for row in 0..tiles.nrows() {
			for col in 0..tiles.ncols() {
				write!(
					f,
					"{}",
					match locs_amphipods.get(&[row, col]) {
						Some(am) => am.kind.to_char(),
						None => tiles[[row, col]].to_char(),
					}
				)?;
			}
			f.write_char('\n')?;
		}
		Ok(())
	}
}
// end::debugging[]
#[derive(Debug)]
struct Burrow {
	tiles: Array2<Tile>,
}
// end::amphipods[]

// tag::debugging[]
impl std::fmt::Display for Burrow {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let tiles = &self.tiles;

		for row in 0..tiles.nrows() {
			for col in 0..tiles.ncols() {
				write!(
					f,
					"{}",
					match tiles[[row, col]] {
						Tile::SideRoom(kind) => kind.to_char(),
						tile => tile.to_char(),
					}
				)?;
			}
			f.write_char('\n')?;
		}
		Ok(())
	}
}
// end::debugging[]
// tag::solve[]
impl Burrow {
	fn solve<const N: usize>(&self, initial_locs: &AmphipodIndexed<Point, N>) -> Cost {
		const N_KINDS: usize = AmphipodKind::n_kinds();

		#[derive(Debug)]
		struct Update {
			amphipod: Amphipod,
			n_steps: usize,
			new_state: SingleAmphipodState,
		}

		#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
		struct SingleAmphipodState {
			loc: Point,
			is_done: bool,
		}
		type AllAmphipodStates<const N: usize> = AmphipodIndexed<SingleAmphipodState, N>;

		fn enqueue<H: std::hash::BuildHasher + Default, const N: usize>(
			seen: &mut BTreeSet<AllAmphipodStates<N>>,
			pq: &mut PriorityQueue<AllAmphipodStates<N>, Cost, H>,
			state: AllAmphipodStates<N>,
			cost: Cost,
		) {
			if seen.contains(&state) {
				return;
			}
			seen.insert(state);
			pq.push_increase(state, cost);
		}

		let hallway_row = 1;
		let sideroom_max_row = self.tiles.nrows() - 2;

		let mut pq = PriorityQueue::<_, _, DefaultHashBuilder>::with_default_hasher();
		let mut seen = BTreeSet::new();

		let tiles = &self.tiles;

		let initial_state = AmphipodIndexed(initial_locs.0.map(|loc| SingleAmphipodState {
			loc,
			is_done: false,
		}));

		pq.push(initial_state, Cost(0));

		let mut updates = Vec::new();

		while let Some((total_state, cost)) = pq.pop() {
			updates.clear();

			if total_state
				.iter_items()
				.all(|(am, state)| tiles[state.loc] == Tile::SideRoom(am.kind))
			{
				return cost;
			}

			let amphipod_locs = total_state
				.iter_items()
				.map(|(am, state)| (state.loc, am))
				.collect::<Map<_, _>>();

			let n_in_correct_sideroom_by_kind = {
				let mut arr = [0_usize; N_KINDS];
				for (am, state) in total_state.iter_items() {
					if tiles[state.loc] == Tile::SideRoom(am.kind) {
						arr[am.kind as usize] += 1;
					}
				}
				arr
			};

			let siderooms_available_for_amphipods =
				[0, 1, 2, 3].zip([3, 5, 7, 9_usize]).map(|(i, col)| {
					(hallway_row + 1..=sideroom_max_row).all(|row| {
						amphipod_locs
							.get(&[row, col])
							.map_or(true, |am| am.kind as usize == i)
					})
				});

			let update_that_moves_am_to_final_loc =
				total_state.iter_items().find_map(|(am, &state)| {
					if state.is_done || !siderooms_available_for_amphipods[am.kind as usize] {
						return None;
					}

					let curr_loc = state.loc;
					let dest_col = am.kind.dest_col();
					let sideroom_last_empty_row =
						sideroom_max_row - n_in_correct_sideroom_by_kind[am.kind as usize];
					let new_loc = [sideroom_last_empty_row, dest_col];

					if curr_loc[COL] == dest_col && curr_loc[ROW] >= sideroom_last_empty_row {
						return Some(Update {
							amphipod: am,
							n_steps: 0,
							new_state: SingleAmphipodState {
								loc: curr_loc,
								is_done: true,
							},
						});
					}

					let [curr_row, curr_col] = curr_loc;

					for row in hallway_row..curr_row {
						if amphipod_locs.contains_key(&[row, curr_col]) {
							return None;
						}
					}

					#[allow(clippy::range_minus_one)]
					let col_range = if curr_col < dest_col {
						curr_col + 1..=dest_col
					} else {
						dest_col..=curr_col - 1
					};
					for col in col_range {
						if amphipod_locs.contains_key(&[hallway_row, col]) {
							return None;
						}
					}

					let n_steps_up = curr_row - hallway_row;
					let n_steps_sideways = abs_diff(curr_col, dest_col);
					let n_steps_down = sideroom_last_empty_row - hallway_row;
					let n_steps = n_steps_up + n_steps_sideways + n_steps_down;

					Some(Update {
						amphipod: am,
						n_steps,
						new_state: SingleAmphipodState {
							loc: new_loc,
							is_done: true,
						},
					})
				});

			if let Some(update) = update_that_moves_am_to_final_loc {
				updates.push(update);
			} else {
				for (am, &state) in total_state.iter_items() {
					if state.is_done {
						continue;
					}

					let curr_loc = state.loc;
					let curr_tile = tiles[curr_loc];

					if !(matches!(curr_tile, Tile::SideRoom(_))
						&& (hallway_row..curr_loc[ROW])
							.all(|row| !amphipod_locs.contains_key(&[row, curr_loc[COL]])))
					{
						continue;
					}

					let n_steps_to_hallway = curr_loc[ROW] - hallway_row;

					// Rust...
					for range in [
						&mut (0..curr_loc[COL]).rev(),
						&mut (curr_loc[COL] + 1..tiles.ncols()),
					] as [&mut dyn Iterator<Item = usize>; 2]
					{
						for col in range {
							let new_loc = [hallway_row, col];
							if amphipod_locs.contains_key(&new_loc) || tiles[new_loc] == Tile::Wall
							{
								break;
							}
							if tiles[new_loc] == Tile::Doorway {
								continue;
							}
							updates.push(Update {
								amphipod: am,
								n_steps: n_steps_to_hallway + abs_diff(col, curr_loc[COL]),
								new_state: SingleAmphipodState {
									loc: new_loc,
									is_done: false,
								},
							});
						}
					}
				}
			}

			for &Update {
				amphipod: am,
				n_steps,
				new_state,
			} in &updates
			{
				let mut new_total_state = total_state;
				new_total_state[am] = new_state;

				enqueue(
					&mut seen,
					&mut pq,
					new_total_state,
					Cost(cost.0 + n_steps * am.kind.energy()),
				);
			}
		}

		// tag::debugging[]
		for amphipod_locs in &seen {
			println!(
				"{}",
				InstantiatedBurrow {
					burrow: Burrow {
						tiles: tiles.clone()
					},
					amphipod_locs: AmphipodIndexed(amphipod_locs.0.map(|state| state.loc))
				}
			);
		}
		// end::debugging[]
		panic!("Could not find a path to the finish line!");
	}
}

pub fn ans() -> Answer<usize, usize> {
	(23, (pt1(), pt2())).into()
}
// end::solve[]

// tag::pt1[]
fn pt1() -> usize {
	let b = InstantiatedBurrow::<8>::from_str(include_str!("input_1.txt")).unwrap();
	b.burrow.solve(&b.amphipod_locs).0
}

// end::pt1[]
// tag::pt2[]
fn pt2() -> usize {
	let b = InstantiatedBurrow::<16>::from_str(include_str!("input_2.txt")).unwrap();
	b.burrow.solve(&b.amphipod_locs).0
}
// end::pt2[]

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_pt1() {
		assert_eq!(pt1(), 16157);
	}

	#[test]
	fn test_pt2() {
		assert_eq!(pt2(), 43481);
	}
}
