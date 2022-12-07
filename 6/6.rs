
use std::fs;
use std::collections::{VecDeque, HashSet};

fn find_unique_string(s: &str, length: usize) -> i32 {
    let mut buf = VecDeque::new();
    let mut marker_end = 0;
    for c in s.chars() {
        marker_end += 1;
        buf.push_back(c);
        if buf.len() > length {
            buf.pop_front();
        }
        if buf.len() == length {
            let char_set: HashSet<char> = buf.clone().into_iter().collect();
            if char_set.len() == length {
                break;
            }
        }
    }
    return marker_end
}

fn main() {

    let input = fs::read_to_string("input.txt").expect("Reading input");

    println!("Packet position: {}", find_unique_string(&input, 4));
    println!("Message position: {}", find_unique_string(&input, 14));

}
