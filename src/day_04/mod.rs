// tag::setup[]
use crate::Answer;
use num::{integer::div_mod_floor, Integer};
use std::{
	collections::{BTreeMap as Map, BTreeSet as Set},
	str::FromStr,
};

#[derive(Debug)]
struct BoardProgress {
	rows: Vec<usize>,
	cols: Vec<usize>,
	has_won: bool,
}

impl BoardProgress {
	fn new(n_rows: usize, n_cols: usize) -> Self {
		let rows = vec![n_cols; n_rows];
		let cols = vec![n_rows; n_cols];
		Self {
			rows,
			cols,
			has_won: false,
		}
	}

	fn handle_entry(&mut self, row: usize, col: usize) {
		if self.has_won {
			return;
		}

		self.rows[row] -= 1;
		self.cols[col] -= 1;

		if self.rows[row] == 0 || self.cols[col] == 0 {
			self.has_won = true;
		}
	}
}

#[derive(Debug)]
struct Board<T: Integer> {
	grid: Map<T, (usize, usize)>,
	progress: BoardProgress,
}

impl<T: Integer + std::iter::Sum + Copy> Board<T> {
	fn new(nums: &[T], n_cols: usize) -> Self {
		let n_rows = nums.len() / n_cols;
		assert_eq!(n_rows * n_cols, nums.len());

		let mut grid = Map::new();
		for (i, &x) in nums.iter().enumerate() {
			let (r, c) = div_mod_floor(i, n_cols);
			grid.insert(x, (r, c));
		}
		Self {
			grid,
			progress: BoardProgress::new(n_rows, n_cols),
		}
	}

	fn play_number(&mut self, n: T) {
		let (r, c) = match self.grid.remove(&n) {
			Some(coords) => coords,
			None => return,
		};
		self.progress.handle_entry(r, c);
	}

	fn has_won(&self) -> bool {
		self.progress.has_won
	}

	fn get_ans(&self, winning_num: T) -> T {
		let unmarked_sum = self.grid.keys().copied().sum::<T>();
		winning_num * unmarked_sum
	}
}

struct Game<T: Integer> {
	boards: Vec<Board<T>>,
	numbers: Vec<T>,
}

impl<T: Integer + std::iter::Sum + Copy + FromStr + std::fmt::Debug> Game<T> {
	fn from_str(s: &str) -> Option<Self> {
		let mut lines = s.lines().chain(std::iter::once(""));
		let nums = lines
			.next()?
			.split(',')
			.map(|s| s.parse::<T>().ok())
			.collect::<Option<Vec<_>>>()?;

		let mut boards = vec![];
		let mut this_board = vec![];

		let mut n_cols = None;

		for line in lines {
			if line.is_empty() {
				if !this_board.is_empty() {
					let board = Board::new(this_board.as_slice(), n_cols.unwrap());
					boards.push(board);
					this_board.clear();
				}
			} else {
				for num in line.split_whitespace().map(|s| s.parse::<T>().ok()) {
					let num = num?;
					this_board.push(num);
				}
				if matches!(n_cols, None) {
					n_cols = Some(this_board.len());
				}
			}
		}

		Some(Self {
			boards,
			numbers: nums,
		})
	}
}

fn ans_for_input(input: &str) -> Answer<i32, i32> {
	let [game1, game2] = [0; 2].map(|_| Game::from_str(input).unwrap());
	(4, (pt1(game1), pt2(game2))).into()
}

pub fn ans() -> Answer<i32, i32> {
	ans_for_input(include_str!("input.txt"))
}
// end::setup[]

// tag::pt1[]
fn pt1(mut game: Game<i32>) -> i32 {
	for &num in &game.numbers {
		for board in &mut game.boards {
			board.play_number(num);
			if board.has_won() {
				return board.get_ans(num);
			}
		}
	}
	unreachable!();
}
// end::pt1[]

// tag::pt2[]
fn pt2(mut game: Game<i32>) -> i32 {
	let mut ongoing_game_idxs = (0..game.boards.len()).collect::<Set<_>>();

	for &num in &game.numbers {
		for (board_idx, board) in game.boards.iter_mut().enumerate() {
			let already_won = !ongoing_game_idxs.contains(&board_idx);
			if already_won {
				continue;
			}

			board.play_number(num);
			if board.has_won() {
				if ongoing_game_idxs.len() == 1 {
					return board.get_ans(num);
				}

				ongoing_game_idxs.remove(&board_idx);
			}
		}
	}
	unreachable!();
}
// end::pt2[]

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_input;

	#[test]
	fn test() {
		test_input!(include_str!("sample_input.txt"), day: 4, ans: (4512, 1924));
		test_input!(include_str!("input.txt"), day: 4, ans: (87456, 15561));
	}
}
