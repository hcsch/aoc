use bitvec::prelude::*;

fn nibble_from_hex_byte(byte: u8) -> u8 {
    match byte {
        b'0'..=b'9' => byte - b'0',
        b'A'..=b'Z' => byte - b'A' + 10,
        b'a'..=b'z' => byte - b'a' + 10,
        _ => panic!("invalid hex digit"),
    }
}

fn parse_input<I: Iterator<Item = String>>(mut input_lines: I) -> BitVec<u8, Msb0> {
    input_lines
        .next()
        .unwrap()
        .bytes()
        .flat_map(|nibble_b| {
            let nibble = nibble_from_hex_byte(nibble_b);
            [
                (nibble >> 3 & 1) != 0,
                (nibble >> 2 & 1) != 0,
                (nibble >> 1 & 1) != 0,
                (nibble & 1) != 0,
            ]
        })
        .collect()
}

fn consume_bit(bits: &mut &BitSlice<u8, Msb0>) -> bool {
    let data = bits[0];
    *bits = &bits[1..];
    data
}

fn consume_n_bits<F: FnMut(&BitSlice<u8, Msb0>) -> T, T>(
    bits: &mut &BitSlice<u8, Msb0>,
    n: usize,
    mut f: F,
) -> T {
    let res = f(&bits[..n]);
    *bits = &bits[n..];
    res
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
enum OperatorKind {
    Sum = 0,
    Product = 1,
    Minimum = 2,
    Maximum = 3,
    GreaterThan = 5,
    LessThan = 6,
    EqualTo = 7,
}

impl TryFrom<u8> for OperatorKind {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(OperatorKind::Sum),
            1 => Ok(OperatorKind::Product),
            2 => Ok(OperatorKind::Minimum),
            3 => Ok(OperatorKind::Maximum),
            5 => Ok(OperatorKind::GreaterThan),
            6 => Ok(OperatorKind::LessThan),
            7 => Ok(OperatorKind::EqualTo),
            _ => Err("invalid operator kind"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
enum Packet {
    Literal {
        version: u8,
        content: BitVec<u8, Msb0>,
    },
    Operator {
        version: u8,
        kind: OperatorKind,
        sub_packets: Vec<Packet>,
    },
}

impl Packet {
    pub fn parse_partial(bits: &mut &BitSlice<u8, Msb0>) -> Self {
        let version: u8 = consume_n_bits(bits, 3, |n_bits| n_bits.load_be());
        let packet_type: u8 = consume_n_bits(bits, 3, |n_bits| n_bits.load_be());

        match packet_type {
            4 => {
                let mut content = BitVec::new();
                loop {
                    if consume_bit(bits) {
                        consume_n_bits(bits, 4, |n_bits| content.extend(n_bits));
                    } else {
                        consume_n_bits(bits, 4, |n_bits| content.extend(n_bits));
                        break;
                    }
                }
                Self::Literal { version, content }
            }
            _ => {
                let length_type = consume_bit(bits);
                let sub_packets = if length_type {
                    let num_sub_packets: u16 = consume_n_bits(bits, 11, |n_bits| n_bits.load_be());

                    (0..num_sub_packets)
                        .into_iter()
                        .map(|_| {
                            let sub_packet = Packet::parse_partial(bits);
                            sub_packet
                        })
                        .collect()
                } else {
                    let num_sub_packets_bits: usize =
                        consume_n_bits(bits, 15, |n_bits| n_bits.load_be::<u16>()) as usize;

                    let mut sub_packets = Vec::new();

                    let initial_bits_len = bits.len();

                    while initial_bits_len - bits.len() < num_sub_packets_bits {
                        let sub_packet = Packet::parse_partial(bits);
                        sub_packets.push(sub_packet);
                    }

                    sub_packets
                };
                Self::Operator {
                    version,
                    kind: OperatorKind::try_from(packet_type).unwrap(),
                    sub_packets,
                }
            }
        }
    }

    pub fn version_sum(&self) -> usize {
        match self {
            Packet::Literal { version, .. } => *version as usize,
            Packet::Operator {
                version,
                sub_packets,
                ..
            } => *version as usize + sub_packets.iter().map(Packet::version_sum).sum::<usize>(),
        }
    }

    pub fn eval(&self) -> usize {
        match self {
            Packet::Literal { content, .. } => content.load_be::<usize>(),
            Packet::Operator {
                kind: OperatorKind::Sum,
                sub_packets,
                ..
            } => sub_packets.iter().map(Packet::eval).sum::<usize>(),
            Packet::Operator {
                kind: OperatorKind::Product,
                sub_packets,
                ..
            } => sub_packets.iter().map(Packet::eval).product::<usize>(),
            Packet::Operator {
                kind: OperatorKind::Minimum,
                sub_packets,
                ..
            } => sub_packets.iter().map(Packet::eval).min().unwrap(),
            Packet::Operator {
                kind: OperatorKind::Maximum,
                sub_packets,
                ..
            } => sub_packets.iter().map(Packet::eval).max().unwrap(),
            Packet::Operator {
                kind: OperatorKind::GreaterThan,
                sub_packets,
                ..
            } => (sub_packets[0].eval() > sub_packets[1].eval()) as usize,
            Packet::Operator {
                kind: OperatorKind::LessThan,
                sub_packets,
                ..
            } => (sub_packets[0].eval() < sub_packets[1].eval()) as usize,
            Packet::Operator {
                kind: OperatorKind::EqualTo,
                sub_packets,
                ..
            } => (sub_packets[0].eval() == sub_packets[1].eval()) as usize,
        }
    }
}

pub fn solve_puzzle1<I: Iterator<Item = String>>(input_lines: I) -> String {
    let bits_packet_bytes = parse_input(input_lines);

    let mut bits = bits_packet_bytes.as_bitslice();

    let packet = Packet::parse_partial(&mut bits);

    packet.version_sum().to_string()
}

pub fn solve_puzzle2<I: Iterator<Item = String>>(input_lines: I) -> String {
    let bits_packet_bytes = parse_input(input_lines);

    let mut bits = bits_packet_bytes.as_bitslice();

    let packet = Packet::parse_partial(&mut bits);

    packet.eval().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        let example_str = "8A004A801A8002F478".to_owned();

        let bits_packet_bytes = parse_input(std::iter::once(example_str));

        let mut bits = bits_packet_bytes.as_bitslice();

        let packet = Packet::parse_partial(&mut bits);

        assert_eq!(
            packet,
            Packet::Operator {
                version: 4,
                kind: OperatorKind::Minimum,
                sub_packets: vec![Packet::Operator {
                    version: 1,
                    kind: OperatorKind::Minimum,
                    sub_packets: vec![Packet::Operator {
                        version: 5,
                        kind: OperatorKind::Minimum,
                        sub_packets: vec![Packet::Literal {
                            version: 6,
                            content: bitvec![u8, Msb0; 1; 4]
                        }]
                    }]
                }]
            }
        );
    }

    #[test]
    fn example_2() {
        let example_str = "620080001611562C8802118E34".to_owned();

        let bits_packet_bytes = parse_input(std::iter::once(example_str));

        let mut bits = bits_packet_bytes.as_bitslice();

        let packet = Packet::parse_partial(&mut bits);

        assert_eq!(
            packet,
            Packet::Operator {
                version: 3,
                kind: OperatorKind::Sum,
                sub_packets: vec![
                    Packet::Operator {
                        version: 0,
                        kind: OperatorKind::Sum,
                        sub_packets: vec![
                            Packet::Literal {
                                version: 0,
                                content: bitvec![u8, Msb0; 1, 0, 1, 0,],
                            },
                            Packet::Literal {
                                version: 5,
                                content: bitvec![u8, Msb0; 1, 0, 1, 1,],
                            },
                        ],
                    },
                    Packet::Operator {
                        version: 1,
                        kind: OperatorKind::Sum,
                        sub_packets: vec![
                            Packet::Literal {
                                version: 0,
                                content: bitvec![u8, Msb0; 1, 1, 0, 0,],
                            },
                            Packet::Literal {
                                version: 3,
                                content: bitvec![u8, Msb0; 1, 1, 0, 1,],
                            },
                        ],
                    },
                ],
            }
        );
    }
}
