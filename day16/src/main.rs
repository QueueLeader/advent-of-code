use anyhow::{Context};

type Num = u64;

#[derive(Debug)]
enum Operator {
    Operator {
        version: u32,
        type_id: OpType,
        packets: Vec<Operator>,
    },
    Literal {
        version: u32,
        value: Num,
    },
}

#[derive(Debug)]
enum OpType {
    Sum,
    Prod,
    Min,
    Max,
    Gt,
    Lt,
    Eq,
}

fn to_num(b: bool) -> Num {
    if b { 1 } else { 0 }
}

impl OpType {
    fn from_num(num: u32) -> Self {
        match num {
            0 => Self::Sum,
            1 => Self::Prod,
            2 => Self::Min,
            3 => Self::Max,
            5 => Self::Gt,
            6 => Self::Lt,
            7 => Self::Eq,
            _ => panic!("Invalid type id: {}", num)
        }
    }

    fn evaluate(&self, packets: &[Operator]) -> Num {
        let mut iter = packets.iter().map(|p| p.evaluate());
        match self {
            Self::Sum => iter.sum(),
            Self::Prod => iter.product(),
            Self::Min => iter.min().unwrap(),
            Self::Max => iter.max().unwrap(),
            Self::Gt => to_num(iter.next().unwrap() > iter.next().unwrap()),
            Self::Lt => to_num(iter.next().unwrap() < iter.next().unwrap()),
            Self::Eq => to_num(iter.next().unwrap() == iter.next().unwrap()),
        }
    }
}

#[derive(Debug)]
enum Binary {
    Zero,
    One,
}

impl Binary {
    fn from_hex(s: &str) -> anyhow::Result<Vec<Self>> {
        let mut digits = Vec::with_capacity(s.len() * 4);

        for c in s.chars() {
            let digit = c.to_digit(16)
                .with_context(|| format!("Invalid digit: {}", c))?;
            digits.push(Self::from_bit(digit & 0b1000 == 0b1000));
            digits.push(Self::from_bit(digit & 0b0100 == 0b0100));
            digits.push(Self::from_bit(digit & 0b0010 == 0b0010));
            digits.push(Self::from_bit(digit & 0b0001 == 0b0001));
        }

        anyhow::Ok(digits)
    }

    fn from_bit(bit: bool) -> Self {
        if bit {
            Self::One
        } else {
            Self::Zero
        }
    }

    fn num(&self) -> u32 {
        match self {
            Self::One => 1,
            Self::Zero => 0,
        }
    }
}

fn from_bin(bin: &[Binary]) -> u32 {
    bin.iter().fold(0, |sum, n| sum * 2 + n.num())
}

fn get_value(bin: &[Binary]) -> (usize, Num) {
    let mut sum = 0;
    let mut count = 0;

    for b in bin.chunks(5) {
        sum *= 16;
        sum += from_bin(&b[1..5]) as Num;
        count += 5;

        if let Binary::Zero = b[0] {
            break;
        }
    }

    (count, sum)
}

fn get_packets(bin: &[Binary]) -> (usize, Vec<Operator>) {
    let mut packets = vec![];
    let mut curr_len;

    if let Binary::One = bin[0] {
        let size = from_bin(&bin[1..12]) as usize;
        packets.reserve(size);
        curr_len = 12;

        for _ in 0 .. size {
            let (len, packet) = Operator::from_binary(&bin[curr_len..]);
            curr_len += len;
            packets.push(packet);
        }
    } else {
        let bits = from_bin(&bin[1..16]) as usize;
        curr_len = 16;

        while curr_len - 16 < bits {
            let (len, packet) = Operator::from_binary(&bin[curr_len..16+bits]);
            curr_len += len;
            packets.push(packet);
        }
    }

    (curr_len, packets)
}

impl Operator {
    fn from_binary(bin: &[Binary]) -> (usize, Self) {
        let version = from_bin(&bin[0..3]);
        let type_id = from_bin(&bin[3..6]);

        match type_id {
            4 => {
                let (len, value) = get_value(&bin[6..]);
                (len + 6, Self::Literal {
                    version,
                    value,
                })
            },
            _ => {
                let (len, packets) = get_packets(&bin[6..]);
                (len + 6, Self::Operator {
                    version,
                    type_id: OpType::from_num(type_id),
                    packets,
                })
            }
        }
    }

    fn sum_version(&self) -> u32 {
        match self {
            Self::Literal {version, ..} => *version,
            Self::Operator {version, packets, ..} => *version
                + packets.iter().fold(0, |sum, p| sum + p.sum_version()),
        }
    }

    fn evaluate(&self) -> Num {
        match self {
            Self::Literal {value, ..} => *value,
            Self::Operator {type_id, packets, ..} =>
                type_id.evaluate(packets),
        }
    }
}

pub fn main() -> anyhow::Result<()> {
    let binary = Binary::from_hex(include_str!("../input.txt").trim())?;
    let packet = Operator::from_binary(&binary).1;

    // Part 1
    println!("{}", packet.sum_version());

    // Part 2
    println!("{}", packet.evaluate());

    /*
    // Sample Inputs
    let inputs = "C200B40A82
        04005AC33890
        880086C3E88112
        CE00C43D881120
        D8005AC2A8F0
        F600BC2D8F
        9C005AC2F8F0
        9C0141080250320F1802104A08";

    for input in inputs.lines() {
        let packet = Operator::from_binary(
            &Binary::from_hex(input.trim())?).1;
        println!("{}", packet.evaluate());
    }
    */

    Ok(())
}
