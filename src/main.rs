macro_rules! show_answers {
	($($day:ident),* $(,)?) => {
		$(
			mod $day {
				pub(super) use advent_of_code_2021::$day::ans;
			}
			println!("{}", $day::ans());
		)*
	};
}

fn main() {
	show_answers!(day_17);
}
