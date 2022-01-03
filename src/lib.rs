#![feature(array_zip)]
#![warn(clippy::pedantic)]
#![allow(
	clippy::enum_glob_use,
	clippy::missing_panics_doc,
	clippy::must_use_candidate,
	clippy::similar_names,
	clippy::too_many_lines
)]

use std::fmt::{Debug, Display};

pub(crate) mod utils;

// tag::mods[]
macro_rules! include_days {
	($($mod_name:ident:$ft_name:literal),* $(,)?) => {
		$(#[cfg(feature = $ft_name)] pub mod $mod_name;)*
	};
}

include_days!(
	/* day_01:"day_01", */
	/* day_02:"day_02", */
	/* day_03:"day_03", */
	/* day_04:"day_04", */
	/* day_05:"day_05", */
	/* day_06:"day_06", */
	// day_07:"day_07",
	day_08:"day_08",
	// day_09:"day_09",
	// day_10:"day_10",
	// day_11:"day_11",
	/* day_12:"day_12", */
	/* day_13:"day_13", */
	/* day_14:"day_14", */
	/* day_15:"day_15", */
	/* day_16:"day_16", */
	/* day_17:"day_17", */
	/* day_18:"day_18", */
	/* day_19:"day_19", */
	/* day_20:"day_20", */
	/* day_21:"day_21", */
	/* day_22:"day_22", */
	// day_23:"day_23",
	/* day_24:"day_24", */
	/* day_25:"day_25", */
);

// end::mods[]
#[derive(Debug, PartialEq, Eq)]
pub struct Answer<T1, T2> {
	day: usize,
	pt1: T1,
	pt2: T2,
}

impl<T1: Debug, T2: Debug> Display for Answer<T1, T2> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let Answer { day, pt1, pt2 } = self;
		write!(f, "Day: {:?} ; Part 1: {:?} ; Part 2: {:?}", day, pt1, pt2)
	}
}

impl<T1, T2> From<(usize, (T1, T2))> for Answer<T1, T2> {
	fn from((day, (pt1, pt2)): (usize, (T1, T2))) -> Self {
		Self { day, pt1, pt2 }
	}
}
