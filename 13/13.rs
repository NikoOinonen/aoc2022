
use std::fmt::Display;
use std::fs;
use std::str::FromStr;
use std::result::Result::Ok;
use std::cmp::Ordering;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Packet {
    Int(i32),
    List(Vec<Packet>)
}

impl Packet {
    fn push(&mut self, packet: Packet) {
        match self {
            Packet::List(p) => p.push(packet),
            _ => panic!("Cannot push into integer.")
        }
    }
}

impl FromStr for Packet {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut packet = Packet::List(Vec::new());
        let mut nesting_level = 0;
        let mut digit_chars = String::new();
        for (ic, c) in s.chars().enumerate() {
            if ic == 0 {continue;}
            match c {
                _ if c.is_digit(10) => digit_chars.push(c),
                '[' => {
                    if nesting_level == 0 {
                        let nested_packet = s.get(ic..).unwrap().parse::<Packet>();
                        match nested_packet {
                            Ok(p) => packet.push(p),
                            Err(()) => return Err(())
                        }
                    }
                    nesting_level += 1;
                    
                },
                ']' => {
                    if nesting_level == 0 {
                        if digit_chars.len() > 0 {
                            packet.push(Packet::Int(digit_chars.parse().unwrap()));
                        }
                        break;
                    }
                    digit_chars = String::new();
                    nesting_level -= 1;
                },
                ',' => {
                    if nesting_level == 0 && digit_chars.len() > 0 {
                        packet.push(Packet::Int(digit_chars.parse().unwrap()));
                    }
                    digit_chars = String::new();
                },
                _ => return Err(())
            };
        }
        return Ok(packet)
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Int(l), Packet::Int(r)) => l.cmp(r),
            (Packet::List(l), Packet::List(r)) => l.cmp(&r),
            (Packet::Int(l), Packet::List(r)) => Packet::List(vec![Packet::Int(*l)]).cmp(&Packet::List(r.to_vec())),
            (Packet::List(l), Packet::Int(r)) => Packet::List(l.to_vec()).cmp(&Packet::List(vec![Packet::Int(*r)]))
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::Int(v) => write!(f, "{}", v),
            Packet::List(l) => {
                write!(f, "[")?;
                for (i, v) in l.iter().enumerate() {
                    Packet::fmt(&v, f)?;
                    if i < l.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}

fn main() {

    let input = fs::read_to_string("input.txt").unwrap();

    let mut right_order_inds = Vec::new();
    let mut all_packets = Vec::new();
    for (ind, pair_str) in input.split("\n\n").enumerate() {
        let pair = pair_str.split("\n").collect::<Vec<&str>>();
        let left: Packet = pair[0].parse().unwrap();
        let right: Packet = pair[1].parse().unwrap();
        all_packets.push(left.clone());
        all_packets.push(right.clone());
        if left < right {
            right_order_inds.push(ind + 1);
        }
    }

    println!("Sum of right order indices: {}", right_order_inds.into_iter().sum::<usize>());

    let divider1 = "[[2]]".parse::<Packet>().unwrap();
    let divider2 = "[[6]]".parse::<Packet>().unwrap();
    all_packets.push(divider1.clone());
    all_packets.push(divider2.clone());
    all_packets.sort();
    let mut decode_key = 1;
    for (ind, packet) in all_packets.iter().enumerate() {
        if *packet == divider1 || *packet == divider2 {
            decode_key *= ind + 1;
        }
    }

    println!("Decode key: {decode_key}");

}
