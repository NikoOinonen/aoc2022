
#![feature(hash_drain_filter)]
use std::{fs, collections::HashSet};

fn manhattan_distance(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn main() {

    let input = fs::read_to_string("input.txt").unwrap();
    let search_row = 2000000;
    let search_min = 0;
    let search_max = 4000000;

    let mut sensor_positions: Vec<(i32, i32)> = Vec::new();
    let mut beacon_positions: Vec<(i32, i32)> = Vec::new();
    let mut no_beacon_cols = HashSet::new();
    let mut possible_beacon_positions = Vec::new();
    for line in input.lines() {

        let line = line.split_whitespace().collect::<Vec<&str>>();
        let nums: [i32; 4] = [line[2], line[3], line[8], line[9]]
            .map(|s| s.trim_end_matches(|c| c == ',' || c == ':').split('=').last().unwrap().parse().unwrap());
        let sensor_pos = (nums[0], nums[1]);
        let beacon_pos = (nums[2], nums[3]);
        sensor_positions.push(sensor_pos);
        beacon_positions.push(beacon_pos);

        // Part 1
        let dist = manhattan_distance(sensor_pos, beacon_pos);
        let col_dist = dist - (sensor_pos.1 - search_row).abs();
        if col_dist > 0 {
            let col_min = sensor_pos.0 - col_dist;
            let col_max = sensor_pos.0 + col_dist;
            no_beacon_cols.extend(col_min..(col_max + 1));
        }

        // Part 2
        let dist = dist + 1;
        for d in -dist .. (dist + 1) {
            let col = sensor_pos.0 + d;
            if col < search_min || col > search_max {continue;}
            let row_delta = dist - d.abs();
            let row_plus = sensor_pos.1 + row_delta;
            let row_minus = sensor_pos.1 - row_delta;
            if row_plus >= search_min && row_plus <= search_max {
                possible_beacon_positions.push((col, row_plus));
            }
            if row_minus >= search_min && row_minus <= search_max {
                possible_beacon_positions.push((col, row_minus));
            }
        }

    }

    for (sensor_pos, beacon_pos) in sensor_positions.iter().zip(beacon_positions.iter()) {
        if sensor_pos.1 == search_row {no_beacon_cols.remove(&sensor_pos.0);}
        if beacon_pos.1 == search_row {no_beacon_cols.remove(&beacon_pos.0);}
    }

    let mut tuning_frequency: i64 = 0;
    'outer: for pos in possible_beacon_positions {
        for (sensor_pos, beacon_pos) in sensor_positions.iter().zip(beacon_positions.iter()) {
            let dist = manhattan_distance(*sensor_pos, *beacon_pos);
            if manhattan_distance(pos, *sensor_pos) <= dist {
                continue 'outer;
            }
        }
        tuning_frequency = 4000000 * (pos.0 as i64) + (pos.1 as i64);
        break;
    }

    println!("Number of columns where beacons cannot be: {}", no_beacon_cols.len());
    println!("Tuning frequency: {}", tuning_frequency);

}
