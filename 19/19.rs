
use std::{convert::TryInto, collections::HashSet, fs};
use std::{sync::mpsc, thread};

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct State {
    time_left: i32,
    resources: [i32; 4],
    robots: [i32; 4]
}

fn get_cost_table(line: &str) -> [[i32; 3]; 4] {
    let cost_line = line.trim().split(":").last().unwrap();
    let costs = cost_line.split(".").take(4)
        .map(|s| s.split_whitespace().filter_map(|s_| s_.parse().ok()).collect())
        .collect::<Vec<Vec<i32>>>();
    [[costs[0][0], 0, 0], [costs[1][0], 0, 0], [costs[2][0], costs[2][1], 0], [costs[3][0], 0, costs[3][1]]]
}

fn search_max_geodes(costs: [[i32; 3]; 4], start_time: i32) -> i32 {

    let start = State {time_left: start_time, resources: [0, 0, 0, 0], robots: [1, 0, 0, 0]};
    let mut queue = Vec::new();
    let mut visited = HashSet::new();
    queue.push(start.clone());
    visited.insert(start);

    let mut max_geodes = 0;
    while let Some(State {time_left, resources, robots}) = queue.pop() {

        if time_left == 0 {
            max_geodes = max_geodes.max(resources[3]);
            continue;
        }

        let max_possible_geodes = resources[3] + robots[3] * time_left + (time_left - 1) * time_left / 2;
        if max_possible_geodes <= max_geodes {continue;}

        let new_resources: [i32; 4] = resources.iter().zip(robots)
            .map(|(rs, rb)| rs + rb)
            .collect::<Vec<i32>>()
            .try_into().unwrap();

        if (resources[0] >= costs[3][0]) && (resources[2] >= costs[3][2]) {
            let mut new_robots = robots.clone();
            new_robots[3] += 1;
            let mut new_resource_deducted = new_resources.clone();
            new_resource_deducted[0] -= costs[3][0];
            new_resource_deducted[2] -= costs[3][2];
            let new_state = State { time_left: time_left - 1, resources: new_resource_deducted, robots: new_robots };
            if visited.contains(&new_state) {continue;}
            queue.push(new_state.clone());
            visited.insert(new_state);
            continue;
        }

        let mut actions = Vec::new();
        for (ir, cost) in costs.iter().enumerate().take(3) {
            if resources.iter().take(3).zip(cost).all(|(r, c)| r >= c) {
                let mut new_resource_deducted = new_resources.clone();
                for (r, c) in new_resource_deducted.iter_mut().zip(cost) {
                    *r -= c;
                }
                let mut new_robots = robots.clone();
                new_robots[ir] += 1;
                let new_state = State { time_left: time_left - 1, resources: new_resource_deducted, robots: new_robots };
                if visited.contains(&new_state) {continue;}
                queue.push(new_state.clone());
                visited.insert(new_state);
                actions.push(ir);
            }
        }

        if actions.len() == costs.len() {continue;}
        if (robots[2] == 0) && (resources[1] == 0) && actions.contains(&0) && actions.contains(&1) {continue;}
        let new_state = State { time_left: time_left - 1, resources: new_resources, robots };
        if visited.contains(&new_state) {continue;}
        queue.push(new_state.clone());
        visited.insert(new_state);

    }

    max_geodes

}

fn main() {

    let input = fs::read_to_string("input.txt").unwrap();

    let (tx, rx) = mpsc::channel();
    for (i, line) in input.lines().enumerate() {
        let tx = tx.clone();
        let costs = get_cost_table(line);
        thread::spawn(move || {
            let quality = (i as i32 + 1) * search_max_geodes(costs, 24);
            tx.send(quality).unwrap();
        });
    }
    drop(tx);
    let qualities: Vec<i32> = rx.iter().collect();
    println!("Total quality (24): {}", qualities.iter().sum::<i32>());

    let (tx, rx) = mpsc::channel();
    for line in input.lines().take(3) {
        let tx = tx.clone();
        let costs = get_cost_table(line);
        thread::spawn(move || tx.send(search_max_geodes(costs, 32)).unwrap());
    }
    drop(tx);
    let qualities: Vec<i32> = rx.iter().collect();
    println!("Total geodes multiplied (32): {}", qualities.iter().product::<i32>());

}