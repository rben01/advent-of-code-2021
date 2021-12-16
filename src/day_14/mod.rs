// tag::setup[]
use crate::Answer;
use std::collections::BTreeMap as Map;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Position {
	Start,
	Middle,
	End,
}

#[derive(Debug)]
struct Polymer {
	template: String,
	mapping: Map<(char, char), char>,
}

impl Polymer {
	fn from_str(s: &str) -> Option<Self> {
		let mut lines = s.lines();
		let template = lines.next()?.to_owned();
		lines.next()?;

		let mut mapping = Map::new();
		for line in lines {
			let mut splat = line.split(" -> ");

			let mut outer_chars = splat.next()?.chars();
			let left = outer_chars.next()?;
			let right = outer_chars.next()?;

			let inner = splat.next()?.chars().next()?;
			mapping.insert((left, right), inner);
		}

		Some(Polymer { template, mapping })
	}

	fn get_initial_char_pair_counts(&self) -> Map<(char, char, Position), usize> {
		use Position::*;

		let mut ans = Map::new();

		// Iterate over adjacent pairs of chars. The last index iterated is `n_pairs - 1`
		// with `n_pairs == self.template.len() - 1`
		for (i, (c1, c2)) in self
			.template
			.chars()
			.zip(self.template.chars().skip(1))
			.enumerate()
		{
			let position = if i == 0 {
				Start
			} else if i == self.template.len() - 2 {
				End
			} else {
				Middle
			};

			*ans.entry((c1, c2, position)).or_default() += 1;
		}
		ans
	}

	fn apply_n_times(&self, n: usize) -> Map<(char, char, Position), usize> {
		use Position::*;

		let mut pair_counts = self.get_initial_char_pair_counts();

		for _ in 0..n {
			let pair_counts_vec = pair_counts
				.iter()
				.filter_map(|(&k, &v)| if v > 0 { Some((k, v)) } else { None })
				.collect::<Vec<_>>();

			for (key, count) in pair_counts_vec {
				let (c1, c2, position) = key;
				if let Some(&c) = self.mapping.get(&(c1, c2)) {
					let first_pos = match position {
						Start => Start,
						_ => Middle,
					};
					let second_pos = match position {
						End => End,
						_ => Middle,
					};

					*pair_counts.entry((c1, c, first_pos)).or_default() += count;
					*pair_counts.entry((c, c2, second_pos)).or_default() += count;

					*pair_counts.get_mut(&key).unwrap() -= count;
				}
			}
		}

		pair_counts
	}
}

fn get_ans(polymer: &Polymer, n: usize) -> usize {
	use Position::*;

	let char_pair_counts = polymer.apply_n_times(n);
	let char_counts = {
		let mut char_counts_2x = Map::new();
		for ((c1, c2, position), count) in char_pair_counts {
			let c1_multiplier = match position {
				Start => 2,
				_ => 1,
			};
			let c2_multiplier = match position {
				End => 2,
				_ => 1,
			};

			for (c, mult) in [(c1, c1_multiplier), (c2, c2_multiplier)] {
				*char_counts_2x.entry(c).or_insert(0) += mult * count;
			}
		}

		char_counts_2x
			.into_iter()
			.map(|(k, v)| (k, v / 2))
			.collect::<Map<_, _>>()
	};

	let mut max_count = 0;
	let mut min_count = usize::MAX;
	for &count in char_counts.values() {
		if count < min_count {
			min_count = count;
		}
		if count > max_count {
			max_count = count;
		}
	}

	max_count - min_count
}

pub fn ans() -> Answer<usize, usize> {
	let input = include_str!("./input.txt");
	let polymer = Polymer::from_str(input).unwrap();
	(14, (pt1(&polymer), pt2(&polymer))).into()
}
// end::setup[]

// tag::pt1[]
fn pt1(polymer: &Polymer) -> usize {
	get_ans(polymer, 10)
}
// end::pt1[]

// tag::pt2[]
fn pt2(polymer: &Polymer) -> usize {
	get_ans(polymer, 40)
}
// end::pt2[]
