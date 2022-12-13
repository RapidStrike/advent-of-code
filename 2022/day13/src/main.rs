use std::cmp::Ordering;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Eq, PartialEq, Clone)]
enum Packet {
    List(Vec<Packet>),
    Val(u32)
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::List(left), Packet::List(right)) => <[Packet]>::cmp(left, right),
            (Packet::List(left), &Packet::Val(right)) => <[Packet]>::cmp(left, &[Packet::Val(right)]),
            (&Packet::Val(left), Packet::List(right)) => <[Packet]>::cmp(&[Packet::Val(left)], right),
            (Packet::Val(left), Packet::Val(right)) => left.cmp(right)
        }
    }
}

fn main() {
    let mut it = include_str!("../input.txt").lines();
    let mut curr_a = 1;
    let mut total_a = 0;
    let mut packets: Vec<Packet> = Vec::new();
    while let Some(p1) = it.next() {
        let p2 = it.next().unwrap();
        let p1p: Packet = parse_packets(&mut p1.chars().peekable());
        let p2p: Packet = parse_packets(&mut p2.chars().peekable());
        packets.push(p1p.clone());
        packets.push(p2p.clone());
        match p1p.cmp(&p2p) {
            Ordering::Less => total_a += curr_a,
            _ => ()
        }
        curr_a += 1;
        // newline
        it.next();
    }
    // B
    let mark1 = parse_packets(&mut "[[2]]".chars().peekable());
    let mark2 = parse_packets(&mut "[[6]]".chars().peekable());
    packets.push(mark1.clone());
    packets.push(mark2.clone());
    packets.sort();
    let mark1p = packets.binary_search(&mark1).unwrap() + 1;
    let mark2p = packets.binary_search(&mark2).unwrap() + 1;

    println!("Pair Sum (A): {}", total_a);
    println!("Decoder Key (B): {}", mark1p * mark2p);
}

fn parse_packets(p: &mut Peekable<Chars<'_>>) -> Packet {
    let mut list = Vec::new();
    while let Some(c) = p.next() {
        match c {
            '0'..='9' => {
                let mut num = c.to_digit(10).unwrap();
                while let Some('0'..='9') = p.peek() {
                    num = num * 10 + p.next().unwrap().to_digit(10).unwrap();
                }
                list.push(Packet::Val(num));
            },
            '[' => list.push(parse_packets(p)),
            ']' => return Packet::List(list),
            _ => continue
        }
    }
    return Packet::List(list);
}
