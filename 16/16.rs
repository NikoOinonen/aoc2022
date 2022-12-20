
use std::{fs, collections::{HashMap, HashSet, VecDeque}};

#[derive(Debug, Eq, PartialEq, Hash)]
struct Valve<'a> {
    name: &'a str,
    flow_rate: u32,
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct State<'a>  {
    pressure: u32,
    time_left: i32,
    prev: Vec<&'a str>
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct StateDouble<'a>  {
    pressure: u32,
    time_left1: i32,
    time_left2: i32,
    locations: (&'a str, &'a str),
    prev: Vec<&'a str>
}


fn bfs_shortest_path(source: &Valve, dest: &Valve, neighbours: &HashMap<&str, Vec<&Valve>>) -> u32 {

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((source, 0));
    visited.insert(source);

    while let Some((valve, dist)) = queue.pop_front() {
        if valve == dest {
            return dist;
        }
        for neighbour in neighbours[valve.name].iter() {
            if visited.contains(neighbour) {continue;}
            visited.insert(neighbour);
            queue.push_back((neighbour, dist + 1));
        }
    }

    u32::MAX

}

fn dfs_max_pressure(source: &Valve, shortest_paths: HashMap<&str, Vec<(&Valve, u32)>>) -> u32 {

    let mut queue = Vec::new();
    let start = State {pressure: 0, time_left: 30, prev: vec![source.name]};
    queue.push(start);

    let mut max_pressure = 0;
    while let Some(State {pressure, time_left, prev}) = queue.pop() {
        let valve_name = *prev.last().unwrap();
        println!("{pressure} {valve_name}, {time_left}, {prev:?}");
        for (neighbour, dist) in shortest_paths[valve_name].iter() {
            if prev.contains(&neighbour.name) {continue;}
            let new_time_left = time_left - (*dist as i32);
            if new_time_left <= 0 {continue;}
            let new_pressure = pressure + neighbour.flow_rate * new_time_left as u32;
            let mut new_prev = prev.clone();
            new_prev.push(neighbour.name);
            let new_state = State {pressure: new_pressure, time_left: new_time_left, prev: new_prev};
            queue.push(new_state);
        }
        max_pressure = max_pressure.max(pressure);
    }

    max_pressure

}

fn dfs_max_pressure_double(source: &Valve, shortest_paths: HashMap<&str, Vec<(&Valve, u32)>>) -> u32 {

    let mut queue = Vec::new();
    let start = StateDouble {pressure: 0, time_left1: 26, time_left2: 26,
        locations: ("AA", "AA"), prev: vec![source.name]};
    queue.push(start);

    let mut max_pressure = 0;
    while let Some(StateDouble {pressure, time_left1, time_left2, locations, prev}) = queue.pop() {

        let valve_name1 = locations.0;
        let valve_name2 = locations.1;

        if time_left1 < time_left2 {
            for (neighbour, dist) in shortest_paths[&valve_name2].iter() {
                if prev.contains(&neighbour.name) {continue;}
                let new_time_left = time_left2 - (*dist as i32);
                if new_time_left <= 0 {continue;}
                let new_pressure = pressure + neighbour.flow_rate * new_time_left as u32;
                let mut new_prev = prev.clone();
                new_prev.push(neighbour.name);
                let new_state = StateDouble {pressure: new_pressure, time_left1: time_left1,
                    time_left2: new_time_left, locations: (valve_name1, neighbour.name), prev: new_prev};
                queue.push(new_state);
            }
        } else if time_left1 > time_left2 {
            for (neighbour, dist) in shortest_paths[&valve_name1].iter() {
                if prev.contains(&neighbour.name) {continue;}
                let new_time_left = time_left1 - (*dist as i32);
                if new_time_left <= 0 {continue;}
                let new_pressure = pressure + neighbour.flow_rate * new_time_left as u32;
                let mut new_prev = prev.clone();
                new_prev.push(neighbour.name);
                let new_state = StateDouble {pressure: new_pressure, time_left1: new_time_left,
                    time_left2: time_left2, locations: (neighbour.name, valve_name2), prev: new_prev};
                queue.push(new_state);
            }
        } else {
            for (neighbour1, dist1) in shortest_paths[&valve_name1].iter() {
                if prev.contains(&neighbour1.name) {continue;}
                let new_time_left1 = time_left1 - (*dist1 as i32);
                if new_time_left1 <= 0 {continue;}
                let new_pressure1 = pressure + neighbour1.flow_rate * new_time_left1 as u32;
                for (neighbour2, dist2) in shortest_paths[&valve_name2].iter() {
                    if neighbour1.name == neighbour2.name {continue;}
                    if prev.contains(&neighbour2.name) {continue;}
                    let new_time_left2 = time_left2 - (*dist2 as i32);
                    if new_time_left2 <= 0 {continue;}
                    let new_pressure2 = new_pressure1 + neighbour2.flow_rate * new_time_left2 as u32;
                    let mut new_prev = prev.clone();
                    new_prev.push(neighbour2.name);
                    new_prev.push(neighbour1.name);
                    let new_state = StateDouble {pressure: new_pressure2, time_left1: new_time_left1,
                        time_left2: new_time_left2, locations: (neighbour1.name, neighbour2.name), prev: new_prev};
                    queue.push(new_state);
                }
            }
        }

        max_pressure = max_pressure.max(pressure);

    }

    max_pressure

}

fn main() {

    let input = fs::read_to_string("input.txt").unwrap();

    let mut valves = Vec::new();
    let mut neighbours_str = HashMap::new();
    for line in input.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();
        let name = line[1];
        let flow_rate: u32 = line[4].trim_end_matches(";").split("=").last().unwrap().parse().unwrap();
        let nb: Vec<&str> = line[9..].iter().map(|s| s.trim_end_matches(",")).collect();
        valves.push(Valve {name, flow_rate});
        neighbours_str.insert(name, nb);
    }

    let neighbours: HashMap<&str, Vec<&Valve>> = neighbours_str.drain()
        .map(|(name, nb)| {
            (name, nb.iter().map(|n| valves.iter().find(|v| v.name == *n).unwrap()).collect())
            })
        .collect();

    let mut shortest_paths: HashMap<&str, Vec<(&Valve, u32)>> = HashMap::new();
    for valve_source in valves.iter() {
        if valve_source.name != "AA" && valve_source.flow_rate == 0 {continue;}
        let mut paths = Vec::new();
        for valve_dest in valves.iter() {
            if valve_dest.flow_rate == 0 || valve_dest == valve_source {continue;}
            let shortest_dist = bfs_shortest_path(valve_source, valve_dest, &neighbours);
            paths.push((valve_dest, shortest_dist + 1));
        }
        shortest_paths.insert(valve_source.name, paths);
    }

    let start = valves.iter().find(|v| v.name == "AA").unwrap();
    println!("Maximum pressure release: {}", dfs_max_pressure(start, shortest_paths));
    println!("Maximum pressure release with elephant: {}", dfs_max_pressure_double(start, shortest_paths));

}
