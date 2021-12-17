// tag::setup[]
use crate::{to_decimal, Answer};
use std::fmt::{Display, Write};

type Number = i64;

struct Binary(Vec<bool>);

impl Binary {
	fn from_hex(s: &str) -> Option<Self> {
		let mut binary = Vec::with_capacity(s.len() * 4);
		for c in s.trim().chars() {
			let n = c.to_digit(16)?;
			let digits = [3usize, 2, 1, 0].map(|place| ((1 << place) & n) != 0);
			binary.extend_from_slice(&digits);
		}

		Some(Self(binary))
	}
}

impl Display for Binary {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for &digit in &self.0 {
			let c = match digit {
				true => '1',
				false => '0',
			};
			f.write_char(c)?;
		}
		Ok(())
	}
}

impl Binary {
	fn as_packets(&self) -> Vec<Packet> {
		#[derive(Debug)]
		enum RemainingData {
			NBits(usize),
			NPackets(usize),
		}

		impl RemainingData {
			fn is_empty(&self) -> bool {
				matches!(self, RemainingData::NBits(0) | RemainingData::NPackets(0))
			}
		}

		#[derive(Debug)]
		struct ParseState {
			depth: usize,
			remaining: RemainingData,
		}

		let mut packets = vec![];

		let orig_data = &self.0;
		let header_length = 6;
		let mut cursor = 0;
		let mut stack = vec![ParseState {
			depth: 0,
			remaining: RemainingData::NPackets(1),
		}];

		while let Some(parse_state) = stack.pop() {
			let ParseState { depth, remaining } = parse_state;

			if remaining.is_empty() {
				continue;
			}

			let packet_bits = &orig_data[cursor..];

			let version_number = to_decimal(&packet_bits[0..3]);
			let kind_number = to_decimal(&packet_bits[3..6]);
			let data_bits = &packet_bits[header_length..];

			let parent_packet_length = match remaining {
				RemainingData::NBits(n) => RemainingData::NBits(n),
				RemainingData::NPackets(n) => RemainingData::NPackets(n - 1),
			};
			stack.push(ParseState {
				depth,
				remaining: parent_packet_length,
			});

			let packet;
			let n_bits_consumed;
			match kind_number {
				4 => {
					let chunk_size = 5;
					let mut bin_bits = vec![];

					let mut n_chunks = 0;
					for chunk in data_bits.chunks_exact(5usize) {
						n_chunks += 1;
						bin_bits.extend_from_slice(&chunk[1..]);
						if !chunk[0] {
							break;
						}
					}

					let value = to_decimal(bin_bits) as Number;

					n_bits_consumed = header_length + n_chunks * chunk_size;
					packet = Packet {
						version_number,
						kind: PacketKind::Literal { value },
						depth,
					};
				}
				op => {
					let op_data_length;
					let n_bits_for_length;

					let length_type = data_bits[0];
					match length_type {
						false => {
							// length in bits
							n_bits_for_length = 16;
							let n_bits = to_decimal(&data_bits[1..n_bits_for_length]);
							op_data_length = RemainingData::NBits(n_bits);
						}
						true => {
							// length in packets
							n_bits_for_length = 12;
							let n_packets = to_decimal(&data_bits[1..n_bits_for_length]);
							op_data_length = RemainingData::NPackets(n_packets);
						}
					};

					n_bits_consumed = header_length + n_bits_for_length;
					// A hack; we're going to subtract n_bits_consumed from this later
					// despite the fact that in theory we shouldn't (because the newly added
					// packet hasn't consumed any data yet), so we we pre-add n_bits_consumed
					// here so that when we subtract it later we end up with the right number
					// of bits
					let op_data_length = match op_data_length {
						RemainingData::NBits(n) => RemainingData::NBits(n + n_bits_consumed),
						rd => rd,
					};

					packet = Packet {
						version_number,
						kind: PacketKind::Operator { op: op.into() },
						depth,
					};

					stack.push(ParseState {
						depth: depth + 1,
						remaining: op_data_length,
					});
				}
			};

			cursor += n_bits_consumed;

			for ps in stack.iter_mut() {
				if let RemainingData::NBits(n) = &mut ps.remaining {
					if *n > 0 {
						// The hack above is to counteract this subtraction; if we just pushed
						// a RemainingData::NumBits, we won't actually have consumed any of
						// its input yet
						//
						// If our code has no bugs, and the input is trustworthy, this will
						// never underflow.
						*n -= n_bits_consumed;
					}
				}
			}

			packets.push(packet);
		}

		packets
	}
}

#[derive(Debug)]
enum PacketKind {
	Literal { value: Number },
	Operator { op: Operation }, // Defined in pt2
}

#[derive(Debug)]
struct Packet {
	version_number: usize,
	kind: PacketKind,
	depth: usize,
}

fn read_input(input: &str) -> Vec<Packet> {
	let b = Binary::from_hex(input).unwrap();
	b.as_packets()
}

fn ans_for_input(input: &str) -> Answer<usize, Number> {
	let p = read_input(input);
	(16, (pt1(&p), pt2(&p).unwrap())).into()
}

pub fn ans() -> Answer<usize, Number> {
	ans_for_input(include_str!("input.txt"))
}
// end::setup[]

// tag::pt1[]
fn pt1(packets: &[Packet]) -> usize {
	packets.iter().map(|packet| packet.version_number).sum()
}
// end::pt1[]

// tag::pt2[]
#[derive(Debug)]
enum Reducer {
	Sum,
	Product,
	Min,
	Max,
}

impl Reducer {
	fn identity(&self) -> Number {
		use Reducer::*;
		match self {
			Sum => 0,
			Product => 1,
			Min => Number::MAX,
			Max => Number::MIN,
		}
	}

	fn combine(&self, x: Number, y: Number) -> Number {
		use Reducer::*;
		match self {
			Sum => x + y,
			Product => x * y,
			Min => x.min(y),
			Max => x.max(y),
		}
	}
}

#[derive(Debug)]
enum Comparitor {
	Gt,
	Lt,
	Eq,
}

impl Comparitor {
	fn apply(&self, x: Number, y: Number) -> Number {
		use Comparitor::*;
		(match self {
			Gt => x > y,
			Lt => x < y,
			Eq => x == y,
		}) as Number
	}
}

#[derive(Debug)]
enum Operation {
	Reduce(Reducer),
	Compare(Comparitor),
}

impl From<usize> for Operation {
	fn from(n: usize) -> Self {
		use Comparitor::*;
		use Operation::*;
		use Reducer::*;
		match n {
			0 => Reduce(Sum),
			1 => Reduce(Product),
			2 => Reduce(Min),
			3 => Reduce(Max),
			5 => Compare(Gt),
			6 => Compare(Lt),
			7 => Compare(Eq),
			_ => unreachable!(),
		}
	}
}

fn pt2(packets: &[Packet]) -> Option<Number> {
	struct Arg {
		depth: usize,
		value: Number,
	}
	let mut arg_stack = vec![];

	for Packet {
		kind: packet_kind,
		depth: packet_depth,
		..
	} in packets.iter().rev()
	{
		let packet_depth = *packet_depth;

		match packet_kind {
			&PacketKind::Literal { value } => arg_stack.push(Arg {
				depth: packet_depth,
				value,
			}),
			PacketKind::Operator { op } => {
				use Operation::*;
				let value = match op {
					Reduce(reducer) => {
						let mut result = reducer.identity();
						while let Some(arg @ Arg { depth, value }) = arg_stack.pop() {
							if depth <= packet_depth {
								arg_stack.push(arg); // oops, went too far
								break;
							}
							result = reducer.combine(result, value);
						}
						result
					}
					Compare(comparitor) => {
						let Arg { value: first, .. } = arg_stack.pop()?;
						let Arg { value: second, .. } = arg_stack.pop()?;

						comparitor.apply(first, second)
					}
				};

				arg_stack.push(Arg {
					depth: packet_depth,
					value,
				});
			}
		}
	}

	Some(arg_stack.first()?.value)
}
// end::pt2[]

#[cfg(test)]
mod test {
	use super::*;
	use crate::test_input;

	#[test]
	fn test() {
		#[track_caller]
		fn test_pt1(in_str: &str, pt1_val: usize) {
			test_input!(&read_input(in_str), pt1: pt1_val);
		}

		#[track_caller]
		fn test_pt2(in_str: &str, pt2_val: Number) {
			test_input!(&read_input(in_str), pt2: Some(pt2_val));
		}

		// pt1 only
		test_pt1("D2FE28", 6);
		test_pt1("8A004A801A8002F478", 16);
		test_pt1("620080001611562C8802118E34", 12);
		test_pt1("C0015000016115A2E0802F182340", 23);
		test_pt1("A0016C880162017C3686B18A3D4780", 31);
		test_pt1(include_str!("input.txt"), 927);

		// pt2 only
		test_pt2("D2FE28", 2021);
		test_pt2("C200B40A82", 3);
		test_pt2("04005AC33890", 54);
		test_pt2("880086C3E88112", 7);
		test_pt2("CE00C43D881120", 9);
		test_pt2("D8005AC2A8F0", 1);
		test_pt2("F600BC2D8F", 0);
		test_pt2("9C005AC2F8F0", 0);
		test_pt2("9C0141080250320F1802104A08", 1);
	}
}
