
use std::{ops::{Index, IndexMut}, fmt::Display, fs, collections::{HashSet, HashMap}};

#[derive(Clone, Debug, PartialEq)]
enum Tile {
    Empty,
    Open,
    Wall
}

#[derive(Debug)]
struct Board {
    grid: Vec<Vec<Tile>>,
    path: HashMap<(usize, usize), Direction>
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up
}

impl Board {
    fn new(h: usize, w: usize) -> Board {
        Board {grid: vec![vec![Tile::Empty; w]; h], path: HashMap::new() }
    }
}

impl Index<usize> for Board {
    type Output = Vec<Tile>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.grid[index]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.grid[index]
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, row) in self.grid.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                match col {
                    Tile::Empty => write!(f, " ")?,
                    Tile::Wall  => write!(f, "#")?,
                    Tile::Open  => {
                        match self.path.get(&(i, j)) {
                            Some(dir) => {
                                match dir {
                                    Direction::Right => write!(f, ">")?,
                                    Direction::Down  => write!(f, "v")?,
                                    Direction::Left  => write!(f, "<")?,
                                    Direction::Up    => write!(f, "∧")?
                                }
                            },
                            None => write!(f, ".")?
                        }
                    }
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}

fn move_on_board(board: &mut Board, dir: Direction, start: (usize, usize), amount: i32) -> (usize, usize) {
    let h = board.grid.len() as i32;
    let w = board[0].len() as i32;
    let mut pos = (start.0 as i32, start.1 as i32);
    for _ in 0..amount {
        let mut next_pos = match dir {
            Direction::Right => ((pos.0    ) % h, (pos.1 + 1) % w),
            Direction::Down  => ((pos.0 + 1) % h, (pos.1    ) % w),
            Direction::Left  => ((pos.0    ) % h, (pos.1 - 1) % w),
            Direction::Up    => ((pos.0 - 1) % h, (pos.1    ) % w)
        };
        if next_pos.0 < 0 {next_pos.0 += h;}
        if next_pos.1 < 0 {next_pos.1 += w;}
        match board[next_pos.0 as usize][next_pos.1 as usize] {
            Tile::Wall => break,
            Tile::Open => {
                pos = next_pos;
                board.path.insert((pos.0 as usize, pos.1 as usize), dir);
            },
            Tile::Empty => {
                let next_pos = move_on_board(board, dir, (next_pos.0 as usize, next_pos.1 as usize), 1);
                if board[next_pos.0][next_pos.1] == Tile::Empty {
                    break;
                }
                board.path.insert(next_pos, dir);
                pos = (next_pos.0 as i32, next_pos.1 as i32);
            }
        }
    }
    (pos.0 as usize, pos.1 as usize)
}

fn main() {

//     let input = "        ...#
//         .#..
//         #...
//         ....
// ...#.......#
// ........#...
// ..#....#....
// ..........#.
//         ...#....
//         .....#..
//         .#......
//         ......#.

// 10R5L5R10L4R5L5".to_owned();

    let input = fs::read_to_string("input.txt").unwrap();

    let mut input_iter = input.split("\n\n");
    let board_str = input_iter.next().unwrap();
    let path_str = input_iter.next().unwrap().trim();
    let h = board_str.lines().count();
    let w = board_str.lines().map(|line| line.len()).max().unwrap();

    let mut board = Board::new(h, w);
    for (row, line) in board_str.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            match c {
                '#' => board[row][col] = Tile::Wall,
                '.' => board[row][col] = Tile::Open,
                _   => ()
            }
        }
    }

    // println!("{board}");
    // println!("{path_str}");
    
    let mut pos = (0, board[0].iter().position(|tile| *tile == Tile::Open).unwrap());
    let mut dir = Direction::Right;
    let directions = [Direction::Right, Direction::Down, Direction::Left, Direction::Up];
    board.path.insert(pos, dir);

    let mut num_chars = String::new();
    for c in path_str.chars() {
        match c {
            _ if c.is_digit(10) => num_chars.push(c),
            _ =>  {
                let amount: i32 = num_chars.parse().unwrap();
                num_chars.clear();
                pos = move_on_board(&mut board, dir, pos.clone(), amount);
                dir = match c {
                    'L' => directions[((dir as usize) - 1) % 4],
                    'R' => directions[((dir as usize) + 1) % 4],
                    _ => unreachable!()
                };
                board.path.insert(pos, dir);
            }
        }
    }
    if num_chars.len() > 0 {
        let amount: i32 = num_chars.parse().unwrap();
        pos = move_on_board(&mut board, dir, pos.clone(), amount);
    }

    println!("{board}");
    println!("{pos:?}, {dir:?} {}", (dir as usize));

    println!("Final password: {}", 1000 * (pos.0 + 1) + 4 * (pos.1 + 1) + (dir as usize))

}
