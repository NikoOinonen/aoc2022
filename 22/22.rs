
use std::{ops::{Index, IndexMut}, fs, collections::HashMap, vec, iter::zip};

#[derive(Clone, Debug, PartialEq)]
enum Tile {
    Open,
    Wall
}

#[derive(Debug, Clone)]
struct Face {
    grid: Vec<Vec<Tile>>,
    rotation: Direction
}

impl Index<usize> for Face {
    type Output = Vec<Tile>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.grid[index]
    }
}

impl IndexMut<usize> for Face {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.grid[index]
    }
}

#[derive(Debug)]
struct Board {
    faces: Vec<Face>,
    coords: Vec<(usize, usize)>,
    edges: Vec<HashMap<Direction, (usize, Direction)>>,
    height: usize,
    width: usize,
    side_length: usize
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}
use Direction::*;

impl Direction {
    fn delta(self) -> (i32, i32) {
        match self {
            Up    => (-1,  0),
            Down  => ( 1,  0),
            Left  => ( 0, -1),
            Right => ( 0,  1)
        }
    }
    fn value(self) -> usize {
        match self {
            Right => 0,
            Down  => 1,
            Left  => 2,
            Up    => 3
        }
    }
}

impl From<usize> for Direction {
    fn from(item: usize) -> Self {
        match item {
            0 => Up,
            1 => Right,
            2 => Down,
            3 => Left,
            _ => panic!()
        }
    }
}

impl From<i32> for Direction {
    fn from(item: i32) -> Self {
        (item.rem_euclid(4) as usize).into()
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Position {
    face_ind: usize,
    coord: (usize, usize)
}

fn print_board(board: &Board, path: &HashMap<Position, Direction>) {

    for row in 0..board.height {
        for col in 0..board.width {
            let face_coord = (row / board.side_length, col / board.side_length);
            match board.coords.iter().position(|coord| *coord == face_coord){
                None => print!(" "),
                Some(face_ind) => {
                    let face = &board.faces[face_ind];
                    let face_row = row % board.side_length;
                    let face_col = col % board.side_length;
                    let tile = &face[face_row][face_col];
                    match tile {
                        Tile::Wall => print!("#"),
                        Tile::Open => {
                            match path.get(&Position {face_ind, coord: (face_row, face_col)}) {
                                Some(dir) => {
                                    match dir {
                                        Right => print!(">"),
                                        Down  => print!("v"),
                                        Left  => print!("<"),
                                        Up    => print!("∧")
                                    }
                                },
                                None => print!(".")
                            }
                        }
                    }
                },
            };
            
        }
        print!("\n");
    }
    print!("")
}

fn get_new_pos(board: &Board, pos: &Position, dir: Direction) -> (Position, Direction) {
    let cur_face_rotation = board.faces[pos.face_ind].rotation;
    let eff_dir = ((cur_face_rotation as usize + dir as usize) % 4).into();
    let (new_face_ind, move_rotation) = *board.edges[pos.face_ind].get(&eff_dir).unwrap();
    let new_face_rotation = board.faces[new_face_ind].rotation;
    let total_rotation = ((cur_face_rotation as i32 - new_face_rotation as i32 + move_rotation as i32) % 4).into();
    let new_dir = ((total_rotation as usize + dir as usize) % 4).into();
    let delta = dir.delta();
    let coord = (pos.coord.0 as i32 + delta.0, pos.coord.1 as i32 + delta.1);
    let new_coord = match total_rotation {
        Up    => ( coord.0,    coord.1  ),
        Right => ( coord.1,   -coord.0-1),
        Down  => (-coord.0-1, -coord.1-1),
        Left  => (-coord.1-1,  coord.0  )
    };
    let sl = board.side_length as i32;
    let new_coord = (new_coord.0.rem_euclid(sl) as usize, (new_coord.1.rem_euclid(sl)) as usize);
    (Position { face_ind: new_face_ind, coord: new_coord }, new_dir)
}

fn move_on_board(board: &Board, path: &mut HashMap<Position, Direction>, mut dir: Direction,
    mut pos: Position, amount: i32) -> (Position, Direction) {
    for _ in 0..amount {
        let delta = dir.delta();
        let (next_pos, next_dir) = match (pos.coord.0 as i32 + delta.0, pos.coord.1 as i32 + delta.1) {
            (row,   _) if row < 0                         => get_new_pos(board, &pos, Up   ),
            (row,   _) if row == board.side_length as i32 => get_new_pos(board, &pos, Down ),
            (  _, col) if col < 0                         => get_new_pos(board, &pos, Left ),
            (  _, col) if col == board.side_length as i32 => get_new_pos(board, &pos, Right),
            (row, col)                                    => (Position {face_ind: pos.face_ind, coord: (row as usize, col as usize)}, dir)
        };
        match board.faces[next_pos.face_ind][next_pos.coord.0][next_pos.coord.1] {
            Tile::Wall => break,
            Tile::Open => {
                pos = next_pos;
                dir = next_dir;
                path.insert(pos.clone(), dir);
            }
        }
    }
    (pos, dir)
}

fn main() {

    let input = fs::read_to_string("input.txt").unwrap();
    let coords: Vec<(usize, usize)> = vec![(0, 1), (3, 0), (0, 2), (1, 1), (2, 0), (2, 1)];

    let mut input_iter = input.split("\n\n");
    let board_str = input_iter.next().unwrap();
    let path_str = input_iter.next().unwrap().trim();

    // Make board
    let h = board_str.lines().count();
    let w = board_str.lines().map(|line| line.len()).max().unwrap();
    let side_length = ((board_str.split_whitespace().collect::<String>().len() / 6) as f64).sqrt() as usize;
    let mut face_grids = vec![vec![vec![Tile::Open; w]; h]; 6];

    for (row, line) in board_str.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                ' ' => continue,
                _   => {
                    let face_coord = (row / side_length, col / side_length);
                    let face_ind = match coords.iter().position(|coord| *coord == face_coord){
                        Some(f) => f,
                        None => panic!("Umatched coordinate {face_coord:?}")
                    };
                    let face = &mut face_grids[face_ind];
                    let face_row = row % side_length;
                    let face_col = col % side_length;
                    match c {
                        '#' => face[face_row][face_col] = Tile::Wall,
                        '.' => face[face_row][face_col] = Tile::Open,
                        _   => panic!("Invalid character {c}")
                    }

                }
            }
        }
    }

    // Gather instructions
    let mut num_chars = String::new();
    let mut instructions: Vec<(Direction, i32)> = Vec::new();
    for c in path_str.chars() {
        match c {
            _ if c.is_digit(10) => num_chars.push(c),
            _ =>  {
                let amount: i32 = num_chars.parse().unwrap();
                num_chars.clear();
                let dir = match c {
                    'L' => Left,
                    'R' => Right,
                    _ => panic!()
                };
                instructions.push((dir, amount));
            }
        }
    }
    if num_chars.len() > 0 {
        let amount: i32 = num_chars.parse().unwrap();
        instructions.push((Up, amount));
    }

    { // Part 1


        let max_row: i32 = coords.iter().max_by(|x, y| x.0.cmp(&y.0)).unwrap().0 as i32 + 1;
        let max_col: i32 = coords.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap().1 as i32 + 1;
        let mut edges = Vec::new();
        for coord in coords.iter() {
            let mut face_edges = HashMap::new();
            for dir in [Up, Down, Left, Right] {
                let delta = dir.delta();
                let mut pos = coord.clone();
                loop {
                    pos.0 = (pos.0 as i32 + delta.0).rem_euclid(max_row) as usize;
                    pos.1 = (pos.1 as i32 + delta.1).rem_euclid(max_col) as usize;
                    match coords.iter().position(|coord| *coord == pos) {
                        Some(coord_ind) => {
                            face_edges.insert(dir, (coord_ind, Up));
                            break;
                        },
                        None => ()
                    }
                }
            }
            edges.push(face_edges);
        }

        let faces = face_grids.clone().into_iter().map(|grid| Face {grid, rotation: Up}).collect();
        let board = Board {
            faces,
            coords: coords.clone(),
            edges,
            height: h,
            width: w,
            side_length
        };

        let mut dir = Right;
        let mut pos = Position {face_ind: 0, coord: (0, board.faces[0].grid[0].iter().position(|tile| *tile == Tile::Open).unwrap())};
        let mut path: HashMap<Position, Direction> = HashMap::new();
        path.insert(pos.clone(), Right);
        for (turn, amount) in instructions.clone() {
            (pos, dir) = move_on_board(&board, &mut path, dir, pos, amount);
            path.insert(pos.clone(), dir);
            dir = match turn {
                Left  => ((dir as i32) - 1).rem_euclid(4).into(), 
                Right => ((dir as i32) + 1).rem_euclid(4).into(),
                _     => break 
            };
        }

        print_board(&board, &path);
        println!("{pos:?}, {dir:?} {}", (dir as usize));

        let face_coord = board.coords[pos.face_ind];
        let final_row = face_coord.0 * board.side_length + pos.coord.0;
        let final_col = face_coord.1 * board.side_length + pos.coord.1;
        println!("Final password (Part 1): {}", 1000 * (final_row + 1) + 4 * (final_col + 1) + dir.value())

    }

    { // Part 2

        // Faces hard-coded in order: center, up, right, down, left, under
        let rotations = vec![Up, Left, Up, Up, Down, Up];
        let faces = zip(face_grids, rotations).into_iter().map(|(grid, rotation)| Face {grid, rotation}).collect();
        let edges = vec![
            vec![(Up, (1, Up   )), (Right, (2, Up   )), (Down, (3, Up   )), (Left, (4, Up   ))].into_iter().collect(),
            vec![(Up, (5, Up   )), (Right, (2, Right)), (Down, (0, Up   )), (Left, (4, Left ))].into_iter().collect(),
            vec![(Up, (1, Left )), (Right, (5, Down )), (Down, (3, Right)), (Left, (0, Up   ))].into_iter().collect(),
            vec![(Up, (0, Up   )), (Right, (2, Left )), (Down, (5, Up   )), (Left, (4, Right))].into_iter().collect(),
            vec![(Up, (1, Right)), (Right, (0, Up   )), (Down, (3, Left )), (Left, (5, Down ))].into_iter().collect(),
            vec![(Up, (3, Up   )), (Right, (2, Down )), (Down, (1, Up   )), (Left, (4, Down ))].into_iter().collect()
        ].into_iter().collect();
        let board = Board {
            faces,
            coords,
            edges,
            height: h,
            width: w,
            side_length
        };

        let mut dir = Right;
        let mut pos = Position {face_ind: 0, coord: (0, board.faces[0].grid[0].iter().position(|tile| *tile == Tile::Open).unwrap())};
        let mut path: HashMap<Position, Direction> = HashMap::new();
        path.insert(pos.clone(), Right);
        for (turn, amount) in instructions {
            (pos, dir) = move_on_board(&board, &mut path, dir, pos, amount);
            path.insert(pos.clone(), dir);
            dir = match turn {
                Left  => ((dir as i32) - 1).rem_euclid(4).into(), 
                Right => ((dir as i32) + 1).rem_euclid(4).into(),
                _     => break 
            };
        }

        print_board(&board, &path);
        println!("{pos:?}, {dir:?} {}", (dir as usize));

        let face_coord = board.coords[pos.face_ind];
        let final_row = face_coord.0 * board.side_length + pos.coord.0;
        let final_col = face_coord.1 * board.side_length + pos.coord.1;
        println!("Final password (Part 2): {}", 1000 * (final_row + 1) + 4 * (final_col + 1) + dir.value());

    }

}
