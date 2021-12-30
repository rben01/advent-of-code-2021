// tag::setup[]
use crate::Answer;
use std::{borrow::Borrow, collections::BTreeSet};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

type Triple = [i32; 3];

#[derive(Debug, Clone, Copy)]
struct Matrix3([i32; 9]);

impl Matrix3 {
	fn from_cols(x: impl Into<Triple>, y: impl Into<Triple>, z: impl Into<Triple>) -> Self {
		let x = x.into();
		let y = y.into();
		let z = z.into();
		Self([x[0], y[0], z[0], x[1], y[1], z[1], x[2], y[2], z[2]])
	}

	fn row(&self, i: usize) -> Triple {
		[0, 1, 2].map(|j| self.0[3 * i + j])
	}

	fn col(&self, i: usize) -> Triple {
		[0, 1, 2].map(|j| self.0[i + 3 * j])
	}

	fn mat_mul(&self, other: Self) -> Self {
		let mut entries = [0; 9];
		let mut i = 0;
		for r in 0..3 {
			let self_row = self.row(r);
			for c in 0..3 {
				let other_col = other.col(c);
				entries[i] = (0..3).map(|i| self_row[i] * other_col[i]).sum();
				i += 1;
			}
		}
		Self(entries)
	}

	fn mul_vec(&self, col: Triple) -> Triple {
		let mut entries = [0; 3];
		for (r, elem) in entries.iter_mut().enumerate() {
			let self_row = self.row(r);
			*elem = (0..3).map(|i| self_row[i] * col[i]).sum();
		}
		entries
	}
}

impl std::ops::Index<[usize; 2]> for Matrix3 {
	type Output = i32;
	fn index(&self, index: [usize; 2]) -> &Self::Output {
		let [row, col] = index;
		&self.0[3 * row + col]
	}
}

#[derive(Clone, Copy, Debug, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
enum Axis {
	PosX,
	NegX,
	PosY,
	NegY,
	PosZ,
	NegZ,
}

impl From<Axis> for Triple {
	fn from(axis: Axis) -> Self {
		use Axis::*;
		match axis {
			PosX => [1, 0, 0],
			NegX => [-1, 0, 0],
			PosY => [0, 1, 0],
			NegY => [0, -1, 0],
			PosZ => [0, 0, 1],
			NegZ => [0, 0, -1],
		}
	}
}

#[derive(Clone, Copy, Debug, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
enum RotationCcw {
	Zero,
	Quarter,
	Half,
	ThreeQuarters,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Swivel {
	up_face: Axis,
	rotation_about_up: RotationCcw,
}

impl Swivel {
	fn new(up_face: Axis, rotation_about_up: RotationCcw) -> Self {
		Self {
			up_face,
			rotation_about_up,
		}
	}

	fn identity() -> Self {
		Self::new(Axis::PosZ, RotationCcw::Zero)
	}

	fn apply(self, point: Triple) -> Triple {
		use Axis::*;
		use RotationCcw::*;

		let mat_to_bring_face_to_up = match self.up_face {
			PosX => Matrix3::from_cols(PosZ, PosY, NegX),
			NegX => Matrix3::from_cols(NegZ, PosY, PosX),
			PosY => Matrix3::from_cols(PosX, PosZ, NegY),
			NegY => Matrix3::from_cols(PosX, NegZ, PosY),
			PosZ => Matrix3::from_cols(PosX, PosY, PosZ),
			NegZ => {
				// We have a couple options; we pick the one that rotates about the y axis
				// because it's easier to actually rotate my hand that way
				Matrix3::from_cols(NegX, PosY, NegZ)
			}
		};

		let mat_to_rotate_about_up = match self.rotation_about_up {
			Zero => Matrix3::from_cols(PosX, PosY, PosZ),
			Quarter => Matrix3::from_cols(PosY, NegX, PosZ),
			Half => Matrix3::from_cols(NegX, NegY, PosZ),
			ThreeQuarters => Matrix3::from_cols(NegY, PosX, PosZ),
		};

		mat_to_bring_face_to_up
			.mat_mul(mat_to_rotate_about_up)
			.mul_vec(point)
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Translation(Triple);

impl Translation {
	fn identity() -> Self {
		Self([0, 0, 0])
	}

	fn apply(&self, point: Triple) -> Triple {
		let [x, y, z] = point;
		let [tx, ty, tz] = self.0;
		[x + tx, y + ty, z + tz]
	}

	fn manhattan_dist(&self) -> u32 {
		let [x, y, z] = self.0;
		u32::try_from(x.abs() + y.abs() + z.abs()).unwrap()
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Transform {
	swivel: Swivel,
	translation: Translation,
}

impl From<Swivel> for Transform {
	fn from(swivel: Swivel) -> Self {
		Self {
			swivel,
			translation: Translation([0, 0, 0]),
		}
	}
}

impl From<Translation> for Transform {
	fn from(translation: Translation) -> Self {
		Self {
			swivel: Swivel::new(Axis::PosZ, RotationCcw::Zero),
			translation,
		}
	}
}

impl Transform {
	fn identity() -> Self {
		Self {
			swivel: Swivel::identity(),
			translation: Translation::identity(),
		}
	}

	fn apply(&self, point: Triple) -> Triple {
		self.translation.apply(self.swivel.apply(point))
	}
}

#[derive(Debug, Clone)]
struct Scanner {
	beacons: BTreeSet<Triple>,
}

impl Scanner {
	fn new(beacons: BTreeSet<Triple>) -> Self {
		Self { beacons }
	}

	fn merged_with(&self, other: &Self) -> Self {
		Self {
			beacons: self.beacons.union(&other.beacons).copied().collect(),
		}
	}

	fn applying<T: Into<Transform> + Copy>(&self, transform: T) -> Self {
		let transform = transform.into();
		Scanner::new(self.beacons.iter().map(|&p| transform.apply(p)).collect())
	}

	fn candidates_for_merge(&self, other: &Scanner) -> Vec<(Transform, Scanner)> {
		let mut transforms = BTreeSet::new();
		let mut candidates = vec![];

		for up_face in Axis::iter() {
			for rotation in RotationCcw::iter() {
				let swivel = Swivel::new(up_face, rotation);
				let swiveled_other = other.applying(swivel);

				for swiveled_other_beacon in &swiveled_other.beacons {
					for this_beacon in &self.beacons {
						let translation = Translation(
							[0, 1, 2].map(|i| this_beacon[i] - swiveled_other_beacon[i]),
						);
						let transformed_other = swiveled_other.applying(translation);

						if self
							.beacons
							.intersection(&transformed_other.beacons)
							.count() >= 12
						{
							let transform = Transform {
								swivel,
								translation,
							};
							let wasnt_present = transforms.insert(transform);
							if wasnt_present {
								candidates.push((transform, transformed_other));
							}
						}
					}
				}
			}
		}

		candidates
	}

	fn merge_all<S: Borrow<Scanner>, V: AsRef<[S]>>(
		scanners: V,
	) -> Option<(Vec<Transform>, Scanner)> {
		struct MergedScanner {
			index: usize,
			merged: Scanner,
			transform: Transform,
		}

		let scanners = scanners.as_ref();

		let (first, rest) = scanners.split_first().unwrap();
		let first: &Scanner = first.borrow();
		if rest.is_empty() {
			return Some((vec![Transform::identity()], first.clone()));
		}

		rest.iter()
			.enumerate()
			.flat_map(|(i, s)| {
				let s = s.borrow();
				first
					.candidates_for_merge(s)
					.iter()
					.map(|(transform, transformed_scanner)| MergedScanner {
						index: i,
						merged: first.merged_with(transformed_scanner),
						transform: *transform,
					})
					.collect::<Vec<_>>()
			})
			.find_map(
				|MergedScanner {
				     index,
				     merged,
				     transform,
				 }| {
					let new_scanners = std::iter::once(&merged)
						.chain(rest.iter().enumerate().filter_map(|(j, s)| {
							if index == j {
								None
							} else {
								Some(s.borrow())
							}
						}))
						.collect::<Vec<_>>();

					Scanner::merge_all(new_scanners).map(|(transforms, ans)| {
						(std::iter::once(transform).chain(transforms).collect(), ans)
					})
				},
			)
	}
}

fn read_input(s: &str) -> Option<Vec<Scanner>> {
	let mut scanners = vec![];
	let mut this_scanner_beacons = BTreeSet::new();
	for line in s.lines().chain(std::iter::once("")) {
		if line.starts_with("---") {
			continue;
		}

		if line.is_empty() {
			scanners.push(Scanner::new(this_scanner_beacons.clone()));
			this_scanner_beacons.clear();
		} else {
			let mut comps = line.split(',');
			let [x, y, z] = [0; 3].map(|_| comps.next()?.parse().ok());
			let [x, y, z] = [x?, y?, z?];
			this_scanner_beacons.insert([x, y, z]);
		}
	}

	Some(scanners)
}

pub fn ans() -> Answer<usize, u32> {
	let scanners = read_input(include_str!("input.txt")).unwrap();
	let (transforms, scanner) = Scanner::merge_all(scanners).unwrap();
	let translations = transforms.iter().map(|t| t.translation).collect::<Vec<_>>();

	(19, (pt1(&scanner), pt2(&translations))).into()
}
// end::setup[]

// tag::pt1[]
fn pt1(scanner: &Scanner) -> usize {
	scanner.beacons.len()
}
// end::pt1[]

// tag::pt2[]
fn pt2<V: AsRef<[Translation]>>(translations: V) -> u32 {
	let translations = translations.as_ref();
	let mut max_manh_dist = u32::MIN;
	for (i, translation1) in translations.iter().enumerate() {
		let [x1, y1, z1] = translation1.0;
		for translation2 in translations.iter().skip(i) {
			let [x2, y2, z2] = translation2.0;

			let diff = Translation([x2 - x1, y2 - y1, z2 - z1]);
			max_manh_dist = max_manh_dist.max(diff.manhattan_dist());
		}
	}

	max_manh_dist
}
// end::pt2[]
