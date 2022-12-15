
// use std::str;

use std::{cmp::Ordering, collections::{BinaryHeap, HashMap}, fs};

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
struct Position {
    coord: (i32, i32),
    dist: u32
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn is_neighbour_up(current_coord: (i32, i32), new_coord: (i32, i32), heights: &Vec<Vec<i32>>) -> bool {
    if new_coord.0 < 0 {return false}
    if new_coord.0 >= heights.len() as i32 {return false}
    if new_coord.1 < 0 {return false}
    if new_coord.1 >= heights[0].len() as i32 {return false}
    if heights[current_coord.0 as usize][current_coord.1 as usize] < 
        (heights[new_coord.0 as usize][new_coord.1 as usize] - 1){return false}
    true
}

fn is_neighbour_down(current_coord: (i32, i32), new_coord: (i32, i32), heights: &Vec<Vec<i32>>) -> bool {
    if new_coord.0 < 0 {return false}
    if new_coord.0 >= heights.len() as i32 {return false}
    if new_coord.1 < 0 {return false}
    if new_coord.1 >= heights[0].len() as i32 {return false}
    if heights[current_coord.0 as usize][current_coord.1 as usize] > 
        (heights[new_coord.0 as usize][new_coord.1 as usize] + 1) {return false}
    true
}

fn find_shortest_path(start: (i32, i32), is_end: Box<dyn Fn((i32, i32),&Vec<Vec<i32>>) -> bool>,
    is_neighbour: Box<dyn Fn((i32, i32), (i32, i32), &Vec<Vec<i32>>) -> bool>, heights: &Vec<Vec<i32>>) -> u32 {

    let mut heap = BinaryHeap::new();
    let mut min_dist = HashMap::new();
    heap.push(Position {coord: start, dist: 0});
    min_dist.insert(start, 0);

    let n_row = heights.len() as i32;
    let n_col = heights[0].len() as i32;
    for i in 0..n_row {
        for j in 0..n_col {
            let coord = (i, j);
            if coord != start {
                min_dist.insert(coord, u32::MAX);
            }
        }
    }

    while let Some(pos) = heap.pop() {
        let c = pos.coord;
        let dist = pos.dist;
        if is_end(c, heights) {
            return *min_dist.get(&c).unwrap();
        }
        let mut neighbours = Vec::new();
        if is_neighbour(c, (c.0 - 1, c.1), &heights) {neighbours.push((c.0 - 1, c.1))}
        if is_neighbour(c, (c.0 + 1, c.1), &heights) {neighbours.push((c.0 + 1, c.1))}
        if is_neighbour(c, (c.0, c.1 - 1), &heights) {neighbours.push((c.0, c.1 - 1))}
        if is_neighbour(c, (c.0, c.1 + 1), &heights) {neighbours.push((c.0, c.1 + 1))}
        for neighbour in neighbours {
            if (dist + 1) < *min_dist.get(&neighbour).unwrap() {
                heap.push(Position {coord: neighbour, dist: dist + 1});
                min_dist.insert(neighbour, dist + 1);
            }
        }
    }

    u32::MAX

}

fn main() {

    let input = fs::read_to_string("input.txt").unwrap();

    let mut heights: Vec<Vec<i32>> = Vec::new();
    
    let mut row = 0;
    let mut start: (i32, i32) = (0, 0);
    let mut end: (i32, i32) = (0, 0);
    for line in input.lines() {
        if line.trim().len() == 0 {continue;}
        heights.push(Vec::new());
        for (col, c) in line.trim().chars().enumerate() {
            let c = match c {
                'S' => {    
                    start = (row as i32, col as i32);
                    'a'
                },
                'E' => {
                    end = (row as i32, col as i32);
                    'z'
                }
                _ => c
            };
            heights[row].push((c as i32) - ('a' as i32));
        }
        row += 1;
    }

    println!("Minimum distance to top: {}",
        find_shortest_path(start, Box::new(move |c, _h| c == end), Box::new(is_neighbour_up), &heights));
    println!("Minimum distance from a: {}",
        find_shortest_path(end, Box::new(|c, h| h[c.0 as usize][c.1 as usize] == 0),
            Box::new(is_neighbour_down), &heights));

}
