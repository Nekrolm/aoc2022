use std::iter::Peekable;

use aoc2022::iter_ext::IteratorExt;

#[derive(Debug, Clone)]
enum Packet {
    Val(i64),
    Sub(Vec<Packet>),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}
impl Eq for Packet {}
impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other)
            .expect("all packets are lexically comparable")
    }
}
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Sub(vself), Self::Sub(vother)) => vself.partial_cmp(vother),
            (Self::Val(xself), Self::Val(yother)) => xself.partial_cmp(yother),
            (Self::Sub(vself), Self::Val(xother)) => {
                (&vself[..]).partial_cmp(&[Self::Val(*xother)])
            }
            (Self::Val(xself), Self::Sub(vother)) => (&[Self::Val(*xself)][..]).partial_cmp(vother),
        }
    }
}

fn parse_packet_item(chars: &mut Peekable<impl Iterator<Item = char>>) -> Option<Packet> {
    if let Some(_) = chars.next_if_eq(&'[') {
        let items = std::iter::from_fn(|| {
            let item = parse_packet_item(chars);
            let _ = chars.next_if_eq(&',');
            item
        })
        .collect();
        let _ = chars.next_if_eq(&']').expect("expected list termination");
        return Some(Packet::Sub(items));
    }
    std::iter::from_fn(|| {
        chars
            .next_if(|c| c.is_ascii_digit())
            .and_then(|d| d.to_digit(10).map(|d| d as i64))
    })
    .reduce(|acc, digit| acc * 10 + digit)
    .map(Packet::Val)
}

fn parse_packet(packet: &str) -> Packet {
    let mut chars = packet.chars().peekable();
    parse_packet_item(&mut chars).expect("expected packet")
}

fn main() {
    // part one
    let file = aoc2022::get_input_file();
    let idx_sum: i64 =
        aoc2022::parse_line_by_line(file, |line| (!line.is_empty()).then(|| parse_packet(line)))
            .flatten()
            .batching(|iter| {
                let first = iter.next()?;
                let second = iter.next()?;
                Some((first, second))
            })
            .zip(1..)
            .filter_map(|((left, right), idx)| (left < right).then_some(idx))
            .sum();

    println!("{idx_sum}");

    // part two
    let file = aoc2022::get_input_file();
    let first_term = parse_packet("[[2]]");
    let last_term = parse_packet("[[6]]");
    let mut packets: Vec<_> =
        aoc2022::parse_line_by_line(file, |line| (!line.is_empty()).then(|| parse_packet(line)))
            .flatten()
            .chain([first_term.clone(), last_term.clone()])
            .collect();
    packets.sort();

    let idx_prod: i64 = packets
        .iter()
        .zip(1..)
        .filter_map(|(p, idx)| (p == &first_term || p == &last_term).then_some(idx))
        .product();
    println!("{idx_prod}")
}
