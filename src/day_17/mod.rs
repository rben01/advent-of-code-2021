// tag::setup[]
use crate::Answer;
use num::integer::Roots;
use std::collections::BTreeSet as Set;

// tag::setup_main[]
type Time = i64;
type Num = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos<T> {
	x: T,
	y: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Velo<T> {
	vx: T,
	vy: T,
}

#[derive(Debug, Clone, Copy)]
struct Rect<T> {
	x_min: T,
	x_max: T,
	y_min: T,
	y_max: T,
}

#[derive(Debug, Clone, Copy)]
struct Trajectory {
	_t: Time,
	_pos: Pos<Num>,
	velo: Velo<Num>,
}

fn read_input(input: &str) -> Option<Rect<Num>> {
	let re = {
		regex::Regex::new(r"target area:\s*x=([\d-]+)\.\.([\d-]+),\s*y=([\d-]+)\.\.([\d-]+)")
			.ok()?
	};

	let caps = re.captures(input.trim())?;
	let [x1, x2, y1, y2] = // force line break :/
		[1, 2, 3, 4].map(|i| caps.get(i).and_then(|m| m.as_str().parse().ok()));

	let [x1, x2, y1, y2] = [x1?, x2?, y1?, y2?];

	let [x_min, x_max] = if x1 < x2 { [x1, x2] } else { [x2, x1] };
	let [y_min, y_max] = if y1 < y2 { [y1, y2] } else { [y2, y1] };

	Some(Rect {
		x_min,
		x_max,
		y_min,
		y_max,
	})
}

fn sqrt(n: Num) -> Option<Num> {
	let sqrt = Roots::sqrt(&n);
	if sqrt * sqrt == n {
		Some(sqrt)
	} else {
		None
	}
}

// end::setup_main[]

// tag::velo_finder[]
fn find_velocities(t: Time, position: Pos<Num>) -> [Option<Velo<Num>>; 2] {
	if t == 0 {
		return [Some(Velo { vx: 0, vy: 0 }), None];
	}

	let Pos { x, y } = position;

	let vy_numer = 2 * y + t * (t - 1);
	let vy_denom = 2 * t;
	let vy = if vy_numer % vy_denom == 0 {
		vy_numer / vy_denom
	} else {
		return [None, None];
	};

	let vx1 = {
		let vx_numer = 2 * x + t * (t - 1);
		let vx_denom = 2 * t;
		let vx = vx_numer / vx_denom;
		if vx_numer % vx_denom == 0 && vx >= t {
			Some(vx)
		} else {
			None
		}
	};

	let vx2 = {
		let discriminant = 1 + 8 * x;
		sqrt(discriminant).and_then(|sqrt_disc| {
			let vx_numer = sqrt_disc - 1;
			let vx_denom = 2;
			let vx = vx_numer / vx_denom;
			if vx_numer % vx_denom == 0 && vx <= t {
				Some(vx)
			} else {
				None
			}
		})
	};

	[vx1, vx2].map(|opt_vx| opt_vx.map(|vx| Velo { vx, vy }))
}
// end::velo_finder[]

// tag::time_finder[]
fn find_ts_and_vys_for_y(y: Num) -> Vec<(Time, Num)> {
	assert_ne!(y, 0);

	let mut ans = vec![];

	// Need to find all integer (t, vy) that satisfy y = vy * t - t*(t-1)/2 with t > 0
	//
	// Step one: (2*vy + 1)^2 - 8*y = must be square
	//
	// In other words there must exist integral m and n such that m^2 - n^2 = 8y (with
	// 2*vy + 1 = m). m^2 - n^2 = (m-n)*(m+n), and so...
	let eight_y = 8 * y;
	let abs_eight_y = eight_y.abs();
	for k1 in 1..=Roots::sqrt(&abs_eight_y) {
		if abs_eight_y % k1 != 0 {
			continue;
		}

		for sign in [-1, 1] {
			let k1 = sign * k1;
			let k2 = eight_y / k1;
			// k1 and k2 are now two signed integers that multiply to y

			let two_m = k1 + k2;
			if two_m % 2 != 0 {
				continue;
			}

			let m = two_m / 2;

			// Now, m was 2*vy + 1, and so...
			if (m - 1) % 2 != 0 {
				continue;
			}
			let vy = (m - 1) / 2;
			let discriminant = m * m - eight_y;
			if discriminant < 0 {
				continue;
			}

			let sqrt_disc = sqrt(discriminant).unwrap();

			for pm in [-1, 1] {
				let t_numer = 2 * vy + 1 + pm * sqrt_disc;
				if t_numer <= 0 || t_numer % 2 != 0 {
					continue;
				}
				let t = t_numer / 2;
				ans.push((t, vy));
			}
		}
	}

	ans
}
// end::time_finder[]

fn ans_for_input(input: &str) -> Answer<Option<Num>, usize> {
	let rect = read_input(input).unwrap();
	let trajectories = get_trajectories(rect);
	(17, (pt1(&trajectories), pt2(&trajectories))).into()
}

pub fn ans() -> Answer<Option<Num>, usize> {
	ans_for_input(include_str!("input.txt"))
}
// end::setup[]

// tag::get_traj[]
fn get_x(t: Time, vx: Num) -> Num {
	if t <= vx {
		vx * t - t * (t - 1) / 2
	} else {
		vx * (vx + 1) / 2
	}
}

fn get_trajectories(rect: Rect<Num>) -> Vec<Trajectory> {
	let Rect {
		x_min,
		x_max,
		y_min,
		y_max,
	} = rect;

	let mut trajectories = vec![];

	for y in y_min..=y_max {
		if y == 0 {
			// If any vx works, then there will be infinitely many choices for vy because it
			// retraces its ascent on its descent. And if no vx works, then it's moot
			continue;
		}

		let ts_and_vys = find_ts_and_vys_for_y(y);

		for (t, vy) in ts_and_vys {
			for x in x_min..=x_max {
				let velocities = find_velocities(t, Pos { x, y });

				for velo in velocities {
					let velo = match velo {
						Some(v) => v,
						None => continue,
					};
					if velo.vy == vy && get_x(t, velo.vx) == x {
						trajectories.push(Trajectory {
							_t: t,
							_pos: Pos { x, y },
							velo,
						});
					}
				}
			}
		}
	}

	trajectories
}
// end::get_traj[]

// tag::pt1[]
fn pt1(trajectories: &[Trajectory]) -> Option<Num> {
	trajectories
		.iter()
		.map(|traj| {
			let vy = traj.velo.vy;
			if vy < 0 {
				return 0;
			}
			vy * (vy + 1) / 2
		})
		.max()
}
// end::pt1[]

// tag::pt2[]
fn pt2(trajectories: &[Trajectory]) -> usize {
	trajectories
		.iter()
		.map(|Trajectory { velo, .. }| (velo.vx, velo.vy))
		.collect::<Set<_>>()
		.len()
}
// end::pt2[]

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_input;

	#[test]
	fn test_velocity_finder() {
		for (t, x, y) in [
			(1, 7, 2),
			(2, 13, 3),
			(3, 18, 3),
			(4, 22, 2),
			(5, 25, 0),
			(6, 27, -3),
			(7, 28, -7),
		] {
			assert_eq!(
				find_velocities(t, Pos { x, y })
					.iter()
					.flatten()
					.next()
					.unwrap(),
				&Velo { vx: 7, vy: 2 }
			);
		}

		for (t, x, y) in [
			(1, 6, 3),
			(2, 11, 5),
			(3, 15, 6),
			(4, 18, 6),
			(5, 20, 5),
			(6, 21, 3),
			(7, 21, 0),
			(8, 21, -4),
			(9, 21, -9),
		] {
			assert_eq!(
				find_velocities(t, Pos { x, y })
					.iter()
					.flatten()
					.next()
					.unwrap(),
				&Velo { vx: 6, vy: 3 }
			);
		}

		test_input!("target area: x=20..30, y=-10..-5", day: 17, ans: (Some(45), 112));
		test_input!("target area: x=34..35, y=-8..-6", day: 17, ans: (Some(3), 9));
	}
}
