// tag::setup[]
use crate::Answer;

type Players = [Player; 2];

fn read_input(s: &str) -> Option<Players> {
	s.lines()
		.map(|line| {
			line.split(':')
				.nth_back(0)?
				.trim()
				.parse()
				.ok()
				.map(Player::new)
		})
		.collect::<Option<Vec<_>>>()?
		.try_into()
		.ok()
}

struct Board {
	size: usize,
}

#[derive(Debug, Clone, Copy)]
struct Player {
	score: usize,
	position: usize,
}

impl Player {
	fn new(pos: usize) -> Self {
		Self {
			score: 0,
			position: pos,
		}
	}

	fn roll_value(&mut self, board: &Board, value: usize) {
		let new_pos = ((self.position + value - 1) % board.size) + 1;
		self.position = new_pos;
		self.score += new_pos;
	}

	fn roll_deterministic_die(&mut self, board: &Board, die: &mut DeterministicDie) {
		let move_dist = (0..3).map(|_| die.roll()).sum::<usize>();
		self.roll_value(board, move_dist);
	}
}

#[derive(Clone, Copy)]
struct DeterministicDie {
	next: usize,
	min: usize,
	max: usize,
	n_rolls: usize,
}

impl DeterministicDie {
	fn new(min: usize, max: usize) -> Self {
		Self {
			next: 1,
			min,
			max,
			n_rolls: 0,
		}
	}

	fn roll(&mut self) -> usize {
		if self.next > self.max {
			self.next = self.min;
		}
		let ret = self.next;
		self.next += 1;
		self.n_rolls += 1;
		ret
	}
}

fn ans_for_input(input: &str) -> Answer<usize, usize> {
	let board = Board { size: 10 };
	let players = read_input(input).unwrap();

	(
		21,
		(
			pt1(players, &board, DeterministicDie::new(1, 10)),
			pt2(players),
		),
	)
		.into()
}

pub fn ans() -> Answer<usize, usize> {
	ans_for_input(include_str!("input.txt"))
}
// end::setup[]

// tag::pt1[]
fn pt1(mut players: Players, board: &Board, die: DeterministicDie) -> usize {
	let mut die = die;
	let mut loser = players[1];

	'game: loop {
		for player in &mut players {
			player.roll_deterministic_die(board, &mut die);

			if player.score >= 1000 {
				break 'game;
			}
			loser = *player;
		}
	}

	die.n_rolls * loser.score
}
// end::pt1[]

// tag::pt2[]
#[derive(Debug)]
struct Turn {
	players: Players,
	is_p0s_turn: bool,
	n_ways: usize,
}

fn play_quantum_dice(
	players: Players,
	board: &Board,
	die_n_faces: usize,
	n_rolls_per_turn: usize,
	win_threshold: usize,
) -> [usize; 2] {
	let mut tally = [0; 2];
	let mut turn_stack = vec![Turn {
		players,
		is_p0s_turn: true,
		n_ways: 1,
	}];

	// Pairs of `(dice sum, # ways)`
	let outcome_counts: [(usize, usize); 7] = {
		let max_sum = die_n_faces * n_rolls_per_turn;
		let mut counts = vec![0; max_sum + 1];

		for r1 in 1..=die_n_faces {
			for r2 in 1..=die_n_faces {
				for r3 in 1..=die_n_faces {
					counts[r1 + r2 + r3] += 1;
				}
			}
		}

		counts
			.iter()
			.enumerate()
			.filter_map(|(i, &c)| if c > 0 { Some((i, c)) } else { None })
			.collect::<Vec<_>>()
			.try_into()
			.unwrap()
	};

	while let Some(Turn {
		players,
		is_p0s_turn,
		n_ways: n_ways_to_have_gotten_here,
	}) = turn_stack.pop()
	{
		let player_index = if is_p0s_turn { 0 } else { 1 };

		for (roll_value, n_ways_to_roll_this) in outcome_counts {
			let n_ways_to_get_here = n_ways_to_have_gotten_here * n_ways_to_roll_this;

			let mut player = players[player_index];
			player.roll_value(board, roll_value);

			if player.score >= win_threshold {
				tally[player_index] += n_ways_to_get_here;
			} else {
				let mut players = players;
				players[player_index] = player;
				turn_stack.push(Turn {
					players,
					is_p0s_turn: !is_p0s_turn,
					n_ways: n_ways_to_get_here,
				});
			}
		}
	}

	tally
}

fn pt2(players: Players) -> usize {
	let [p1_n_wins, p2_n_wins] = play_quantum_dice(players, &Board { size: 10 }, 3, 3, 21);
	p1_n_wins.max(p2_n_wins)
}
// end::pt2[]

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_input;

	#[test]
	fn test() {
		test_input!(include_str!("input.txt"), day: 21, ans: (757_770, 712_381_680_443_927));
	}
}
