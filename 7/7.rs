
use std::{fs, str, collections::HashMap};

fn get_dir_size(dir: String, dir_contents: &mut HashMap<String, i32>, dir_children: &HashMap<String, Vec<String>>) {
    let mut dir_size = match dir_contents.get(&dir) {
        Some(s) => s.clone(),
        None => 0
    };
    match dir_children.get(&dir) {
        Some(children) => {
            for child_dir in children {
                get_dir_size(child_dir.to_owned(), dir_contents, dir_children);
                dir_size += dir_contents.get(child_dir).unwrap().clone();
            }
        },
        None => ()
    }
    dir_contents.insert(dir, dir_size);
}

fn cat_dir(parent: String, cur_dir:String, new_dir: String) -> String {
    if new_dir == ".." {
        if parent == "" {panic!("cd .. when no parent");}
        parent
    } else {
        if new_dir == "/" {
            new_dir
        } else if cur_dir == "/" {
            format!("/{new_dir}")
        } else {
            format!("{cur_dir}/{new_dir}")
        }
    }
}

fn main() {

    let input = fs::read_to_string("input.txt").expect("Reading input");
    
    let mut cur_dir: String= "".to_owned();
    let mut dir_contents: HashMap<String, i32> = HashMap::new();
    let mut dir_children: HashMap<String, Vec<String>> = HashMap::new();
    let mut dir_parents: HashMap<String, String> = HashMap::new();
    for line in str::lines(&input) {
        if line.trim().len() == 0 {continue;}
        if line.chars().nth(0).unwrap() == '$' {
            match line {
                _ if line.contains("cd") => {
                    let new_dir = line.split_whitespace().collect::<Vec<&str>>()[2].to_owned();
                    let parent_dir = match dir_parents.get(&cur_dir) {
                        Some(dir) => dir.to_owned(),
                        None => "".to_owned()
                    };
                    cur_dir = cat_dir(parent_dir, cur_dir, new_dir);
                },
                _ if line.contains("ls") => (),
                _ => panic!("Unhandled line: {}", line)
            }
        } else {
            match line {
                _ if line.contains("dir") => {
                    let dir = cat_dir("".to_owned(), cur_dir.clone(), 
                        line.split_whitespace().collect::<Vec<&str>>()[1].to_owned());
                    dir_parents.insert(dir.clone(), cur_dir.clone());
                    match dir_children.get_mut(&cur_dir) {
                        Some(children) => children.push(dir),
                        None => {
                            dir_children.insert(cur_dir.clone(), Vec::from([dir]));
                            ()
                        }
                    }
                }
                _ => {
                    let file_size: i32 = line.split_whitespace().collect::<Vec<&str>>()[0].parse().unwrap();
                    match dir_contents.get(&cur_dir) {
                        Some(prev_size) => dir_contents.insert(cur_dir.clone(), prev_size+file_size),
                        None => dir_contents.insert(cur_dir.clone(), file_size)
                    };
                }
            }
        }
    }

    get_dir_size("/".to_owned(), &mut dir_contents, &dir_children);

    let mut small_sum = 0;
    for (_, size) in dir_contents.iter() {
        if *size <= 100000 {
            small_sum += size;
        }
    }
    println!("Sum of dirs with less than 10000: {}", small_sum);

    let used_space = dir_contents.get("/").unwrap();
    let free_space = 70_000_000 - used_space;
    let required_space = 30_000_000 - free_space;
    println!("Used space: {used_space}, free space {free_space}, required space: {required_space}");

    let mut smallest_big_enough = i32::MAX;
    for (_, size) in dir_contents.iter() {
        if *size >= required_space && *size < smallest_big_enough{
            smallest_big_enough = *size;
        }
    }
    println!("Size of smallest dir big enough to free enough space: {}", smallest_big_enough);

}
