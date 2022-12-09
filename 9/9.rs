
use std::{fs, str, cmp::{min, max}, collections::HashSet};

fn pull_rope(rope_len: usize, input: String) -> i32 {
    let mut rope_pos = vec![(0, 0); rope_len];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    for line in str::lines(&input) {
        if line.trim().len() == 0 {continue;}
        let line_split = line.trim().split_whitespace().collect::<Vec<&str>>();
        let direction = line_split[0];
        let amount: u32 = line_split[1].parse().unwrap();
        for _ in 0..amount {
            match direction {
                "U" => rope_pos[0].1 += 1,
                "D" => rope_pos[0].1 -= 1,
                "R" => rope_pos[0].0 += 1,
                "L" => rope_pos[0].0 -= 1,
                _ => panic!("Unhandled direction: {}", direction)
            }
            for i in 1..rope_len {
                if i32::abs(rope_pos[i-1].0 - rope_pos[i].0) > 1 || i32::abs(rope_pos[i-1].1 - rope_pos[i].1) > 1 {
                    rope_pos[i].0 += i32::signum(rope_pos[i-1].0 - rope_pos[i].0);
                    rope_pos[i].1 += i32::signum(rope_pos[i-1].1 - rope_pos[i].1);
                }
                rope_pos[i].0 = min(rope_pos[i].0, rope_pos[i-1].0 + 1);
                rope_pos[i].0 = max(rope_pos[i].0, rope_pos[i-1].0 - 1);
                rope_pos[i].1 = min(rope_pos[i].1, rope_pos[i-1].1 + 1);
                rope_pos[i].1 = max(rope_pos[i].1, rope_pos[i-1].1 - 1);
                visited.insert(rope_pos[rope_len-1]);
            }
        }
    }

    visited.len() as i32

}

fn main() {

    let input = fs::read_to_string("input.txt").expect("Reading input");

    println!("Number of visited positions (length 2): {}", pull_rope(2, input.clone()));
    println!("Number of visited positions (length 10): {}", pull_rope(10, input.clone()));

}
