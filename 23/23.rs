
use std::fs;

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East
}

fn print_positions(positions: &Vec<(i32, i32)>) {
    let row_min = *positions.iter().map(|(r, _)| r).min().unwrap();
    let row_max = *positions.iter().map(|(r, _)| r).max().unwrap();
    let col_min = *positions.iter().map(|(_, c)| c).min().unwrap();
    let col_max = *positions.iter().map(|(_, c)| c).max().unwrap();
    for row in row_min .. (row_max + 1) {
        for col in col_min .. (col_max + 1) {
            if positions.contains(&(row, col)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n")
    }
}

fn count_spaces(positions: &Vec<(i32, i32)>) -> u32 {
    let row_min = *positions.iter().map(|(r, _)| r).min().unwrap();
    let row_max = *positions.iter().map(|(r, _)| r).max().unwrap();
    let col_min = *positions.iter().map(|(_, c)| c).min().unwrap();
    let col_max = *positions.iter().map(|(_, c)| c).max().unwrap();
    let mut num_spaces = 0;
    for row in row_min .. (row_max + 1) {
        for col in col_min .. (col_max + 1) {
            if !positions.contains(&(row, col)) {
                num_spaces += 1;
            }
        }
    }
    num_spaces
}

fn make_proposals(positions: &Vec<(i32, i32)>, directions: &Vec<Direction>) -> Vec<(i32, i32)> {
    let mut proposals: Vec<(i32, i32)> = Vec::new();
    for (row, col) in positions.iter() {
        let n  = (row - 1, col + 0);
        let s  = (row + 1, col + 0);
        let w  = (row + 0, col - 1);
        let e  = (row + 0, col + 1);
        let nw = (row - 1, col - 1);
        let ne = (row - 1, col + 1);
        let sw = (row + 1, col - 1);
        let se = (row + 1, col + 1);
        if !(
            positions.contains(&n)  ||
            positions.contains(&s)  ||
            positions.contains(&w)  ||
            positions.contains(&e)  ||
            positions.contains(&nw) ||
            positions.contains(&ne) ||
            positions.contains(&sw) ||
            positions.contains(&se)
        ) {
            proposals.push((*row, *col));
            continue;
        }
        let mut found_proposal = false;
        for dir in directions.iter() {
            match dir {
                Direction::North => {
                    if !(
                        positions.contains(&n)  ||
                        positions.contains(&ne) ||
                        positions.contains(&nw) 
                    ) {
                        proposals.push(n);
                        found_proposal = true;
                        break;
                    }
                },
                Direction::South => {
                    if !(
                        positions.contains(&s)  ||
                        positions.contains(&se) ||
                        positions.contains(&sw) 
                    ) {
                        proposals.push(s);
                        found_proposal = true;
                        break;
                    }
                },
                Direction::West => {
                    if !(
                        positions.contains(&w)  ||
                        positions.contains(&nw) ||
                        positions.contains(&sw) 
                    ) {
                        proposals.push(w);
                        found_proposal = true;
                        break;
                    }
                },
                Direction::East => {
                    if !(
                        positions.contains(&e)  ||
                        positions.contains(&ne) ||
                        positions.contains(&se) 
                    ) {
                        proposals.push(e);
                        found_proposal = true;
                        break;
                    }
                },
            }
        }
        if !found_proposal {
            proposals.push((*row, *col));
        }
    }
    proposals
}

fn make_move(positions: &mut Vec<(i32, i32)>, directions: &Vec<Direction>) {
    let proposals = make_proposals(&positions, &directions);
    for (i, proposal) in proposals.iter().enumerate() {
        let num_occurences = proposals.iter().filter(|&p| *p == *proposal).count();
        if num_occurences == 1 {
            positions[i] = *proposal;
        }
    }
}

fn main() {

    let input = fs::read_to_string("input.txt").unwrap();

    let directions = [Direction::North, Direction::South, Direction::West, Direction::East].iter().cycle();
    let mut positions: Vec<(i32, i32)> = Vec::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                '#' => positions.push((row as i32, col as i32)),
                '.' => (),
                _ => panic!("Unhandled character")
            }
        }
    }

    {
        let mut positions = positions.clone();
        for round in 0..10 {
            let directions: Vec<Direction> = directions.clone().skip(round).take(4).map(|v| *v).collect();
            make_move(&mut positions, &directions);
        }

        println!("Number of empty spaces: {}", count_spaces(&positions));

    }

    {
        let mut round = 0;
        loop {
            let init_positions = positions.clone();
            let directions: Vec<Direction> = directions.clone().skip(round).take(4).map(|v| *v).collect();
            make_move(&mut positions, &directions);
            round += 1;
            if positions == init_positions {
                break;
            }
        }

        println!("Number of rounds until stop: {round}");

    }


}
