
use std::{fs, collections::VecDeque};

struct Monkey {
    items: VecDeque<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    divisor: u64,
    targets: (i32, i32),
    num_inspected: u32
}

impl Monkey {
    fn throw(&mut self) -> Option<(u64, usize)> {
        match self.items.pop_front() {
            None => None,
            Some(item) => {
                self.num_inspected += 1;
                // println!("{item}");
                let new_item = (self.operation)(item);
                let target = match new_item % self.divisor {
                    0 => self.targets.0,
                    _ => self.targets.1
                };
                Some((new_item, target as usize))
            }
        }
    }
    fn receive(&mut self, item: u64) {
        self.items.push_back(item);
    }
}

fn main() {

    let input = fs::read_to_string("input.txt").expect("Reading input");
    let mut monkeys: Vec<Monkey> = Vec::new();

    for block in input.split("\n\n") {
        let mut lines = block.split("\n");
        lines.next();
        let items: VecDeque<u64> = lines.next().unwrap()
            .split(":").last().unwrap()
            .split(",").map(|s| match s.trim().parse() {
                Ok(i) => i,
                Err(_) => {println!("Could not parse {s}"); 0},
            })
            .collect();
        let operation: Box<dyn Fn(u64) -> u64> = match lines.next().unwrap().split("=").last().unwrap() {
            s if s.contains("+") => {
                let other: u64 = s.split("+").last().unwrap().trim().parse().unwrap();
                Box::new(move |old:u64| old + other)
            },
            s if s.contains("*") => {
                let other = s.split("*").last().unwrap().trim();
                if other == "old" {
                    Box::new(move |old: u64| old * old)
                } else {
                    let other: u64 = other.parse().unwrap();
                    Box::new(move |old: u64| old * other)
                }
            }
            _ => unreachable!()
        };
        let divisor = lines.next().unwrap().split_whitespace().last().unwrap().parse().unwrap();
        let mut t: Vec<i32> = Vec::new();
        for _ in 0..2 {
            t.push(lines.next().unwrap().split_whitespace().last().unwrap().parse().unwrap())
        }
        let targets = match t[..] {
            [a, b, ..] => (a, b),
            _ => unreachable!()
        };
        monkeys.push(Monkey { items, operation, divisor, targets, num_inspected: 0 });
    }

    let common_multiple: u64 = monkeys.iter().map(|m| m.divisor).product();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while let Some((item, target)) = monkeys[i].throw() {
                monkeys[target].receive(item % common_multiple);
            }
        }
    }

    let mut num_inspections = monkeys.iter().map(|m| m.num_inspected as u64).collect::<Vec<u64>>();
    println!("{num_inspections:?}");
    num_inspections.sort();
    num_inspections.reverse();
    println!("Activity level {}", num_inspections[0] * num_inspections[1]);

}
