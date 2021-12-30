// tag::setup[]
use crate::Answer;
use std::fmt::{Display, Write};

type Span = [i32; 2];

#[derive(Debug, Clone, Copy)]
struct Cuboid {
	x_range: Span,
	y_range: Span,
	z_range: Span,
}

impl Cuboid {
	fn size(&self) -> usize {
		fn width(span: Span) -> usize {
			usize::try_from(span[1] - span[0] + 1).unwrap()
		}
		let &Cuboid {
			x_range,
			y_range,
			z_range,
		} = self;

		width(x_range) * width(y_range) * width(z_range)
	}
}

// tag::debugging[]
impl std::fmt::Display for Cuboid {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let [x0, x1] = &self.x_range;
		let [y0, y1] = &self.y_range;
		let [z0, z1] = &self.z_range;
		write!(f, "{:?}-{:?},{:?}-{:?},{:?}-{:?}", x0, x1, y0, y1, z0, z1)
	}
}

// end::debugging[]
impl Cuboid {
	fn intersection(&self, other: &Self) -> Option<Self> {
		fn span_intersection(span1: Span, span2: Span) -> Option<Span> {
			let lower = span1[0].max(span2[0]);
			let upper = span1[1].min(span2[1]);
			if upper < lower {
				None
			} else {
				Some([lower, upper])
			}
		}

		Some(Self {
			x_range: span_intersection(self.x_range, other.x_range)?,
			y_range: span_intersection(self.y_range, other.y_range)?,
			z_range: span_intersection(self.z_range, other.z_range)?,
		})
	}

	/// `other` divides `self` into 3^3 = 27 (potentially empty) sub-cuboids. Of the 26
	/// that aren't `other`, we keep the nonempty ones. Once we've found them, we merge
	/// them into as large cuboids as possible.
	fn difference(&self, other: &Self) -> Vec<Self> {
		fn get_spans(my_span: Span, intersection_span: Span) -> [Option<Span>; 3] {
			let span1 = if my_span[0] == intersection_span[0] {
				None
			} else {
				Some([my_span[0], intersection_span[0] - 1])
			};

			let span2 = Some(intersection_span);

			let span3 = if intersection_span[1] == my_span[1] {
				None
			} else {
				Some([intersection_span[1] + 1, my_span[1]])
			};

			[span1, span2, span3]
		}

		let intersection = match self.intersection(other) {
			Some(c) => c,
			None => return vec![*self],
		};

		let x_ranges = get_spans(self.x_range, intersection.x_range);
		let y_ranges = get_spans(self.y_range, intersection.y_range);
		let z_ranges = get_spans(self.z_range, intersection.z_range);

		let mut on_cuboids = Vec::<Self>::new();
		for &x_range in x_ranges.iter().flatten() {
			for &y_range in y_ranges.iter().flatten() {
				for &z_range in z_ranges.iter().flatten() {
					if x_range != intersection.x_range
						|| y_range != intersection.y_range
						|| z_range != intersection.z_range
					{
						on_cuboids.push(Cuboid {
							x_range,
							y_range,
							z_range,
						});
					}
				}
			}
		}

		// Iteratively merge the split-up cuboids together, where possible. For instance,
		// if the middle of a cuboid was removed, there are 26 small cuboids created, but they
		// can be merged into six larger cuboids. This is done by looking for abutting
		// cuboids with the same dimensions along their respective abutting faces and
		// combining them into one cuboid.
		'merge: loop {
			for (
				i,
				&c1 @ Cuboid {
					x_range,
					y_range,
					z_range,
				},
			) in on_cuboids.iter().enumerate()
			{
				for (j, &c2) in on_cuboids.iter().enumerate().skip(i + 1) {
					let mut cs = [c1, c2];
					let mut merged_cuboid = None;
					if y_range == c2.y_range && z_range == c2.z_range {
						cs.sort_by_key(|c| c.x_range);
						if cs[0].x_range[1] == cs[1].x_range[0] - 1 {
							merged_cuboid = Some(Cuboid {
								x_range: [cs[0].x_range[0], cs[1].x_range[1]],
								y_range,
								z_range,
							});
						}
					} else if x_range == c2.x_range && z_range == c2.z_range {
						cs.sort_by_key(|c| c.y_range);
						if cs[0].y_range[1] == cs[1].y_range[0] - 1 {
							merged_cuboid = Some(Cuboid {
								x_range,
								y_range: [cs[0].y_range[0], cs[1].y_range[1]],
								z_range,
							});
						}
					} else if x_range == c2.x_range && y_range == c2.y_range {
						cs.sort_by_key(|c| c.z_range);
						if cs[0].z_range[1] == cs[1].z_range[0] - 1 {
							merged_cuboid = Some(Cuboid {
								x_range,
								y_range,
								z_range: [cs[0].z_range[0], cs[1].z_range[1]],
							});
						}
					}

					if let Some(merged_cuboid) = merged_cuboid {
						on_cuboids.swap_remove(j);
						on_cuboids.swap_remove(i);
						on_cuboids.push(merged_cuboid);
						continue 'merge;
					}
				}
			}

			break;
		}

		on_cuboids
	}
}

impl Cuboid {
	fn from_coords_str(s: &str) -> Option<Self> {
		let mut coords = s.split(',');

		let mut comps = (0..3).filter_map(|_| {
			coords
				.next()?
				.split('=')
				.nth_back(0)?
				.split('.')
				.filter_map(|splat| splat.parse().ok())
				.collect::<Vec<_>>()
				.try_into()
				.ok()
		});

		let x_range = comps.next()?;
		let y_range = comps.next()?;
		let z_range = comps.next()?;

		Some(Self {
			x_range,
			y_range,
			z_range,
		})
	}
}

#[derive(Debug, Clone, Copy)]
enum State {
	On,
	Off,
}

impl State {
	fn from_str(s: &str) -> Option<Self> {
		Some(match s {
			"on" => Self::On,
			"off" => Self::Off,
			_ => return None,
		})
	}
}

#[derive(Debug, Clone, Copy)]
struct RebootStep {
	state: State,
	cuboid: Cuboid,
}

// tag::debugging[]
impl Display for RebootStep {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_char(match self.state {
			State::On => '▲',
			State::Off => '▽',
		})?;
		f.write_char(' ')?;

		let Cuboid {
			x_range,
			y_range,
			z_range,
		} = &self.cuboid;
		write!(f, "{:?} {:?} {:?}", x_range, y_range, z_range)
	}
}

// end::debugging[]
impl RebootStep {
	fn from_line(line: &str) -> Option<Self> {
		let mut str_comps = line.split_ascii_whitespace();
		let state = State::from_str(str_comps.next()?)?;
		let cuboid = Cuboid::from_coords_str(str_comps.next()?)?;
		Some(Self { state, cuboid })
	}
}

#[derive(Debug)]
struct Grid {
	on_cuboids: Vec<Cuboid>,
	bounds: Option<Cuboid>,
}

impl Grid {
	fn new_with_size(n: i32) -> Self {
		Grid {
			on_cuboids: vec![],
			bounds: Some(Cuboid {
				x_range: [-n, n],
				y_range: [-n, n],
				z_range: [-n, n],
			}),
		}
	}

	fn new_unbounded() -> Self {
		Grid {
			on_cuboids: vec![],
			bounds: None,
		}
	}

	fn apply_step(&mut self, RebootStep { state, cuboid }: &RebootStep) -> Option<()> {
		let cuboid = if let Some(bounds) = self.bounds {
			cuboid.intersection(&bounds)?
		} else {
			*cuboid
		};

		match state {
			State::On => {
				self.on_cuboids.push(cuboid);
			}
			State::Off => {
				let mut on_cuboids = vec![];
				for my_cuboid in &self.on_cuboids {
					on_cuboids.extend(my_cuboid.difference(&cuboid));
				}
				self.on_cuboids = on_cuboids;
			}
		};

		// println!("{:?}", self.on_cuboids);

		Some(())
	}

	fn n_on(&self) -> usize {
		let mut nonintersecting_cuboids = vec![];
		for (i, &c1) in self.on_cuboids.iter().enumerate() {
			let mut pieces = vec![c1];
			for &c2 in self.on_cuboids.iter().skip(i + 1) {
				pieces = pieces
					.into_iter()
					.flat_map(|piece| piece.difference(&c2))
					.collect();
			}
			nonintersecting_cuboids.extend(pieces);
		}

		nonintersecting_cuboids
			.iter()
			.fold(0, |accum, cuboid| accum + cuboid.size())
	}
}

fn read_input(input: &str) -> Option<Vec<RebootStep>> {
	input
		.lines()
		.map(RebootStep::from_line)
		.collect::<Option<Vec<_>>>()
}

fn ans_for_input(input: &str) -> Answer<usize, usize> {
	let steps = read_input(input).unwrap();
	(22, (pt1(steps.iter()), pt2(steps.iter()))).into()
}

pub fn ans() -> Answer<usize, usize> {
	ans_for_input(include_str!("input.txt"))
}
// end::setup[]

// tag::pt1[]
fn pt1<R: std::borrow::Borrow<RebootStep>>(steps: impl Iterator<Item = R>) -> usize {
	let mut grid = Grid::new_with_size(50);
	for step in steps {
		grid.apply_step(step.borrow());
	}

	grid.n_on()
}
// end::pt1[]

// tag::pt2[]
fn pt2<R: std::borrow::Borrow<RebootStep>>(steps: impl Iterator<Item = R>) -> usize {
	let mut grid = Grid::new_unbounded();
	for step in steps {
		grid.apply_step(step.borrow());
	}

	grid.n_on()
}
// end::pt2[]
