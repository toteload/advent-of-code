use std::cmp;
use std::io;

#[derive(Debug)]
struct BitVec {
    bits: Vec<bool>,
}

fn bits_to_u64(bits: &[bool]) -> u64 {
    assert!(bits.len() <= 64);

    let mut acc: u64 = 0;

    for (i, b) in bits.iter().rev().enumerate() {
        acc |= (if *b { 1 } else { 0 }) << i;
    }

    acc
}

fn hex_to_bits(c: u8) -> [bool; 4] {
    let x = match c {
        b'0'..=b'9' => (c - b'0') as u8,
        b'A'..=b'F' => (c - b'A') as u8 + 10,
        _ => unreachable!(),
    };

    [
        (x & 0x8) != 0,
        (x & 0x4) != 0,
        (x & 0x2) != 0,
        (x & 0x1) != 0,
    ]
}

impl BitVec {
    fn new() -> Self {
        BitVec { bits: Vec::new() }
    }

    fn from_hexstring(s: &str) -> Self {
        let mut res = BitVec::new();

        for b in s.as_bytes() {
            res.push_bits(&hex_to_bits(*b));
        }

        res
    }

    fn push_bits(&mut self, bits: &[bool]) {
        self.bits.extend_from_slice(bits);
    }

    fn read(&self, offset: usize, n: usize) -> u64 {
        assert!(offset + n <= self.bits.len());
        assert!(offset <= self.bits.len());
        assert!(n <= 64);

        bits_to_u64(&self.bits[offset..(offset + n)])
    }

    fn reader(&self, offset: usize, n: usize) -> BitReader {
        assert!(offset + n <= self.bits.len());
        assert!(offset <= self.bits.len());

        BitReader {
            src: self,
            offset,
            len: n,
        }
    }
}

struct BitReader<'a> {
    src: &'a BitVec,
    offset: usize,
    len: usize,
}

impl<'a> BitReader<'a> {
    fn read(&mut self, n: usize) -> u64 {
        let res = self.src.read(self.offset, n);
        self.offset += n;
        res
    }
}

#[derive(Debug)]
enum Op {
    Sum,
    Product,
    Minimum,
    Maximum,
    GT,
    LT,
    EQ,
}

#[derive(Debug)]
enum PacketVariant {
    Literal(u64),
    Operator(Op, Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: u8,
    id: u8,
    val: PacketVariant,
}

impl Packet {
    fn from_bits(bits: &mut BitReader) -> Self {
        let version = bits.read(3) as u8;
        let id = bits.read(3) as u8;

        if id == 4 {
            let mut parts = Vec::new();

            loop {
                let group = bits.read(5);

                parts.push(group & 0xf);

                if (group & 0x10) == 0 {
                    break;
                }
            }

            let mut acc: u64 = 0;

            for (i, part) in parts.iter().rev().enumerate() {
                acc |= part << (i * 4);
            }

            Packet {
                version,
                id,
                val: PacketVariant::Literal(acc),
            }
        } else {
            let len_id = bits.read(1);

            let mut subs = Vec::new();

            if len_id == 0 {
                let sub_packet_len = bits.read(15);

                let end = bits.offset + sub_packet_len as usize;

                loop {
                    subs.push(Packet::from_bits(bits));

                    if bits.offset >= end {
                        break;
                    }
                }
            } else {
                let sub_packet_count = bits.read(11);

                for _ in 0..sub_packet_count {
                    subs.push(Packet::from_bits(bits));
                }
            }

            let op = match id {
                0 => Op::Sum,
                1 => Op::Product,
                2 => Op::Minimum,
                3 => Op::Maximum,
                5 => Op::GT,
                6 => Op::LT,
                7 => Op::EQ,
                _ => unreachable!(),
            };

            Packet {
                version,
                id,
                val: PacketVariant::Operator(op, subs),
            }
        }
    }
}

fn version_sum(packet: &Packet) -> u64 {
    (packet.version as u64)
        + match &packet.val {
            PacketVariant::Operator(_, packs) => packs.iter().map(version_sum).sum(),
            _ => 0,
        }
}

fn eval(p: &Packet) -> u64 {
    match &p.val {
        PacketVariant::Literal(x) => *x,
        PacketVariant::Operator(op, xs) => match op {
            Op::Sum => xs.iter().map(|x| eval(x)).sum(),
            Op::Product => xs.iter().map(|x| eval(x)).product(),
            Op::Minimum => xs
                .iter()
                .map(eval)
                .reduce(cmp::min)
                .unwrap(),
            Op::Maximum => xs
                .iter()
                .map(eval)
                .reduce(cmp::max)
                .unwrap(),
            Op::GT => {
                if eval(&xs[0]) > eval(&xs[1]) {
                    1
                } else {
                    0
                }
            }
            Op::LT => {
                if eval(&xs[0]) < eval(&xs[1]) {
                    1
                } else {
                    0
                }
            }
            Op::EQ => {
                if eval(&xs[0]) == eval(&xs[1]) {
                    1
                } else {
                    0
                }
            }
        },
    }
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    println!("[INPUT] {}", line.trim());

    let bits = BitVec::from_hexstring(line.trim());

    let packets = Packet::from_bits(&mut bits.reader(0, bits.bits.len()));

    println!("{}", eval(&packets));
}
