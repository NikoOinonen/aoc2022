
use std::{fs, collections::HashSet};

#[derive(Debug, Clone)]
enum Shape {
    Horizontal,
    Plus,
    L,
    Vertical,
    Square
}

type Rock = Vec<(u64, u64)>;

fn new_rock(shape: Shape, h: u64) -> Rock {
    match shape {
        Shape::Horizontal => vec![(3, h), (4, h), (5, h), (6, h)],
        Shape::Vertical => vec![(3, h), (3, h + 1), (3, h + 2), (3, h + 3)],
        Shape::Square => vec![(3, h), (4, h), (3, h + 1), (4, h + 1)],
        Shape::Plus => vec![(4, h), (3, h + 1), (4, h + 1), (4, h + 2), (5, h + 1)],
        Shape::L => vec![(3, h), (4, h), (5, h), (5, h + 1), (5, h + 2)]
    }
}

fn move_horizontal(rock: &mut Rock, pile: &HashSet<(u64, u64)>, direction: char) {
    match direction {
        '>' => {
            let x_max = rock.iter().map(|c| c.0).max().unwrap();
            if x_max >= 7 {return;}
            let mut blocked = false;
            for c in rock.iter() {
                if pile.contains(&(c.0 + 1, c.1)) {
                    blocked = true;
                    break;
                }
            }
            if !blocked {
                for c in rock {
                    c.0 += 1;
                }
            }
        },
        '<' => {
            let x_min = rock.iter().map(|c| c.0).min().unwrap();
            if x_min <= 1 {return;}
            let mut blocked = false;
            for c in rock.iter() {
                if pile.contains(&(c.0 - 1, c.1)) {
                    blocked = true;
                    break;
                }
            }
            if !blocked {
                for c in rock {
                    c.0 -= 1;
                }
            }
        }
        _ => panic!("Invalid direction {}", direction)
    }
}

fn move_down(rock: &mut Rock, pile: &HashSet<(u64, u64)>) -> bool {
    let mut blocked = false;
    for c in rock.iter() {
        if pile.contains(&(c.0, c.1 - 1)) {
            blocked = true;
            break;
        }
    }
    if !blocked {
        for c in rock {
            c.1 -= 1;
        }
    }
    !blocked
}

fn print_column(pile: &HashSet<(u64, u64)>, rock: &Rock, limit: usize) {
    let top = rock.iter().map(|c| c.1).max().unwrap();
    for (iy, y) in (1..top+2).rev().enumerate() {
        if iy > limit {break;}
        print!("{y:>5}|");
        for x in 1..8 {
            if rock.contains(&(x, y)) {
                print!("@");
            } else if pile.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".")
            }
        }
        print!("|\n");
    }
    print!("     +");
    for _ in 0..7 {
        print!("-");
    }
    print!("+\n");
}

fn main() {

    let input = fs::read_to_string("input.txt").unwrap();
    let target_rocks: u64 = 1_000_000_000_000;

    let mut shape_list = [
        Shape::Horizontal,
        Shape::Plus,
        Shape::L,
        Shape::Vertical,
        Shape::Square
    ].iter().cycle();
    let mut directions = input.chars().cycle();

    let mut pile = HashSet::from([(1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0)]);
    let mut falling_rock = new_rock(shape_list.next().unwrap().to_owned(), 4);
    
    let mut top_coord = 0;
    let mut num_rocks = 0;
    let mut num_loops = 0;

    let mut prev_top = 0;
    let mut prev_rocks = 0;
    let mut stop_rocks = u64::MAX;
    let mut top_delta = 0;
    let mut rock_delta = 0;
    let repeat_loops = input.len() * 5;

    loop {
        let direction = directions.next().unwrap();
        move_horizontal(&mut falling_rock, &pile, direction);
        match move_down(&mut falling_rock, &pile) {
            true => (),
            false => {
                pile.extend(falling_rock);
                top_coord = pile.iter().map(|c| c.1).max().unwrap();
                falling_rock = new_rock(shape_list.next().unwrap().to_owned(), top_coord + 4);
                num_rocks += 1;
                if num_rocks == 2022 {
                    println!("Column height at 2022 rocks: {top_coord}");
                }
                if num_rocks == stop_rocks {
                    let num_repetitions = (target_rocks - num_rocks) / rock_delta;
                    top_coord += num_repetitions * top_delta;
                    println!("Column height at {target_rocks} rocks: {top_coord}");
                    break;
                }
            }
        }

        num_loops += 1;
        if num_loops % repeat_loops == 0 {
            if prev_top != 0 {
                top_delta = top_coord - prev_top;
                rock_delta = num_rocks - prev_rocks;
                stop_rocks = target_rocks - (target_rocks / rock_delta) * rock_delta;
                while stop_rocks < (num_rocks) {
                    stop_rocks += rock_delta;
                }
            }
            prev_top = top_coord;
            prev_rocks = num_rocks;
        }

    }

}
