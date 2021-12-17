use std::fs;
use std::num::ParseIntError;
use PacketType::*;

pub fn input_packet() -> Packet {
    let input =
        fs::read_to_string("day_16/input").expect("Failed to read input file");
    let binary = hex_str_to_binary_str(&input);
    Packet::parse_one(&binary).expect("Bad input").0
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Packet {
    version: u8,
    ptype: PacketType,
}

impl Packet {
    fn parse_many(s: &str) -> Result<Vec<Packet>, ParseIntError> {
        let mut packets = Vec::new();
        let mut to_parse = s;
        loop {
            let (packet, remaining) = Packet::parse_one(to_parse)?;
            packets.push(packet);
            if remaining.is_empty() || remaining.chars().all(|c| c == '0') {
                return Ok(packets);
            } else {
                to_parse = remaining;
            }
        }
    }

    fn parse_n(
        s: &str,
        n: usize,
    ) -> Result<(Vec<Packet>, &str), ParseIntError> {
        let mut packets = Vec::with_capacity(n);
        let mut to_parse = s;
        for _ in 0..n {
            let (packet, remaining) = Packet::parse_one(to_parse)?;
            //println!("Got packet: {:?}\nRemaining {:?}", &packet, remaining);
            packets.push(packet);
            if remaining.is_empty() || remaining.chars().all(|c| c == '0') {
                return Ok((packets, remaining));
            }
            to_parse = remaining;
        }
        Ok((packets, to_parse))
    }

    fn parse_one(s: &str) -> Result<(Self, &str), ParseIntError> {
        let version = u8::from_str_radix(&s[..3], 2)?;
        let type_id = u8::from_str_radix(&s[3..6], 2)?;
        match type_id {
            4 => {
                // Literal value
                let mut point = 6;
                let mut total = String::new();
                while s.chars().nth(point).expect("Bad input") == '1' {
                    total.push_str(&s[point + 1..point + 5]);
                    point += 5;
                }
                total.push_str(&s[point + 1..point + 5]);
                assert!(total.len() <= 64, "Need bigger number type");
                let packet = Packet {
                    version,
                    ptype: Literal(u64::from_str_radix(&total, 2)?),
                };
                Ok((packet, &s[point + 5..]))
            }
            operator => {
                match s.chars().nth(6).expect("Bad input") {
                    '0' => {
                        // The next 15 bits are a number that represents the
                        // total length in bits of the sub-packets contained by
                        // this packet
                        let point = usize::from_str_radix(&s[7..22], 2)?;
                        let sub_packets = Packet::parse_many(&s[22..22 + point])?;
                        let packet = Packet {
                            version,
                            ptype: Operator {
                                operator,
                                sub_packets,
                            }
                        };
                        Ok((packet, &s[22 + point..]))
                    }
                    '1' => {
                        // The next 11 bits are a number that represents the
                        // number of sub-packets immediately contained by this
                        // packet
                        let sub_packet_count =
                            usize::from_str_radix(&s[7..18], 2)?;
                        //println!("Parsing subpackets from {:?}", &s[18..]);
                        let (sub_packets, remaining) =
                            Packet::parse_n(&s[18..], sub_packet_count)?;
                        let packet = Packet {
                            version,
                            ptype: Operator {
                                operator,
                                sub_packets,
                            },
                        };
                        Ok((packet, remaining))
                    }
                    _ => unreachable!("I must have got the indexes wrong"),
                }
            }
        }
    }

    pub fn version_total(&self) -> u32 {
        let sub_packet_total = match &self.ptype {
            Literal(_) => 0,
            Operator { sub_packets, .. } => sub_packets.iter().map(Packet::version_total).sum::<u32>(),
        };
        self.version as u32 + sub_packet_total
    }

    pub fn evaluate(&self) -> u64 {
        match &self.ptype {
            Literal(n) => *n,
            Operator { operator, sub_packets } => {
                let operands_iter = sub_packets.iter().map(Packet::evaluate);
                match *operator {
                    // Sum
                    0 => operands_iter.sum::<u64>(),
                    // Product
                    1 => operands_iter.product::<u64>(),
                    // Minimum
                    2 => operands_iter.min().unwrap(),
                    // Maximum
                    3 => operands_iter.max().unwrap(),
                    5 => {
                        // Greater than
                        let v = operands_iter.collect::<Vec<_>>();
                        assert_eq!(v.len(), 2);
                        (v[0] > v[1]) as u64
                    }
                    6 => {
                        // Less than
                        let v = operands_iter.collect::<Vec<_>>();
                        assert_eq!(v.len(), 2);
                        (v[0] < v[1]) as u64
                    }
                    7 => {
                        // Equal to
                        let v = operands_iter.collect::<Vec<_>>();
                        assert_eq!(v.len(), 2);
                        (v[0] == v[1]) as u64
                    }
                    _ => unreachable!("Bad packet: {:?}", self),
                }
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum PacketType {
    Literal(u64),
    Operator {
        operator: u8,
        sub_packets: Vec<Packet>,
    },
}

fn hex_str_to_binary_str(s: &str) -> String {
    s.trim()
        .chars()
        .map(|c| match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => panic!("Invalid hex character"),
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_literal() {
        let input = "110100101111111000101000";
        let (actual, r) = Packet::parse_one(input).unwrap();
        assert_eq!(
            actual,
            Packet {
                version: 6,
                ptype: Literal(2021),
            },
        );
        assert_eq!(r, "000");
    }

    #[test]
    fn parse_operator_ltype_0() {
        let input = "00111000000000000110111101000101001010010001001000000000";
        let (actual, r) = Packet::parse_one(input).unwrap();
        assert_eq!(
            actual,
            Packet {
                version: 1,
                ptype: Operator {
                    operator: 6,
                    sub_packets: vec![
                        Packet {
                            version: 6,
                            ptype: Literal(10),
                        },
                        Packet {
                            version: 2,
                            ptype: Literal(20),
                        },
                    ],
                },
            },
        );
        assert_eq!(r, "0000000");
    }

    #[test]
    fn parse_operator_ltype_1() {
        let input = "11101110000000001101010000001100100000100011000001100000";
        let (actual, r) = Packet::parse_one(input).unwrap();
        assert_eq!(
            actual,
            Packet {
                version: 7,
                ptype: Operator {
                    operator: 3,
                    sub_packets: vec![
                        Packet {
                            version: 2,
                            ptype: Literal(1),
                        },
                        Packet {
                            version: 4,
                            ptype: Literal(2),
                        },
                        Packet {
                            version: 1,
                            ptype: Literal(3),
                        },
                    ],
                },
            },
        );
        assert_eq!(r, "00000");
    }
}
