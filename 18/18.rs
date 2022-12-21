use std::{fs, collections::HashSet};

type Coord = (i32, i32, i32);

const NEIGHBOURS: [Coord; 6] = [
    (-1,  0,  0),
    ( 1,  0,  0),
    ( 0, -1,  0),
    ( 0,  1,  0),
    ( 0,  0, -1),
    ( 0,  0,  1)
];

fn count_area(cube_coords: &HashSet<Coord>) -> u32 {
    let mut area = 0;
    for coord in cube_coords.iter() {
        for (dx, dy, dz) in NEIGHBOURS {
            let neighbour_coord = (coord.0 + dx, coord.1 + dy, coord.2 + dz);
            if !cube_coords.contains(&neighbour_coord) {
                area += 1;
            }
        }
    }
    area
}

fn dfs(start: Coord, cube_coords: &HashSet<Coord>, limits: ((i32, i32), (i32, i32), (i32, i32)))
    -> Option<HashSet<Coord>> {
    
    let mut queue = Vec::new();
    let mut visited = HashSet::new();
    queue.push(start);
    visited.insert(start);

    while let Some(coord) = queue.pop() {
        if  coord.0 < limits.0.0 ||
            coord.0 > limits.0.1 ||
            coord.1 < limits.1.0 ||
            coord.1 > limits.1.1 ||
            coord.2 < limits.2.0 ||
            coord.2 > limits.2.1 {
            return None;
        }
        for (dx, dy, dz) in NEIGHBOURS {
            let neighbour_coord = (coord.0 + dx, coord.1 + dy, coord.2 + dz);
            if cube_coords.contains(&neighbour_coord) {continue;}
            if visited.contains(&neighbour_coord) {continue;}
            visited.insert(neighbour_coord);
            queue.push(neighbour_coord);
        }
    }
    
    Some(visited)

}

fn main() {

    let input = fs::read_to_string("input.txt").unwrap();

    let mut cube_coords = HashSet::new();
    for line in input.lines() {
        let coord: Vec<i32> = line.trim().split(",").map(|s| s.parse().unwrap()).collect();
        let coord = (coord[0], coord[1], coord[2]);
        cube_coords.insert(coord);
    }

    let xmin = cube_coords.iter().map(|c| c.0).min().unwrap();
    let xmax = cube_coords.iter().map(|c| c.0).max().unwrap();
    let ymin = cube_coords.iter().map(|c| c.1).min().unwrap();
    let ymax = cube_coords.iter().map(|c| c.1).max().unwrap();
    let zmin = cube_coords.iter().map(|c| c.2).min().unwrap();
    let zmax = cube_coords.iter().map(|c| c.2).max().unwrap();
    let limits = ((xmin, xmax), (ymin, ymax), (zmin, zmax));
    let mut air_pocket_coords: Vec<Coord> = Vec::new();
    for x in xmin .. (xmax + 1) {
        for y in ymin .. (ymax + 1) {
            for z in zmin .. (zmax + 1) {
                let coord = (x, y, z);
                if cube_coords.contains(&coord) {continue;}
                if let Some(coords) = dfs(coord, &cube_coords, limits) {
                    air_pocket_coords.extend(coords);
                }
            }
        }
    }

    println!("Area: {}", count_area(&cube_coords));

    cube_coords.extend(air_pocket_coords);
    println!("Area without air pockets: {}", count_area(&cube_coords));

}
