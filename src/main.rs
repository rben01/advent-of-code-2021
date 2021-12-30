macro_rules! show_answers {
	($($day:ident),* $(,)?) => {
		$(
			mod $day {
				pub(super) use advent_of_code_2021::$day::ans;
			}
		)*
      $(
			println!("{}", $day::ans());
		)*
	};
}

fn main() {
	show_answers!(
		day_01, day_02, day_03, day_04, day_05, day_06, day_12, day_13, day_14, day_15, day_16,
		day_17, day_18, day_19, day_20, day_21, day_22, day_24, day_25,
	);
}
