
use std::{fs, collections::{HashMap, BinaryHeap, HashSet}};

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Stay
}

static DIRECTIONS: [Direction; 5] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
    Direction::Stay
];

struct Blizzard {
    pos: (usize, usize),
    dir: Direction
}

struct Field {
    width: usize,
    height: usize,
    blizzards: Vec<Blizzard>
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Expedition {
    pos: (usize, usize),
    t: usize
}

impl Field{
    fn make_move(&self, exp: &Expedition, dir: Direction) -> Option<Expedition> {
        let new_exp_pos = match dir {
            Direction::Up => {
                if exp.pos.0 > 1 || exp.pos == (1, 1) {
                    (exp.pos.0 - 1, exp.pos.1)
                } else {
                    return None
                }
            },
            Direction::Down => {
                if exp.pos.0 < (self.height - 2) || (exp.pos == (self.height - 2, self.width - 2)) {
                    (exp.pos.0 + 1, exp.pos.1)
                } else {
                    return None
                }
            },
            Direction::Left => {
                if exp.pos.1 > 1 && exp.pos.0 >= 1 && exp.pos.0 < (self.height - 1) {
                    (exp.pos.0, exp.pos.1 - 1)
                } else {
                    return None
                }
            },
            Direction::Right => {
                if exp.pos.1 < (self.width - 2) && exp.pos.0 >= 1 {
                    (exp.pos.0, exp.pos.1 + 1)
                } else {
                    return None
                }
            },
            Direction::Stay => exp.pos,
        };
        let t_new = exp.t + 1;
        for blizzard in self.blizzards.iter() {
            let new_pos = get_blizzard_position(blizzard, t_new, self.width, self.height);
            if new_pos == new_exp_pos {
                return None;
            }
        }
        Some(Expedition {pos: new_exp_pos, t: t_new})
    }
}

fn get_blizzard_position(blizzard: &Blizzard, t: usize, width: usize, height: usize) -> (usize, usize) {
    match blizzard.dir {
        Direction::Right => (blizzard.pos.0, (blizzard.pos.1 - 1 + t) % (width - 2) + 1),
        Direction::Down => ((blizzard.pos.0 - 1 + t) % (height - 2) + 1, blizzard.pos.1),
        Direction::Left => (
            blizzard.pos.0,
            ((blizzard.pos.1 as i32 - 1 - t as i32).rem_euclid((width - 2) as i32) + 1) as usize
        ),
        Direction::Up => (
            ((blizzard.pos.0 as i32 - 1 - t as i32).rem_euclid((height - 2) as i32) + 1) as usize,
            blizzard.pos.1
        ),
        Direction::Stay => panic!("Blizzard cannot stay still.")
    }
}

fn print_field(field: &Field, exp: &Expedition) {
    let blizzard_positions: Vec<(usize, usize)> = field.blizzards.iter()
        .map(|b| get_blizzard_position(b, exp.t, field.width, field.height)).collect();
    for j in 0..field.width {
        if exp.pos == (0, j) {
            print!("E");
        } else {
            print!("#");
        }
    }
    print!("\n");
    for i in 1..(field.height-1) {
        print!("#");
        for j in 1..(field.width-1) {
            if exp.pos == (i, j) {
                print!("E");
            } else {
                let mut matching_inds = Vec::new();
                for (ind, blizzard_pos) in blizzard_positions.iter().enumerate() {
                    if *blizzard_pos == (i, j) {
                        matching_inds.push(ind);
                    }
                }
                if matching_inds.len() == 1 {
                    match field.blizzards[matching_inds[0]].dir {
                        Direction::Right => print!(">"),
                        Direction::Left => print!("<"),
                        Direction::Down => print!("v"),
                        Direction::Up => print!("^"),
                        _ => ()
                    }
                } else if matching_inds.len() > 1 {
                    print!("{}", matching_inds.len());
                } else {
                    print!(".");
                }
            }
        }
        print!("#\n");
    }
    for j in 0..field.width {
        if exp.pos == (field.height-1, j) {
            print!("E");
        } else {
            print!("#");
        }
    }
    print!("\n");
}


#[derive(PartialEq, Eq)]
struct State {
    cost: usize,
    exp: Expedition
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost).then_with(|| self.exp.cmp(&other.exp))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn search_path(field: &Field, start_expedition: Expedition, target_pos: (usize, usize)) -> Option<Vec<Expedition>> {

    let mut queue = BinaryHeap::new();
    let mut visited: HashSet<Expedition> = HashSet::new();
    let mut parents: HashMap<Expedition, Expedition> = HashMap::new();
    queue.push(State {
        cost: start_expedition.t + start_expedition.pos.0.abs_diff(target_pos.0) + start_expedition.pos.1.abs_diff(target_pos.1),
        exp: start_expedition
    });
    visited.insert(start_expedition);

    while let Some(State {cost: _, exp} ) = queue.pop() {

        if exp.pos == target_pos {
            let mut path = Vec::new();
            let mut cur_exp = &exp;
            path.push(exp);
            while let Some(parent_exp) = parents.get(cur_exp) {
                path.push(*parent_exp);
                cur_exp = parent_exp;
            }
            return Some(path);
        }

        for dir in DIRECTIONS {
            if let Some(new_exp) = field.make_move(&exp, dir) {
                if !visited.contains(&new_exp) {
                    queue.push(State {
                        cost: new_exp.t + new_exp.pos.0.abs_diff(target_pos.0) + new_exp.pos.1.abs_diff(target_pos.1),
                        exp: new_exp });
                    visited.insert(new_exp);
                    parents.insert(new_exp, exp);
                }
            }
        }

    }

    None

}

fn main() {
    
    let input = fs::read_to_string("input.txt").unwrap();

    let mut width = 0;
    let mut height = 0;
    let mut blizzards = Vec::new();
    for (i, line) in input.lines().enumerate() {
        let mut j = 0;
        for c in line.chars() {
            match c {
                '>' => blizzards.push(Blizzard{pos: (i, j), dir: Direction::Right}),
                '<' => blizzards.push(Blizzard{pos: (i, j), dir: Direction::Left}),
                '^' => blizzards.push(Blizzard{pos: (i, j), dir: Direction::Up}),
                'v' => blizzards.push(Blizzard{pos: (i, j), dir: Direction::Down}),
                _ => ()
            }
            j += 1;
        }
        width = j;
        height += 1;
    }

    let field = Field{width, height, blizzards};

    let start_pos = (0, 1);
    let end_pos = (field.height - 1, field.width - 2);
    let expedition = Expedition {pos: start_pos, t: 0};
    print_field(&field, &expedition);

    let shortest_path = search_path(&field, expedition, end_pos).unwrap();
    println!("Shortest path length 1: {}", shortest_path.len() - 1);
    let shortest_path = search_path(&field, shortest_path[0], start_pos).unwrap();
    println!("Shortest path length 2: {}", shortest_path.len() - 1);
    let shortest_path = search_path(&field, shortest_path[0], end_pos).unwrap();
    println!("Shortest path length 3: {}", shortest_path.len() - 1);
    println!("Shortest path length (forth-back-forth): {}", shortest_path[0].t);

}
