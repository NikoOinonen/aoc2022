
#![feature(iter_next_chunk)]

use std::{str, fs, fmt::{Display, Result, Formatter}};

#[derive(Clone)]
struct CargoStacks {
    stacks: Vec<Vec<char>>
}

impl Display for CargoStacks {
    fn fmt(&self, f: &mut Formatter) -> Result {
        for (i, stack) in self.stacks.iter().enumerate() {
            if i != 0 {write!(f, "\n")?;}
            write!(f, "{}: [", i + 1)?;
            for (j, cargo) in stack.iter().enumerate() {
                if j != 0 {write!(f, ", ")?;}
                write!(f, "{}", cargo)?;
            }
            write!(f, "]",)?;
        }
        write!(f, "")
    }
}

impl CargoStacks {
    fn move_cargo_single(&mut self, amount: i32, from: usize, into: usize) {
        for _ in 0..amount {
            let cargo = self.stacks[from].pop().unwrap();
            self.stacks[into].push(cargo);
        }
    }
    fn move_cargo_multiple(&mut self, amount: i32, from: usize, into: usize) {
        let rem_ind = self.stacks[from].len() - (amount as usize);
        for _ in 0..amount {
            let cargo = self.stacks[from].remove(rem_ind);
            self.stacks[into].push(cargo);
        }
    }
    fn get_top_cargo(self) -> Vec<char> {
        let mut cargo = Vec::new();
        for stack in self.stacks.iter() {
            cargo.push(stack[stack.len() - 1])
        }
        cargo
    }
}

fn main() {

    let contents = fs::read_to_string("input.txt").expect("Reading input");

    let mut lines = str::lines(&contents);
    let mut cargo_stacks = CargoStacks{ stacks: Vec::new() };
    let mut first_line = true;
    loop {
        let line = lines.next().unwrap();
        if line.trim() == "" || !line.contains('[') {break;}
        let mut j = 0;
        let mut line_chars = line.chars();
        loop {
            let cargo_char: char = match line_chars.next_chunk::<4>() {
                Ok(c) => c[1],
                Err(c) => {
                    let c_ = c.as_slice();
                    if c_.len() == 0 {
                        break;
                    }
                    c_[1]
                }
            };
            if first_line {
                cargo_stacks.stacks.push(Vec::new());
            }
            let stack = &mut cargo_stacks.stacks[j];
            if cargo_char != ' ' {
                stack.insert(0, cargo_char);
            }
            j += 1;
        }
        first_line = false;
    }

    let mut cargo_stacks2 = cargo_stacks.clone();

    for line in lines {
        if line.trim() == "" {continue;}
        let moves: Vec<i32> = line
            .split_whitespace().collect::<Vec<&str>>()[1..]
            .iter().step_by(2).map(|s| s.parse::<i32>().unwrap())
            .collect();
        cargo_stacks.move_cargo_single(moves[0], (moves[1] - 1) as usize, (moves[2] - 1) as usize);
        cargo_stacks2.move_cargo_multiple(moves[0], (moves[1] - 1) as usize, (moves[2] - 1) as usize);
    }

    println!("Top cargo (one at a time): {}", cargo_stacks.get_top_cargo().iter().collect::<String>());
    println!("Top cargo (multiple at a time): {}", cargo_stacks2.get_top_cargo().iter().collect::<String>());

}
