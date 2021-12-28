use crate::Answer;
use std::{
	collections::{BTreeMap as Map, BTreeSet as Set},
	ops::{Index, IndexMut},
};

type Digit = i32;
type Output = String;

#[derive(Debug, Clone, Copy)]
enum Operand {
	Number(Digit),
	Reg(Register),
}

impl Operand {
	fn from_str(s: &str) -> Option<Self> {
		use Operand::*;
		Some(match s.parse().ok() {
			Some(val) => Number(val),
			None => Reg(Register::from_str(s)?),
		})
	}
}

#[derive(Debug, Clone, Copy)]
enum MathOp {
	Add,
	Mul,
	Div,
	Mod,
	Eql,
}

impl MathOp {
	fn from_str(s: &str) -> Option<Self> {
		use MathOp::*;
		Some(match s {
			"add" => Add,
			"mul" => Mul,
			"div" => Div,
			"mod" => Mod,
			"eql" => Eql,
			_ => return None,
		})
	}
}

#[derive(Debug, Clone)]
struct MathInstr {
	operation: MathOp,
	register: Register,
	operand: Operand,
}

#[derive(Debug)]
struct InstrBlock {
	in_reg: Register,
	instrs: Vec<MathInstr>,
}

#[derive(Debug, Clone, Copy)]
enum Register {
	W = 0,
	X,
	Y,
	Z,
}

impl Register {
	fn from_str(s: &str) -> Option<Self> {
		use Register::*;
		Some(match s {
			"w" => W,
			"x" => X,
			"y" => Y,
			"z" => Z,
			_ => return None,
		})
	}
}

#[derive(Debug)]
struct Alu {
	registers: [Digit; 4],
}

impl Index<Register> for Alu {
	type Output = Digit;
	fn index(&self, register: Register) -> &Self::Output {
		&self.registers[register as usize]
	}
}

impl IndexMut<Register> for Alu {
	fn index_mut(&mut self, register: Register) -> &mut Self::Output {
		&mut self.registers[register as usize]
	}
}

impl Alu {
	fn new() -> Self {
		Self { registers: [0; 4] }
	}

	fn run_block(&mut self, block: &InstrBlock, input: Digit) {
		use MathOp::*;
		use Operand::*;

		self[block.in_reg] = input;

		for &MathInstr {
			operation,
			register,
			operand,
		} in block.instrs.iter()
		{
			let value = match operand {
				Number(n) => n,
				Reg(register) => self[register],
			};

			let r = &mut self[register];
			match operation {
				Add => *r += value,
				Mul => *r *= value,
				Div => *r /= value,
				Mod => *r %= value,
				Eql => *r = if r == &value { 1 } else { 0 },
			}
		}
	}

	fn from_running_block_on(
		block: &InstrBlock,
		input: Digit,
		setup: impl FnOnce(&mut Self),
	) -> Self {
		let mut alu = Alu::new();
		setup(&mut alu);
		alu.run_block(block, input);
		alu
	}
}

fn read_input(s: &str) -> Option<Vec<InstrBlock>> {
	let mut blocks = vec![];
	let mut curr_in_reg = None;
	let mut curr_instrs = vec![];

	// Dummy input line at the end that tells the last block it's done
	for line in s.lines().chain(std::iter::once("inp x")) {
		let mut splat = line.split_ascii_whitespace();
		let instr_str = splat.next()?;
		if instr_str == "inp" {
			if let Some(r) = curr_in_reg {
				blocks.push(InstrBlock {
					in_reg: r,
					instrs: curr_instrs.clone(),
				});
			}

			curr_in_reg = Some(Register::from_str(splat.next()?)?);
			curr_instrs.clear();
		} else {
			let operation = MathOp::from_str(instr_str)?;
			let register = Register::from_str(splat.next()?)?;
			let operand = Operand::from_str(splat.next()?)?;
			curr_instrs.push(MathInstr {
				operation,
				register,
				operand,
			});
		}
	}

	Some(blocks)
}

fn ans_for_input(input: &str) -> Answer<Output, Output> {
	let blocks = read_input(input).unwrap();
	let valid_zs = get_valid_zs(&blocks);
	(24, (pt1(&blocks, &valid_zs), pt2(&blocks, &valid_zs))).into()
}

pub fn ans() -> Answer<Output, Output> {
	ans_for_input(include_str!("input.txt"))
}

fn get_valid_zs<V: AsRef<[InstrBlock]>>(blocks: V) -> Vec<Set<Digit>> {
	let blocks = blocks.as_ref();
	let n_digits = blocks.len();

	let mut all_zs_ltr = vec![Map::new(); n_digits + 1];
	all_zs_ltr.first_mut().unwrap().insert(0, Set::new());

	for (digit_idx, block) in blocks.iter().enumerate() {
		let (all_prev_zs, all_next_zs) = all_zs_ltr.split_at_mut(digit_idx + 1);
		let prev_zs = &all_prev_zs[all_prev_zs.len() - 1];
		let curr_zs = &mut all_next_zs[0];

		for &prev_z in prev_zs.keys() {
			for digit in 1..=9 {
				let z = Alu::from_running_block_on(block, digit, |alu| alu[Register::Z] = prev_z)
					[Register::Z];
				curr_zs.entry(z).or_insert_with(Set::new).insert(prev_z);
			}
		}

		println!("{}: {:?}", digit_idx, curr_zs.len());
	}

	let mut all_valid_zs_rtl = vec![Set::new(); n_digits];
	let mut curr_zs = Set::from_iter(std::iter::once(0));

	for (digit_idx, valid_zs) in all_valid_zs_rtl.iter_mut().enumerate().rev() {
		valid_zs.extend(curr_zs.iter().copied());

		let mut new_curr_zs = Set::new();
		let prev_zs = &all_zs_ltr[digit_idx + 1];
		for z in curr_zs.iter() {
			new_curr_zs.extend(prev_zs.get(z).unwrap().iter().copied());
		}
		curr_zs = new_curr_zs;
	}

	all_valid_zs_rtl
}

fn find_digits<DigitRange: Iterator<Item = Digit>>(
	blocks: impl AsRef<[InstrBlock]>,
	valid_zs: impl AsRef<[Set<Digit>]>,
	first_digit: Digit,
	attempted_digit_range_ctor: impl Fn(Digit) -> DigitRange,
	get_next_digit: impl Fn(Digit) -> Digit,
	can_continue: impl Fn(Digit) -> bool,
) -> Output {
	struct CandidateDigit {
		z_init: Digit,
		digit: Digit,
		next_digit_attempted: Digit,
	}

	let blocks = blocks.as_ref();
	let valid_zs = valid_zs.as_ref();

	let n_digits = blocks.len();

	let mut candidates = vec![CandidateDigit {
		z_init: 0,
		digit: 0,
		next_digit_attempted: first_digit,
	}];

	'find_digits: while candidates.len() <= n_digits {
		let digit_idx = candidates.len() - 1;
		let block = &blocks[digit_idx];

		let CandidateDigit {
			z_init,
			digit,
			next_digit_attempted,
		} = candidates[digit_idx];

		for attempted_digit in attempted_digit_range_ctor(next_digit_attempted) {
			let z =
				Alu::from_running_block_on(block, attempted_digit, |alu| alu[Register::Z] = z_init)
					[Register::Z];

			if valid_zs[digit_idx].contains(&z) {
				candidates[digit_idx].next_digit_attempted = get_next_digit(attempted_digit);
				candidates.push(CandidateDigit {
					z_init: z,
					digit: attempted_digit,
					next_digit_attempted: first_digit,
				});
				continue 'find_digits;
			}
		}

		// Could not find a digit that worked; need to backtrack
		if can_continue(digit) {
			candidates[digit_idx] = CandidateDigit {
				z_init,
				digit: get_next_digit(digit),
				next_digit_attempted: first_digit,
			};
		} else {
			candidates.pop();
			assert_ne!(candidates.len(), 0)
		}
	}

	candidates
		.iter()
		.skip(1)
		.map(|c| c.digit.to_string())
		.collect::<Vec<_>>()
		.join("")
}

fn pt1(blocks: impl AsRef<[InstrBlock]>, valid_zs: impl AsRef<[Set<Digit>]>) -> Output {
	find_digits(
		blocks,
		valid_zs,
		9,
		|digit| (1..=digit).rev(),
		|digit| digit - 1,
		|digit| digit > 1,
	)
}

fn pt2(blocks: impl AsRef<[InstrBlock]>, valid_zs: impl AsRef<[Set<Digit>]>) -> Output {
	find_digits(
		blocks,
		valid_zs,
		1,
		|digit| digit..=9,
		|digit| digit + 1,
		|digit| digit < 9,
	)
}
