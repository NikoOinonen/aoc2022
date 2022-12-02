
use std::fs;
use std::str;
use std::result::Result;

const HAND_SCORES: [i32; 3] = [1, 2, 3];
const GAME_SCORES: [i32; 3] = [3, 6, 0];

#[derive(Copy, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors
}

fn hand_from_str(s: &str) -> Result<Hand, ()> {
    match s {
        "A" | "X" => Result::Ok(Hand::Rock),
        "B" | "Y" => Result::Ok(Hand::Paper),
        "C" | "Z" => Result::Ok(Hand::Scissors),
        &_ => Result::Err(())
    }
}

fn score_from_str1(s: &str) -> Result<i32, ()> {
    if s.len() < 3 {
        return Result::Err(());
    }
    let hand1 = match hand_from_str(&s[..1]) {
        Ok(h) => h,
        Err(_) => return Result::Err(())
    };
    let hand2 = match hand_from_str(&s[2..]) {
        Ok(h) => h,
        Err(_) => return Result::Err(())
    };
    let game_ind = ((hand2 as i32 - hand1 as i32 + 3) % 3) as usize;
    let hand_ind = hand2 as usize;
    Result::Ok(HAND_SCORES[hand_ind] + GAME_SCORES[game_ind])
}

fn score_from_str2(s: &str) -> Result<i32, ()> {
    if s.len() < 3 {
        return Result::Err(());
    }
    let hand1 = match hand_from_str(&s[..1]) {
        Ok(h) => h,
        Err(_) => return Result::Err(())
    };
    let (game_ind, offset): (usize, i32) = match &s[2..] {
        "X" => (2, -1),
        "Y" => (0, 0),
        "Z" => (1, 1),
        &_ => return Result::Err(())
    };
    let hand_ind = ((hand1 as i32 + offset + 3) % 3) as usize;
    Result::Ok(HAND_SCORES[hand_ind] + GAME_SCORES[game_ind])
}

fn main() {

    let contents = fs::read_to_string("input.txt").expect("Reading contents");

    let mut score1 = 0;
    let mut score2 = 0;
    for line in str::lines(&contents) {
        match score_from_str1(line){
            Err(_) => continue,
            Ok(s) => score1 += s
        };
        match score_from_str2(line){
            Err(_) => continue,
            Ok(s) => score2 += s
        };
    }

    println!("Total score 1: {}", score1);
    println!("Total score 2: {}", score2);

}
