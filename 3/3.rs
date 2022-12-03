
#![feature(iter_next_chunk)]

use std::fs;
use std::str;
use std::collections::HashSet;

fn char_value(c: char) -> i32 {
    let c_int = c as u8;
    if c_int >= ('a' as u8) {
        return (c_int - ('a' as u8) + 1) as i32;
    } else {
        return (c_int - ('A' as u8) + 27) as i32;
    }
}

fn main() {

    let contents = fs::read_to_string("input.txt").expect("Reading contents");

    // Part 1
    let mut sum: i32 = 0;
    for line in str::lines(&contents) {
        let first_half = &line[..(line.len()/2)];
        let second_half = &line[(line.len()/2)..];
        for c in first_half.chars() {
            if second_half.contains(c) {
                sum += char_value(c);
                break;
            }
        }
    }
    println!("Total sum 1: {}", sum);

    // Part 2
    let mut sum: i32 = 0;
    let mut lines = str::lines(&contents);
    loop {
        let line_chunk = match lines.next_chunk::<3>() {
            Ok(v) => v,
            Err(_) => break
        };
        let unique_char = line_chunk
            .iter().map(|line| line.chars().collect::<HashSet<char>>())
            .reduce(|prev, chars| &prev & &chars).unwrap()
            .into_iter().collect::<Vec<char>>()[0];
        sum += char_value(unique_char);
    }
    println!("Total sum 2: {}", sum);

}
