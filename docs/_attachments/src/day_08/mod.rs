// tag::setup[]
use crate::Answer;
use std::collections::{btree_map::Entry as MapEntry, BTreeMap as Map, BTreeSet as Set};

const N_SEGMENTS: usize = 7;
// end::setup[]

// tag::digit[]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Digit {
	segments: [bool; N_SEGMENTS],
	n_on: usize,
}

impl Digit {
	fn new(segments: [bool; N_SEGMENTS]) -> Self {
		Self {
			segments,
			n_on: segments.iter().filter(|&&b| b).count(),
		}
	}

	fn from_str(s: &str) -> Self {
		let mut segments = [false; N_SEGMENTS];
		for c in s.bytes() {
			let i = c - b'a';
			segments[usize::from(i)] = true;
		}
		Self::new(segments)
	}

	fn _bin_op(self, rhs: Self, f: impl Fn(bool, bool) -> bool) -> Self {
		Self::new(self.segments.zip(rhs.segments).map(|(x, y)| f(x, y)))
	}
}

impl std::ops::BitOr for Digit {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self::Output {
		self._bin_op(rhs, |x, y| x | y)
	}
}

impl std::ops::BitAnd for Digit {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self::Output {
		self._bin_op(rhs, |x, y| x & y)
	}
}

impl std::ops::Not for Digit {
	type Output = Self;
	fn not(self) -> Self::Output {
		Self::new(self.segments.map(|b| !b))
	}
}

impl From<usize> for Digit {
	fn from(n: usize) -> Self {
		let segments = match n {
			/*
			-----[A, B, C, D, E, F, G]
			*/
			0 => [1, 1, 1, 0, 1, 1, 1],
			1 => [0, 0, 1, 0, 0, 1, 0],
			2 => [1, 0, 1, 1, 1, 0, 1],
			3 => [1, 0, 1, 1, 0, 1, 1],
			4 => [0, 1, 1, 1, 0, 1, 0],
			5 => [1, 1, 0, 1, 0, 1, 1],
			6 => [1, 1, 0, 1, 1, 1, 1],
			7 => [1, 0, 1, 0, 0, 1, 0],
			8 => [1, 1, 1, 1, 1, 1, 1],
			9 => [1, 1, 1, 1, 0, 1, 1],
			_ => panic!("Cannot make digit for n={}", n),
		};
		let segments = segments.map(|i| i != 0);
		Self::new(segments)
	}
}

impl From<Digit> for usize {
	fn from(digit: Digit) -> Self {
		let segments = digit.segments.map(u8::from);
		match segments {
			/*
			[A, B, C, D, E, F, G]
			*/
			[1, 1, 1, 0, 1, 1, 1] => 0,
			[0, 0, 1, 0, 0, 1, 0] => 1,
			[1, 0, 1, 1, 1, 0, 1] => 2,
			[1, 0, 1, 1, 0, 1, 1] => 3,
			[0, 1, 1, 1, 0, 1, 0] => 4,
			[1, 1, 0, 1, 0, 1, 1] => 5,
			[1, 1, 0, 1, 1, 1, 1] => 6,
			[1, 0, 1, 0, 0, 1, 0] => 7,
			[1, 1, 1, 1, 1, 1, 1] => 8,
			[1, 1, 1, 1, 0, 1, 1] => 9,
			_ => panic!("Digit {:?} is not valid", digit),
		}
	}
}
// end::digit[]

// tag::setup[]
fn get_mapping_from_garbled_digits<D: std::borrow::Borrow<Digit>>(
	garbled_digits: impl Iterator<Item = D>,
) -> Result<Map<Digit, Digit>, Map<Digit, Set<Digit>>> {
	let mut mappings = Map::new();

	{
		let mut grouped_by_n_on = Map::new();
		for n in 0..=9 {
			let digit = Digit::from(n);
			grouped_by_n_on
				.entry(digit.n_on)
				.or_insert_with(Set::new)
				.insert(digit);
		}

		for gd in garbled_digits {
			let gd = *gd.borrow();
			let digits_w_same_n_segments = &grouped_by_n_on[&gd.n_on];
			mappings.insert(gd, digits_w_same_n_segments.clone());
		}
	}

	let identity: &dyn Fn(Digit) -> _ = &(|x| x);
	let bitwise_not: &dyn Fn(Digit) -> _ = &(|x| !x);

	loop {
		let mut new_mappings = Map::new();

		for (i, (&garbled1, choices1)) in mappings.iter().enumerate() {
			for (&garbled2, choices2) in mappings.iter().skip(i + 1) {
				for (op1, op2) in [
					(identity, identity),
					(identity, bitwise_not),
					(bitwise_not, identity),
				] {
					let new_garbled = op1(garbled1) & op2(garbled2);

					if new_garbled.n_on == 0 {
						continue;
					}

					let mut new_good_candidates = Set::new();
					for &good_digit1 in choices1 {
						for &good_digit2 in choices2 {
							let candidate = op1(good_digit1) & op2(good_digit2);
							if candidate.n_on == new_garbled.n_on {
								new_good_candidates.insert(candidate);
							}
						}
					}

					match new_mappings.entry(new_garbled) {
						MapEntry::Vacant(v) => {
							v.insert(new_good_candidates);
						}
						MapEntry::Occupied(mut o) => {
							o.insert(o.get() & &new_good_candidates);
						}
					}
				}
			}
		}

		// Remove all keys that can be written as the disjoint-bitwise-or of two other
		// keys, as they're redundant. This means if e.g., A and BC are present, then
		// remove ABC. But if only AB and BC are present, then do *not* remove ABC (as AB
		// and BC are not disjoint)
		let mut redundant_keys = Set::new();
		let new_garbled_keys = new_mappings.keys().copied().collect::<Set<_>>();
		for (i, &garbled1) in new_garbled_keys.iter().enumerate() {
			for &garbled2 in new_garbled_keys.iter().skip(i + 1) {
				if (garbled1 & garbled2).n_on != 0 {
					continue;
				}
				let segment_union = garbled1 | garbled2;
				if new_garbled_keys.contains(&segment_union) {
					redundant_keys.insert(segment_union);
				}
			}
		}

		for k in &redundant_keys {
			new_mappings.remove(k);
		}

		if mappings.len() == N_SEGMENTS && mappings.values().all(|m| m.len() == 1) {
			return Ok(mappings
				.into_iter()
				.map(|(k, v)| (k, v.iter().next().copied().unwrap()))
				.collect());
		} else if mappings == new_mappings {
			return Err(mappings);
		}

		mappings = new_mappings;
	}
}

fn apply_mapping_to_garbled_digit(mapping: &Map<Digit, Digit>, garbled_digit: Digit) -> usize {
	let mut result = Digit::new([false; 7]);
	for (&k, &v) in mapping {
		if (garbled_digit & k).n_on > 0 {
			result = result | v;
		}
	}
	result.into()
}

fn read_input(input: &str) -> Vec<(Vec<Digit>, Vec<Digit>)> {
	fn whitespace_sepd_strs_to_digits(strs: &str) -> Vec<Digit> {
		strs.trim()
			.split_ascii_whitespace()
			.map(Digit::from_str)
			.collect()
	}
	input
		.lines()
		.filter_map(|line| {
			let line = line.trim();
			if line.is_empty() {
				return None;
			}
			let mut in_out = line.split('|');
			let in_digits = whitespace_sepd_strs_to_digits(in_out.next()?);
			let out_digits = whitespace_sepd_strs_to_digits(in_out.next()?);
			Some((in_digits, out_digits))
		})
		.collect()
}

fn translate_line_to_digits<D: std::borrow::Borrow<Digit>>(
	idod: (impl Iterator<Item = D>, impl Iterator<Item = D>),
) -> Option<Vec<usize>> {
	let (in_digits, out_digits) = idod;

	let mapping = get_mapping_from_garbled_digits(in_digits).ok()?;
	Some(
		out_digits
			.map(|d| apply_mapping_to_garbled_digit(&mapping, *d.borrow()))
			.collect(),
	)
}

fn ans_for_input(input: &str) -> Answer<usize, usize> {
	let in_out_lines = read_input(input);

	let output_digits = in_out_lines
		.iter()
		.map(|(in_d, out_d)| translate_line_to_digits((in_d.iter(), out_d.iter())))
		.collect::<Option<Vec<_>>>()
		.unwrap();
	(8, (pt1(output_digits.iter()), pt2(output_digits.iter()))).into()
}

pub fn ans() -> Answer<usize, usize> {
	ans_for_input(include_str!("input.txt"))
}
// end::setup[]

// tag::pt1[]
fn pt1<Nums: AsRef<[usize]>>(out_digits: impl Iterator<Item = Nums>) -> usize {
	out_digits
		.map(|v| {
			v.as_ref()
				.iter()
				.filter(|&n| [1, 4, 7, 8].contains(n))
				.count()
		})
		.sum()
}
// end::pt1[]

// tag::pt2[]
fn pt2<Nums: AsRef<[usize]>>(out_digits: impl Iterator<Item = Nums>) -> usize {
	out_digits
		.map(|v| {
			v.as_ref()
				.iter()
				.rev()
				.enumerate()
				.map(|(pow10, &val)| val * 10_usize.pow(u32::try_from(pow10).unwrap()))
				.sum::<usize>()
		})
		.sum()
}
// end::pt2[]

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_input;

	#[test]
	fn test() {
		test_input!(include_str!("input.txt"), day: 8, ans: (237, 1_009_098));
	}
}
