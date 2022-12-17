
use std::{vec, collections::HashMap, fs};

#[derive(Debug, Clone, PartialEq)]
enum Material {
    Rock,
    Sand
}

fn print_grid(grid: HashMap<(i32, i32), Material>) {
    let mut x_min = i32::MAX;
    let mut x_max = i32::MIN;
    let mut y_min = i32::MAX;
    let mut y_max = i32::MIN;
    for (coord, _) in grid.iter() {
        if coord.0 < x_min {x_min = coord.0;}
        if coord.0 > x_max {x_max = coord.0;}
        if coord.1 < y_min {y_min = coord.1;}
        if coord.1 > y_max {y_max = coord.1;}
    }
    for j in (y_min - 1) .. (y_max + 2) {
        for i in (x_min - 1) .. (x_max + 2) {
            match grid.get(&(i, j)) {
                Some(material) => {
                    match material {
                        Material::Rock => print!("#"),
                        Material::Sand => print!("O")
                    }
                },
                None => print!(".")
            }
        }
        print!("\n");
    }
}

fn drop_sand(grid: &mut HashMap<(i32, i32), Material>, has_floor: bool) {
    let mut y_max = i32::MIN;
    for (coord, _) in grid.iter() {
        if coord.1 > y_max {y_max = coord.1;}
    }
    let y_floor = y_max + 2;
    'outer: loop {
        let mut sand_pos = (500, -1);
        loop {
            let next_pos = (sand_pos.0, sand_pos.1 + 1);
            if has_floor && next_pos.1 == y_floor {
                break;
            }
            if !has_floor && next_pos.1 > y_max {
                break 'outer;
            }
            match grid.get(&next_pos) {
                None => sand_pos = next_pos,
                Some(_) => {
                    let left = (next_pos.0 - 1, next_pos.1);
                    match grid.get(&left) {
                        None => sand_pos = left,
                        Some(_) => {
                            let right = (next_pos.0 + 1, next_pos.1);
                            match grid.get(&right) {
                                None => sand_pos = right,
                                Some(_) => break
                            }
                        }
                    }
                }
            }
        }
        grid.insert(sand_pos, Material::Sand);
        if sand_pos == (500, 0) {
            break;
        }
    }
}

fn count_sand(grid: HashMap<(i32, i32), Material>) -> i32 {
    let mut num_sand = 0;
    for (_, material) in grid {
        if material == Material::Sand {
            num_sand += 1;
        }
    }
    num_sand
}

fn main() {

    let input = fs::read_to_string("input.txt").unwrap();

    let mut grid: HashMap<(i32, i32), Material> = HashMap::new();
    for line in input.lines() {
        let mut prev_coord = vec![-1, -1];
        for coord_str in line.split(" -> ") {
            let coord: Vec<i32> = coord_str.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
            if prev_coord != vec![-1, -1] {
                if prev_coord[0] == coord[0] {
                    let i = prev_coord[0];
                    let j_start = prev_coord[1].min(coord[1]);
                    let j_end = prev_coord[1].max(coord[1]) + 1;
                    for j in j_start..j_end {
                        grid.insert((i, j), Material::Rock);
                    }
                } else {
                    let j = prev_coord[1];
                    let i_start = prev_coord[0].min(coord[0]);
                    let i_end = prev_coord[0].max(coord[0]) + 1;
                    for i in i_start..i_end {
                        grid.insert((i, j), Material::Rock);
                    }
                }
            }
            prev_coord = coord.clone();
        }
    }

    let mut grid2 = grid.clone();

    print_grid(grid.clone());
    drop_sand(&mut grid, false);
    print_grid(grid.clone());

    drop_sand(&mut grid2, true);
    print_grid(grid2.clone());

    println!("Total number of sand tiles without floor: {}", count_sand(grid));
    println!("Total number of sand tiles with floor: {}", count_sand(grid2));

}