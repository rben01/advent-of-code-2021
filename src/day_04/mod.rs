use std::{
	collections::{BTreeMap as Map, BTreeSet as Set},
	str::FromStr,
};

use num::{integer::div_mod_floor, Integer};

use crate::Answer;

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
		self.rows[row] -= 1;
		self.cols[col] -= 1;

		if self.rows[row] == 0 || self.cols[col] == 0 {
			self.has_won = true
		}
	}
}

struct Board<T: Integer> {
	grid: Map<T, (usize, usize)>,
	progress: BoardProgress,
}

impl<T: Integer + Copy> Board<T> {
	fn from(nums: &[T], n_cols: usize) -> Self {
		let n_rows = nums.len() / n_cols;
		assert_eq!(n_rows * n_cols, nums.len());

		let mut grid = Map::new();
		for (i, x) in nums.iter().enumerate() {
			let (r, c) = div_mod_floor(i, n_cols);
			grid.insert(*x, (r, c));
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
		let unmarked_sum = {
			let mut keys_iter = self.grid.keys();
			let mut s = *keys_iter.next().unwrap();
			for k in keys_iter {
				s = s + *k;
			}
			s
		};
		winning_num * unmarked_sum
	}
}

struct Game<T: Integer> {
	boards: Vec<Board<T>>,
	numbers: Vec<T>,
}

impl<T: Integer + Copy + FromStr> Game<T> {
	fn from(s: &str) -> Option<Self> {
		let mut lines = s.lines();
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
					let board = Board::from(this_board.as_slice(), n_cols.unwrap());
					boards.push(board);
					this_board.clear();
				}
			} else {
				for num in line.split_whitespace().map(|s| s.parse::<T>().ok()) {
					let num = num?;
					this_board.push(num)
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

fn pt1() -> i32 {
	let mut game = Game::<i32>::from(include_str!("./input.txt")).unwrap();
	for num in game.numbers.iter() {
		for board in game.boards.iter_mut() {
			board.play_number(*num);
			if board.has_won() {
				return board.get_ans(*num);
			}
		}
	}
	unreachable!();
}

fn pt2() -> i32 {
	let mut game = Game::<i32>::from(include_str!("./input.txt")).unwrap();
	let mut ongoing_game_idxs = Set::from_iter(0..game.boards.len());

	for num in game.numbers.iter() {
		for (board_idx, board) in game.boards.iter_mut().enumerate() {
			let already_won = !ongoing_game_idxs.contains(&board_idx);
			if already_won {
				continue;
			}

			board.play_number(*num);
			if board.has_won() {
				if ongoing_game_idxs.len() == 1 {
					return board.get_ans(*num);
				}

				ongoing_game_idxs.remove(&board_idx);
			}
		}
	}
	unreachable!();
}

pub fn ans() -> Answer<i32, i32> {
	(pt1(), pt2()).into()
}
