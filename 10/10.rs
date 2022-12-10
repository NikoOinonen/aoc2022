
use std::{fs, str};

fn main() {

    let input = fs::read_to_string("input.txt").expect("Reading input");

    let mut cycle = 1;
    let mut wait = 0;
    let mut v = 0;
    let mut x = 1;
    let mut instruction = "";
    let mut signal_strengths = 0;
    let mut lines = str::lines(&input).into_iter();

    loop {
        
        if (cycle - 20) % 40 == 0 {
            signal_strengths += cycle * x;
        }

        if cycle <= 240 {
            let col = (cycle - 1) % 40 + 1;
            let dist = col - x;
            if dist >= 0 && dist <= 2 {
                print!("#");
            } else {
                print!(".");
            }
            if col == 40 {
                print!("\n");
            }
        }

        if wait == 0 {
            instruction = match lines.next() {
                Some(s) => s.trim(),
                None => "stop"
            };
            match instruction {
                "noop" => wait = 1,
                _ if instruction.contains("addx") => {
                    wait = 2;
                    v = instruction.split_whitespace().collect::<Vec<&str>>()[1].parse().unwrap();
                },
                "stop" => (),
                _ => panic!("Should not happen")
            }
        }

        wait -= 1;
        if wait == 0 && instruction.contains("addx") {
            x += v;
        }

        if instruction == "stop" {
            break;
        }

        cycle += 1;

    }

    println!("Sum of signal stengths: {signal_strengths}");

}
