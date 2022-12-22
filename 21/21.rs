
use std::{fs, collections::HashMap};

fn parse_lines<'a>(target: &'a str, lines: &HashMap<&str, &'a str>) -> i64 {
    let line = lines.get(target).unwrap();
    match line.parse::<i64>() {
        Ok(v) => v,
        Err(_) => {
            let line: Vec<&str> = line.split_whitespace().collect();
            let (target1, op, target2) = (line[0], line[1], line[2]);
            let val1 = parse_lines(target1, lines);
            let val2 = parse_lines(target2, lines);
            let num = match op {
                "+" => val1 + val2,
                "-" => val1 - val2,
                "*" => val1 * val2,
                "/" => val1 / val2,
                _ => panic!("Unhandled operator")
            };
            num
        }
    }
}

fn reduce_tree<'a>(start: &'a str, lines: &HashMap<&str, &'a str>, nums: &mut HashMap<&'a str, i64>) -> Option<i64> {
    let line = lines.get(start).unwrap();
    match line.parse::<i64>() {
        Ok(v) => {
            nums.insert(start, v);
            Some(v)
        },
        Err(_) => {
            let line: Vec<&str> = line.split_whitespace().collect();
            let (target1, op, target2) = (line[0], line[1], line[2]);
            if target1 == "humn" {
                reduce_tree(target2, lines, nums);
                None
            } else if target2 == "humn" {
                reduce_tree(target1, lines, nums);
                None
            } else {
                let val1 = reduce_tree(target1, lines, nums);
                let val2 = reduce_tree(target2, lines, nums);
                if val1.is_none() || val2.is_none() {
                    return None
                }
                let val1 = val1.unwrap();
                let val2 = val2.unwrap();
                let num = match op {
                    "+" => val1 + val2,
                    "-" => val1 - val2,
                    "*" => val1 * val2,
                    "/" => val1 / val2,
                    _ => panic!("Unhandled operator")
                };
                nums.insert(start, num);
                nums.remove(target1);
                nums.remove(target2);
                Some(num)
            }
        }
    }
}

fn parse_lines_inverse<'a>(target: &'a str, lines: &HashMap<&str, &'a str>, nums: &HashMap<&'a str, i64>, iter: u64) -> i64 {
    match nums.get(target) {
        Some(v) => *v,
        None => {
            let (name, line) = lines.iter().find(|(_, line)| line.contains(target)).unwrap();
            let line: Vec<&str> = line.split_whitespace().collect();
            let (target1, op, target2) = (line[0], line[1], line[2]);
            if *name == "root" {
                if target == target1 {
                    return *nums.get(target2).unwrap()
                } else {
                    return *nums.get(target1).unwrap()
                }
            }
            let val1 = parse_lines_inverse(name, lines, nums, iter + 1);
            if target1 == target {
                let val2 = parse_lines_inverse(target2, lines, nums, iter + 1);
                match op {
                    "+" => val1 - val2,
                    "-" => val1 + val2,
                    "*" => val1 / val2,
                    "/" => val1 * val2,
                    _ => panic!("Unhandled operator")
                }
            } else {
                let val2 = parse_lines_inverse(target1, lines, nums, iter + 1);
                match op {
                    "+" => val1 - val2,
                    "-" => val2 - val1,
                    "*" => val1 / val2,
                    "/" => val2 / val1,
                    _ => panic!("Unhandled operator")
                }
            }
        }
    }
}

fn main() {

    let input = fs::read_to_string("input.txt").unwrap();

    let monkey_lines: HashMap<&str, &str> = input.lines()
        .map(|line| line.split(":").take(2).collect::<Vec<&str>>())
        .map(|v| (v[0].trim(), v[1].trim()))
        .collect();
    
    let root_num = parse_lines("root", &monkey_lines);
    println!("root number: {root_num}");

    let mut monkey_lines = monkey_lines.clone();
    monkey_lines.remove("humn");
    let mut monkey_nums = HashMap::new();
    reduce_tree("root", &monkey_lines, &mut monkey_nums);
    let humn_num = parse_lines_inverse("humn", &monkey_lines, &monkey_nums, 0);
    println!("humn number: {humn_num}");


}
