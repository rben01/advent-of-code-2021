// tag::setup[]
use crate::Answer;
use std::fmt::{Debug, Display};

// tag::snail_num[]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Elem {
	value: u32,
	depth: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct SnailNum<Elems: AsRef<[Elem]>> {
	elems: Elems,
	depth: usize,
}
// end::snail_num[]
type SnailNumOwned = SnailNum<Vec<Elem>>;
type SnailNumBorrowed<'a> = SnailNum<&'a [Elem]>;

impl SnailNumOwned {
	fn from_line(line: &str) -> Self {
		let mut addends = Vec::<Elem>::new();
		let mut depth = 0;
		let mut prev_was_digit = false;

		for c in line.trim().chars() {
			let mut c_is_digit = false;
			match c {
				'[' => depth += 1,
				']' => depth -= 1,
				'0'..='9' => {
					let digit = c.to_digit(10).unwrap();
					if prev_was_digit {
						let val = addends.last_mut().unwrap();
						val.value = 10 * val.value + digit;
					} else {
						addends.push(Elem {
							value: digit,
							depth,
						});
					}
					c_is_digit = true;
				}
				',' => {}
				_ => {
					panic!("Unexpected character {:?}", c);
				}
			}

			prev_was_digit = c_is_digit;
		}
		SnailNum::owning(addends)
	}

	fn by_adding_lines_in<S: AsRef<str>, I: IntoIterator<Item = S>>(lines: I) -> Self {
		let mut lines = lines.into_iter();
		let mut ans = SnailNumOwned::from_line(lines.next().unwrap().as_ref());
		for line in lines {
			ans = ans.add(&SnailNumOwned::from_line(line.as_ref()));
		}
		ans
	}

	fn from_str(input: &str) -> Self {
		Self::by_adding_lines_in(input.lines())
	}

	fn owning(elems: Vec<Elem>) -> Self {
		Self { elems, depth: 0 }
	}

	fn reduce_once(&mut self) -> bool {
		self.explode_first().is_some() || self.split_first().is_some()
	}

	fn reduce(&mut self) {
		while self.reduce_once() {}
	}

	// Contains explode_first and split_first, which only operate on owned snail nums
	// tag::explode[]
	/// Explodes the first explode-able pair in the list. \
	/// Returns the `Option` pair of `Option` indices of the elements to the left and right
	/// (the ones being modified), in that order. `None` means there was no pair to explode;
	/// `(None, Some(2))` means the exploded pair had no elements to the left of it (it
	/// was at index 0), and the element to its right was added to
	fn explode_first(&mut self) -> Option<(Option<usize>, Option<usize>)> {
		let elems = &mut self.elems;

		let ((l_idx, l_elem), (r_idx, r_elem)) = elems
			.iter()
			.enumerate()
			.zip(elems.iter().enumerate().skip(1))
			.find_map(|((i1, &e1), (i2, &e2))| {
				if e1.depth > 4 && e1.depth == e2.depth {
					Some(((i1, e1), (i2, e2)))
				} else {
					None
				}
			})?;

		elems[l_idx] = Elem {
			value: 0,
			depth: l_elem.depth - 1,
		};

		let changed_l_idx = if l_idx > 0 {
			let idx = l_idx - 1;
			elems[idx].value += l_elem.value;
			Some(idx)
		} else {
			None
		};

		elems.remove(r_idx);

		let changed_r_idx = if r_idx < elems.len() {
			elems[r_idx].value += r_elem.value;
			Some(r_idx)
		} else {
			None
		};

		Some((changed_l_idx, changed_r_idx))
	}
	// end::explode[]
	// tag::split[]
	/// Splits the first splittable pair in the list. \
	/// Returns the index at which the split occurred, which is now the first element of the
	/// resulting pair
	fn split_first(&mut self) -> Option<usize> {
		let elems = &mut self.elems;

		let (split_idx, Elem { value, depth }) =
			elems.iter().enumerate().find_map(|(i, &elem)| {
				if elem.value >= 10 {
					Some((i, elem))
				} else {
					None
				}
			})?;

		let new_l_value = value / 2;
		let new_r_value = (value + 1) / 2;

		let new_elem = Elem {
			value: new_l_value,
			depth: depth + 1,
		};
		elems[split_idx] = new_elem;

		elems.insert(
			split_idx + 1,
			Elem {
				value: new_r_value,
				depth: new_elem.depth,
			},
		);

		Some(split_idx)
	}
	// end::split[]
}

impl<'a> SnailNumBorrowed<'a> {
	fn borrowing(elems: &'a [Elem], base_depth: usize) -> Self {
		Self {
			elems,
			depth: base_depth,
		}
	}
}

impl<E: AsRef<[Elem]>> SnailNum<E> {
	// Contains add, as_pair, and magnitude
	// tag::debugging[]
	#[cfg(test)]
	fn reduced(&self) -> SnailNumOwned {
		let mut sn = SnailNumOwned::owning(self.elems.as_ref().to_owned());
		sn.reduce();
		sn
	}
	// end::debugging[]
	// tag::add[]
	fn add(&self, other: &Self) -> SnailNumOwned {
		let elems = self
			.elems
			.as_ref()
			.iter()
			.chain(other.elems.as_ref())
			.map(|&Elem { value, depth }| Elem {
				value,
				depth: depth + 1,
			})
			.collect();

		let mut ans = SnailNumOwned::owning(elems);
		ans.reduce();
		ans
	}
	// end::add[]
	// tag::pair[]
	fn as_pair(&self) -> Result<(SnailNumBorrowed, SnailNumBorrowed), u32> {
		let elems = self.elems.as_ref();
		assert_ne!(elems.len(), 0, "{}", self.depth);

		if elems.len() == 1 {
			return Err(self.elems.as_ref()[0].value);
		}

		let mut depth_stack = vec![];
		for (i, &Elem { depth, .. }) in elems.iter().enumerate() {
			depth_stack.push(depth);

			loop {
				match depth_stack.pop() {
					None => break,
					Some(curr_depth) => {
						if curr_depth == self.depth + 1 {
							let (left, right) = elems.split_at(i + 1);

							return Ok((
								SnailNum::borrowing(left, curr_depth),
								SnailNum::borrowing(right, curr_depth),
							));
						}

						match depth_stack.pop() {
							None => {
								depth_stack.push(curr_depth);
								break;
							}
							Some(prev_depth) => {
								if prev_depth == curr_depth {
									depth_stack.push(curr_depth - 1);
								} else {
									depth_stack.extend([prev_depth, curr_depth]);
									break;
								}
							}
						}
					}
				}
			}
		}
		unreachable!()
	}
	// end::pair[]
	// tag::magnitude[]
	fn magnitude(&self) -> u32 {
		match self.as_pair() {
			Ok((left, right)) => 3 * left.magnitude() + 2 * right.magnitude(),
			Err(val) => val,
		}
	}
	// end::magnitude[]
}

// tag::debugging[]
impl<E: AsRef<[Elem]>> Display for SnailNum<E> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.elems.as_ref().is_empty() {
			return Ok(());
		}
		match self.as_pair() {
			Ok((left, right)) => {
				write!(f, "[{},{}]", left, right)
			}
			Err(value) => {
				write!(f, "{}", value)
			}
		}
	}
}

// end::debugging[]
fn ans_for_input(input: &str) -> Answer<u32, u32> {
	let snail_num = SnailNumOwned::from_str(input);
	(18, (pt1(&snail_num), pt2(input))).into()
}

pub fn ans() -> Answer<u32, u32> {
	ans_for_input(include_str!("input.txt"))
}
// end::setup[]

// tag::pt1[]
fn pt1<E: AsRef<[Elem]>>(snail_num: &SnailNum<E>) -> u32 {
	snail_num.magnitude()
}
// end::pt1[]

// tag::pt2[]
fn pt2(input: &str) -> u32 {
	let mut max_mag = u32::MIN;
	let snail_nums = input
		.lines()
		.map(SnailNumOwned::from_line)
		.collect::<Vec<_>>();

	for (i, sn1) in snail_nums.iter().enumerate() {
		for sn2 in snail_nums.iter().skip(i + 1) {
			let mag1 = sn1.add(sn2).magnitude();
			let mag2 = sn2.add(sn1).magnitude();

			max_mag = max_mag.max(mag1).max(mag2);
		}
	}

	max_mag
}
// end::pt2[]

#[cfg(test)]
mod test {
	use super::*;

	#[track_caller]
	fn test_action<
		V: AsRef<[(u32, usize)]>,
		T: Eq + std::fmt::Debug,
		F: Fn(&mut SnailNumOwned) -> T,
	>(
		input: &str,
		before: V,
		action: F,
		after: &str,
		result: &T,
	) {
		let mut snail_num = SnailNumOwned::from_line(input);

		assert_eq!(
			snail_num,
			SnailNumOwned::owning(
				before
					.as_ref()
					.iter()
					.map(|&(value, depth)| Elem { value, depth })
					.collect::<Vec<_>>()
			)
		);
		assert_eq!(result, &action(&mut snail_num));
		assert_eq!(snail_num, SnailNumOwned::from_line(after));
	}

	#[track_caller]
	fn test_explode<V: AsRef<[(u32, usize)]>>(
		input: &str,
		before: V,
		indices: Option<(Option<usize>, Option<usize>)>,
		after: &str,
	) {
		test_action(input, before, SnailNumOwned::explode_first, after, &indices);
	}

	#[track_caller]
	fn test_split<V: AsRef<[(u32, usize)]>>(
		input: &str,
		before: V,
		index: Option<usize>,
		after: &str,
	) {
		test_action(input, before, SnailNumOwned::split_first, after, &index);
	}

	#[test]
	fn test_explodes() {
		test_explode(
			"[[[[[9,8],1],2],3],4]",
			vec![(9, 5), (8, 5), (1, 4), (2, 3), (3, 2), (4, 1)],
			Some((None, Some(1))),
			"[[[[0,9],2],3],4]",
		);

		test_explode(
			"[7,[6,[5,[4,[3,2]]]]]",
			vec![(7, 1), (6, 2), (5, 3), (4, 4), (3, 5), (2, 5)],
			Some((Some(3), None)),
			"[7,[6,[5,[7,0]]]]",
		);

		test_explode(
			"[[6,[5,[4,[3,2]]]],1]",
			vec![(6, 2), (5, 3), (4, 4), (3, 5), (2, 5), (1, 1)],
			Some((Some(2), Some(4))),
			"[[6,[5,[7,0]]],3]",
		);

		test_explode(
			"[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
			vec![
				(3, 2),
				(2, 3),
				(1, 4),
				(7, 5),
				(3, 5),
				(6, 2),
				(5, 3),
				(4, 4),
				(3, 5),
				(2, 5),
			],
			Some((Some(2), Some(4))),
			"[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
		);
	}

	#[test]
	fn test_splits() {
		test_split(
			"[[[[0,7],4],[15,[0,13]]],[1,1]]",
			vec![
				(0, 4),
				(7, 4),
				(4, 3),
				(15, 3),
				(0, 4),
				(13, 4),
				(1, 2),
				(1, 2),
			],
			Some(3),
			"[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
		);

		test_split(
			"[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
			vec![
				(0, 4),
				(7, 4),
				(4, 3),
				(7, 4),
				(8, 4),
				(0, 4),
				(13, 4),
				(1, 2),
				(1, 2),
			],
			Some(6),
			"[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
		);
	}

	#[test]
	fn test_reduction() {
		assert_eq!(
			SnailNumOwned::by_adding_lines_in(vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]"]).reduced(),
			SnailNumOwned::from_line("[[[[1,1],[2,2]],[3,3]],[4,4]]")
		);
		assert_eq!(
			SnailNumOwned::by_adding_lines_in(vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]"])
				.reduced(),
			SnailNumOwned::from_line("[[[[3,0],[5,3]],[4,4]],[5,5]]")
		);
		assert_eq!(
			SnailNumOwned::by_adding_lines_in(vec![
				"[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"
			])
			.reduced(),
			SnailNumOwned::from_line("[[[[5,0],[7,4]],[5,5]],[6,6]]")
		);

		assert_eq!(
			SnailNumOwned::from_line("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]").reduced(),
			SnailNumOwned::from_line("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
		);

		assert_eq!(
			SnailNumOwned::by_adding_lines_in(vec![
				"[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
				"[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
				"[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
				"[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
				"[7,[5,[[3,8],[1,4]]]]",
				"[[2,[2,2]],[8,[8,1]]]",
				"[2,9]",
				"[1,[[[9,3],9],[[9,0],[0,7]]]]",
				"[[[5,[7,4]],7],1]",
				"[[[[4,2],2],6],[8,7]]"
			],)
			.reduced(),
			SnailNumOwned::from_line("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
		);
	}

	#[test]
	fn test() {
		assert_eq!(pt1(&SnailNumOwned::from_line("[[1,2],[[3,4],5]]")), 143);
		assert_eq!(
			pt1(&SnailNumOwned::from_line(
				"[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
			)),
			1384
		);
		assert_eq!(
			pt1(&SnailNumOwned::from_line("[[[[1,1],[2,2]],[3,3]],[4,4]]")),
			445
		);
	}
}
